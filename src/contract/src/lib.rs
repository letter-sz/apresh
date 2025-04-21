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
use apresh_store::{Record, BALANCES};
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
use utils::{assert_admin, assert_whitelisted, memo};

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
    BALANCES.with_borrow(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));
        (balance, locked)
    })
}

#[update]
async fn deposit(amount: u64) {
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

    BALANCES.with_borrow_mut(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));
        balances.insert(caller_bytes, (balance + amount, locked));
    });
}

#[update]
async fn withdraw(amount: u64) {
    assert_whitelisted();

    // If the amount is smaller than the fee, nothing happens
    let fee = get_transfer_fee();
    if amount <= fee {
        ic_cdk::trap("Insufficient balance");
    }

    BALANCES.with_borrow_mut(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));

        if balance < amount {
            ic_cdk::trap("Insufficient balance");
        }

        balances.insert(caller_bytes, (balance - amount, locked));
    });

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

    let finalize_shipment_result = STATE
        .with_borrow_mut(|state| {
            FinalizeShipmentOp::new(&mut shipment, secret_key, caller).apply(state)
        })
        .map_err(|e| e.to_string())?;

    let amount = finalize_shipment_result.value() + finalize_shipment_result.price();

    BALANCES.with_borrow_mut(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));
        balances.insert(caller_bytes, (balance + amount, locked));
    });

    Ok(())
}

#[update(name = "buyShipment")]
async fn buy_shipment(
    carrier_name: Option<String>,
    shipment_id: u64,
    channel_key: ChannelKey,
) -> Result<(), String> {
    assert_whitelisted();
    let caller = CarrierKey(ActorId(ic_cdk::caller()));

    let mut shipment = ShipmentKey(shipment_id).get().unwrap();

    let shipment_value = STATE
        .with_borrow_mut(|state| {
            // Register carrier if carrier name is provided
            if let Some(carrier_name) = carrier_name {
                let carrier = Carrier::new(caller.0, carrier_name.as_str());

                RegisterActorOp::AddCarrier {
                    id: carrier.id(),
                    name: carrier_name,
                }
                .apply(state)
                .map_err(|e| e.to_string())
                .unwrap();
            }

            let mut carrier = <Carrier as Record>::get_guard(caller).unwrap();
            BuyShipmentOp::new(&mut carrier, &mut shipment, channel_key).apply(state)
        })
        .map_err(|e| e.to_string())?;

    BALANCES.with_borrow_mut(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));

        if balance < shipment_value {
            ic_cdk::trap("Insufficient balance");
        }

        balances.insert(caller_bytes, (balance - shipment_value, locked));
    });

    Ok(())
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

    let shipment_id = STATE
        .with_borrow_mut(|state| {
            // First register the shipper if needed
            if let Some(customer_name) = &customer_name {
                let register_op = RegisterActorOp::AddShipper {
                    id: caller.0,
                    name: customer_name.clone(),
                };
                if let Err(e) = register_op.apply(state) {
                    return Err(e.to_string());
                }
            }

            let mut shipper = caller.get().unwrap();
            let create_op = CreateShipmentOp::new(
                &mut shipper,
                hashed_secret,
                channel_key,
                &shipment_name,
                &shipment_info,
                ic_cdk::api::time(),
            );

            let shipment_id = match create_op.apply(state) {
                Ok(shipment_id) => shipment_id,
                Err(e) => ic_cdk::trap(&e.to_string()),
            };

            Ok(shipment_id)
        })
        .map_err(|e| e.to_string())?;

    BALANCES.with_borrow_mut(|balances| {
        let caller_bytes = ic_cdk::caller().as_slice().to_vec();
        let (balance, locked) = balances.get(&caller_bytes).unwrap_or((0, 0));

        if balance < price {
            ic_cdk::trap("Insufficient balance");
        }

        balances.insert(caller_bytes, (balance - price, locked));
    });

    Ok(shipment_id)
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
    let shipper = (ShipperKey(caller.into()).get()).is_some();

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
