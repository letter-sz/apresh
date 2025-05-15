mod refund_log;
mod transfer;
mod utils;

#[cfg(not(feature = "no-mocks"))]
mod mock_data;

use std::cell::RefCell;

use apresh_engine::{
    operations::{
        AddMessageOp, BuyShipmentOp, CancelShipmentOp, CreateShipmentOp, FinalizeShipmentOp,
        ReadMessageOp, RegisterActorOp, StateOp,
    },
    state::CanisterState,
};
use apresh_qr_code::{generate, QrCodeOptions};
use apresh_store::Record;
use apresh_types::{
    ActorId, Carrier, CarrierKey, Channel, ChannelKey, PrintableShipment, Shipment, ShipmentInfo,
    ShipmentKey, ShipmentStatus, ShipperKey,
};
use candid::Principal;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use refund_log::RefundLog;
pub use transfer::consts;
use transfer::{transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams};
use utils::{assert_admin, assert_whitelisted, balances_of, callers_balances, memo};

thread_local! {
    pub static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
    pub static TRANSFER_FEE: RefCell<u64> = const { RefCell::new(10_000) };
    pub static ADMIN: RefCell<Principal> = const{ RefCell::new(Principal::anonymous()) };
    pub static WHITELIST: RefCell<Vec<Principal>> = RefCell::default();
    pub static REFUND_LOG: RefCell<RefundLog> = RefCell::new(RefundLog::default());
    pub static CANISTER_LOCKED: RefCell<bool> = const { RefCell::new(false) };
}

#[init]
fn init() {
    ADMIN.with_borrow_mut(|caller| *caller = ic_cdk::caller());

    #[cfg(not(feature = "no-mocks"))]
    mock_data::mock_shipments();
}

#[query]
fn balance() -> (u64, u64) {
    let balances = callers_balances();
    (balances.balance(), balances.locked())
}

#[update]
async fn deposit(amount: u64) -> Result<(), String> {
    assert_whitelisted();

    if let Err(e) = transfer_in(TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: memo("DEPOSIT", amount),
        },
        from: ic_cdk::caller().into(),
    })
    .await
    {
        ic_cdk::trap(&e.to_string());
    }

    let mut balances = callers_balances();
    balances.deposit(amount).map_err(|e| e.to_string())?;
    balances.commit();

    Ok(())
}

#[update]
async fn withdraw(amount: u64) -> Result<(), String> {
    assert_whitelisted();

    // If the amount is smaller than the fee, nothing happens
    let fee = get_transfer_fee();
    if amount <= fee {
        ic_cdk::trap("Insufficient balance");
    }

    let mut balances = callers_balances();
    balances.withdraw(amount).map_err(|e| e.to_string())?;

    let transfer_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: memo("WITHDRAW", amount),
        },
        to: ic_cdk::caller().into(),
    };

    // If transfer fails, return the error
    if let Err(e) = transfer_out(transfer_args, get_transfer_fee()).await {
        if let Err(e_log) = REFUND_LOG.with_borrow_mut(|log| {
            log.append(amount, ic_cdk::caller(), format!("ERROR WITHDRAW: {}", e))
        }) {
            ic_cdk::trap(&format!(
                "Error while withdrawing and appending to log {} {}",
                e, e_log
            ));
        }

        ic_cdk::trap(&format!("Error while withdrawing, {}", e));
    }

    balances.commit();

    Ok(())
}

#[query]
fn is_mainnet() -> bool {
    cfg!(feature = "mainnet")
}

#[update(name = "addWhitelisted")]
fn add_whitelisted(principal: Principal) {
    assert_admin();

    WHITELIST.with_borrow_mut(|whitelist| whitelist.push(principal));
}

#[update(name = "setTransferFee")]
fn set_transfer_fee(fee: u64) {
    assert_admin();

    TRANSFER_FEE.set(fee);
}

#[query(name = "getTransferFee")]
fn get_transfer_fee() -> u64 {
    TRANSFER_FEE.with_borrow(|fee| *fee)
}

#[update]
async fn add_message(message: Vec<u8>, shipment_id: u64) -> Result<(), String> {
    assert_whitelisted();

    let caller = ActorId(ic_cdk::caller());
    let mut shipment = ShipmentKey(shipment_id).get().unwrap();

    STATE
        .with_borrow_mut(|state| AddMessageOp::new(&mut shipment, message, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[query]
async fn read_channel(shipment_id: u64) -> Result<Channel, String> {
    let caller = ActorId(ic_cdk::caller());
    let shipment = ShipmentKey(shipment_id).get().unwrap();

    STATE
        .with_borrow(|state| ReadMessageOp::new(&shipment, caller).read(state))
        .map_err(|e| e.to_string())
}

#[update(name = "finalizeShipment")]
async fn finalize_shipment(shipment_id: u64, secret_key: Option<String>) -> Result<(), String> {
    let caller = ActorId(ic_cdk::caller());

    let mut shipment = ShipmentKey(shipment_id).get().unwrap();

    let result = STATE.with_borrow_mut(|state| {
        FinalizeShipmentOp::new(&mut shipment, secret_key, caller).apply(state)
    });

    let result = match result {
        Ok(result) => result,
        Err(e) => {
            shipment.revert();
            return Err(e.to_string());
        }
    };

    let mut shipper_balances = balances_of(shipment.shipper_id().into());
    let mut carrier_balances = balances_of(result.carrier_id().0);

    let transfer_result = carrier_balances.transfer_from(&mut shipper_balances, result.price());
    let unlock_result = carrier_balances.unlock(result.value());

    match (transfer_result, unlock_result) {
        (Ok(_), Ok(_)) => {
            shipment.commit();
            shipper_balances.commit();
            carrier_balances.commit();
            Ok(())
        }
        (err1, err2) => {
            shipment.revert();
            shipper_balances.revert();
            carrier_balances.revert();
            err1.map_err(|e| e.to_string())?;
            err2.map_err(|e| e.to_string())
        }
    }
}

use entrypoint::entrypoint;

#[entrypoint]
#[update(name = "buyShipment")]
async fn buy_shipment(
    carrier_name: Option<String>,
    #[key] shipment: Shipment,
    channel_key: ChannelKey,
) -> Result<(), String> {
    assert_whitelisted();
    let caller = ActorId(ic_cdk::caller());

    let mut carrier = CarrierKey(caller).get().unwrap();

    let result = STATE.with_borrow_mut(|state| {
        // Register carrier if carrier name is provided
        if let Some(carrier_name) = carrier_name {
            let carrier = Carrier::new(caller, carrier_name.as_str());

            RegisterActorOp::AddCarrier {
                id: carrier.id(),
                name: carrier_name,
            }
            .apply(state)
            .map_err(|e| e.to_string())
            .unwrap();
        }

        BuyShipmentOp::new(&mut carrier, shipment, channel_key).apply(state)
    });

    let shipment_value = match result {
        Ok(shipment_value) => shipment_value,
        Err(e) => {
            carrier.revert();
            return Err(e.to_string());
        }
    };

    let mut balances = callers_balances();
    match balances.lock(shipment_value) {
        Ok(_) => {
            carrier.commit();
            // shipment.commit();
            balances.commit();
            Ok(())
        }
        Err(e) => {
            carrier.revert();
            // shipment.revert();
            balances.revert();
            Err(e.to_string())
        }
    }
}

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: Option<String>,
    shipment_name: String,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_info: ShipmentInfo,
) -> Result<u64, String> {
    assert_whitelisted();
    let caller = ShipperKey(ActorId(ic_cdk::caller()));
    let price = shipment_info.price();

    let mut shipper = STATE
        .with_borrow_mut(|state| {
            // First register the shipper if needed
            let shipper = match (caller.get(), customer_name) {
                (Some(shipper), _) => shipper,
                (None, Some(customer_name)) => {
                    RegisterActorOp::AddShipper {
                        id: caller.0,
                        name: customer_name.clone(),
                    }
                    .apply(state)
                    .map_err(|e| e.to_string())?;
                    caller.get().ok_or("Shipper could not be registered")?
                }
                (None, None) => {
                    ic_cdk::trap("Shipper does not exist and no name was provided");
                }
            };

            Result::<_, String>::Ok(shipper)
        })
        .map_err(|e| e.to_string())?;

    let result = STATE.with_borrow_mut(|state| {
        let create_op = CreateShipmentOp::new(
            &mut shipper,
            hashed_secret,
            channel_key,
            &shipment_name,
            &shipment_info,
            ic_cdk::api::time(),
        );

        let shipment = match create_op.apply(state) {
            Ok(shipment) => shipment,
            Err(e) => ic_cdk::trap(&e.to_string()),
        };

        Result::<_, String>::Ok(shipment)
    });

    let shipment = match result {
        Ok(shipment) => shipment,
        Err(e) => {
            shipper.revert();
            return Err(e);
        }
    };

    let mut balances = callers_balances();
    match balances.lock(price) {
        Ok(_) => {
            balances.commit();
            shipper.commit();
            let shipment_id = shipment.id();
            shipment.set();
            Ok(shipment_id)
        }
        Err(e) => {
            shipper.revert();
            balances.revert();
            Err(e.to_string())
        }
    }
}

#[update]
fn cancel_shipment(shipment_id: u64) -> Result<(), String> {
    assert_whitelisted();
    let caller = ShipperKey(ActorId(ic_cdk::caller()));

    let mut shipment = ShipmentKey(shipment_id).get().unwrap();
    let shipper = caller.get().unwrap();

    STATE
        .with_borrow_mut(|state| CancelShipmentOp::new(&shipper, &mut shipment).apply(state))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<PrintableShipment> {
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
        .map(PrintableShipment::from)
        .collect()
}

#[query]
fn shipper_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
        .filter(|shipment| shipment.shipper_id() == customer_id)
        .filter(|shipment| !shipment.status().is_finished())
        .map(PrintableShipment::from)
        .collect()
}

#[query]
fn carrier_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());

    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| shipment.carrier_id() == Some(customer_id))
        .filter(|shipment| !shipment.status().is_finished())
        .map(PrintableShipment::from)
        .collect()
}

#[query]
fn roles() -> (bool, bool) {
    let caller = ic_cdk::caller();

    let carrier = (CarrierKey(caller.into()).get()).is_some();
    let shipper: bool = (ShipperKey(caller.into()).get()).is_some();

    (carrier, shipper)
}

#[query]
fn shipments() -> Vec<PrintableShipment> {
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .map(PrintableShipment::from)
        .collect()
}

#[query]
fn shipment(shipment_id: u64) -> Option<PrintableShipment> {
    ShipmentKey(shipment_id)
        .get()
        .map(|s| PrintableShipment::from(&*s))
}

#[query(name = "generateQr")]
async fn generate_qr(link: String, size: usize) -> Result<Vec<u8>, String> {
    generate(QrCodeOptions {
        gradient: false,
        link,
        size,
        transparent: false,
    })
    .map_err(|e| e.to_string())
}

#[update(name = "lockCanister")]
fn lock_canister() {
    assert_admin();
    CANISTER_LOCKED.with_borrow_mut(|locked| *locked = true);
}

#[update(name = "unlockCanister")]
fn unlock_canister() {
    assert_admin();
    CANISTER_LOCKED.with_borrow_mut(|locked| *locked = false);
}

#[ic_cdk::post_upgrade]
pub fn post_upgrade() {
    // lock until the state is explicitly unlocked
    CANISTER_LOCKED.with_borrow_mut(|locked| {
        *locked = true;
    });
}

ic_cdk::export_candid!();
