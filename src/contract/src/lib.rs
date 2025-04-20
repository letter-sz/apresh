mod refund_log;
mod transfer;
mod utils;

use apresh_types::{
    ActorId, Carrier, Channel, ChannelKey, PrintableShipment, ShipmentInfo, ShipmentStatus,
};
pub use transfer::consts;

#[cfg(not(feature = "no-mocks"))]
mod mock_data;

use std::cell::RefCell;

use apresh_engine::{
    operations::{
        AddMessageOp, BuyShipmentOp, CancelShipmentOp, CreateShipmentOp, FinalizeShipmentOp,
        ReadMessageOp, RegisterActorOp, StateOp, ValidatedStateOp,
    },
    state::{CanisterActors, CanisterShipments, CanisterState},
};
use apresh_qr_code::{generate, QrCodeOptions};
use candid::Principal;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use refund_log::RefundLog;
use transfer::{transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams};
use utils::{assert_admin, assert_whitelisted, memo};

thread_local! {
    pub static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
    pub static TRANSFER_FEE: RefCell<u64> = const { RefCell::new(10_000) };
    pub static DEAD_TOKENS: RefCell<u64> = RefCell::default(); // Tokens, where transfer amount is less than the fee needed to transfer it.
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

    STATE
        .with_borrow_mut(|state| AddMessageOp::new(shipment_id, message, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[query]
async fn read_channel(shipment_id: u64) -> Result<Channel, String> {
    let caller = ActorId(ic_cdk::caller());

    STATE
        .with_borrow(|state| ReadMessageOp::new(shipment_id, caller).read(state))
        .map_err(|e| e.to_string())
}

#[update(name = "finalizeShipment")]
async fn finalize_shipment(shipment_id: u64, secret_key: Option<String>) -> Result<(), String> {
    let caller = ActorId(ic_cdk::caller());

    let op = FinalizeShipmentOp::new(shipment_id, secret_key, caller);

    let finalize_shipment_result = STATE
        .with_borrow_mut(|state| op.validate(state))
        .map_err(|e| e.to_string())?;

    let fee = get_transfer_fee();
    let amount = finalize_shipment_result.value() + finalize_shipment_result.price();

    // If the amount is smaller than the fee, skip the transfer and add the amount to the dead tokens
    if amount <= fee {
        DEAD_TOKENS.with_borrow_mut(|dead_tokens| *dead_tokens += amount);

        STATE
            .with_borrow_mut(|state| op.validate_and_apply(state))
            .map_err(|e| e.to_string())?;

        ic_cdk::print(format!("Shipment finalized, without transfers: {:?}", shipment_id).as_str());
        return Ok(());
    }

    let transfer_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: memo("SETTLE", shipment_id),
        },
        to: (finalize_shipment_result.carrier_id().0).into(),
    };

    // If transfer fails, return the error
    if let Err(e) = transfer_out(transfer_args, get_transfer_fee()).await {
        return Err(e.to_string());
    }

    // Modify the state
    let res = STATE
        .with_borrow_mut(|state| op.validate_and_apply(state))
        .map_err(|e| e.to_string());

    // At this stage there should be way to return error, but refund there is one
    if let Err(_e) = res {
        let refund_res = transfer_out(
            TransferOutParams {
                params: TransferParams {
                    amount: NumTokens::from(amount),
                    memo: memo("REFUND", shipment_id),
                },
                to: caller.0.into(),
            },
            get_transfer_fee(),
        )
        .await;

        if let Err(refund_error) = refund_res {
            ic_cdk::print(format!("Error refunding: {:?}", refund_error).as_str());
            REFUND_LOG.with_borrow_mut(|log| {
                log.append(
                    amount,
                    caller.0,
                    format!(
                        "ERROR FINALIZE SHIPMENT: {}, REFUND ERROR: {}",
                        shipment_id, refund_error
                    ),
                )
            })?;
        } else {
            REFUND_LOG.with_borrow_mut(|log| {
                log.append(
                    amount,
                    caller.0,
                    format!("FINALIZE SHIPMENT REFUNDED: {} DONE", shipment_id),
                )
            })?;
        }
    }

    ic_cdk::print(format!("Shipment finalized: {:?}", shipment_id).as_str());
    Ok(())
}

#[update(name = "buyShipment")]
async fn buy_shipment(
    carrier_name: Option<String>,
    shipment_id: u64,
    channel_key: ChannelKey,
) -> Result<(), String> {
    assert_whitelisted();
    let caller = ActorId(ic_cdk::caller());

    let op = BuyShipmentOp::new(caller, shipment_id, channel_key);

    let shipment_value = STATE
        .with_borrow_mut(|state| {
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

            // Validate the operation
            op.validate(state)
        })
        .unwrap();

    let transfer_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(shipment_value),
            memo: memo("BUY", shipment_id),
        },
        from: caller.0.into(),
    };

    // If transfer fails, return the error
    if let Err(e) = transfer_in(transfer_args).await {
        return Err(e.to_string());
    }

    // Modify the state
    let res = STATE.with_borrow_mut(|state| op.validate_and_apply(state));

    // If the operation fails, refund the shipment value
    if let Err(e) = res {
        let refund_res = transfer_out(
            TransferOutParams {
                params: TransferParams {
                    amount: NumTokens::from(shipment_value),
                    memo: memo("REFUND", shipment_id),
                },
                to: caller.0.into(),
            },
            get_transfer_fee(),
        )
        .await;

        if let Err(refund_error) = refund_res {
            ic_cdk::print(format!("Error refunding: {:?}", refund_error).as_str());
            REFUND_LOG.with_borrow_mut(|log| {
                log.append(
                    shipment_value,
                    caller.0,
                    format!(
                        "ERROR BUY SHIPMENT: {}, REFUND ERROR: {}",
                        shipment_id, refund_error
                    ),
                )
            })?;
        } else {
            REFUND_LOG.with_borrow_mut(|log| {
                log.append(
                    shipment_value,
                    caller.0,
                    format!("BUY SHIPMENT REFUNDED: {} DONE", shipment_id),
                )
            })?;
        }
        return Err(e.to_string());
    }

    ic_cdk::print(format!("Shipment bought: {:?}", shipment_id).as_str());
    Ok(())
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

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: Option<String>,
    shipment_name: String,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_info: ShipmentInfo,
) -> Result<u64, String> {
    assert_whitelisted();
    let caller = ActorId(ic_cdk::caller());
    let price = shipment_info.price();
    let created_at = ic_cdk::api::time();

    // First register the shipper if needed
    if let Some(customer_name) = &customer_name {
        STATE
            .with_borrow_mut(|state| {
                RegisterActorOp::AddShipper {
                    id: caller,
                    name: customer_name.clone(),
                }
                .apply(state)
            })
            .map_err(|e| e.to_string())?;
    }

    // Validate the shipment creation operation
    let create_op = CreateShipmentOp::new(
        caller,
        hashed_secret,
        channel_key,
        &shipment_name,
        &shipment_info,
        created_at,
    );

    // Validate before doing any transfers
    let expected_shipment_id = STATE
        .with_borrow(|state| create_op.validate(state))
        .map_err(|e| e.to_string())?;

    // Do the transfer
    let transfer_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(price),
            memo: memo("CREATE", expected_shipment_id),
        },
        from: caller.0.into(),
    };

    // If transfer fails, return the error
    if let Err(e) = transfer_in(transfer_args).await {
        return Err(e.to_string());
    }

    // Modify the state
    let shipment_id = STATE.with_borrow_mut(|state| create_op.validate_and_apply(state));

    // If the operation fails, refund the shipment value
    let shipment_id = match shipment_id {
        Ok(id) => id,
        Err(e) => {
            let refund_res = transfer_out(
                TransferOutParams {
                    params: TransferParams {
                        amount: NumTokens::from(price),
                        memo: memo("REFUND", expected_shipment_id),
                    },
                    to: caller.0.into(),
                },
                get_transfer_fee(),
            )
            .await;

            if let Err(refund_error) = refund_res {
                ic_cdk::print(format!("Error refunding: {:?}", refund_error).as_str());

                // TODO: handle error, there shouldn't be any error here, if it happens we're screwed
                REFUND_LOG.with_borrow_mut(|log| {
                    log.append(
                        price,
                        caller.0,
                        format!(
                            "ERROR CREATE SHIPMENT: {}, REFUND ERROR: {}",
                            expected_shipment_id, refund_error
                        ),
                    )
                })?;
            } else {
                REFUND_LOG.with_borrow_mut(|log| {
                    log.append(
                        price,
                        caller.0,
                        format!("CREATE SHIPMENT REFUNDED: {} DONE", expected_shipment_id),
                    )
                })?;
            }

            return Err(e.to_string());
        }
    };

    Ok(shipment_id)
}

#[update]
fn cancel_shipment(shipment_id: u64) -> Result<(), String> {
    assert_whitelisted();
    let caller = ActorId(ic_cdk::caller());

    STATE
        .with_borrow_mut(|state| CancelShipmentOp::new(caller, shipment_id).apply(state))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<PrintableShipment> {
    STATE.with_borrow(|state| {
        state
            .shipments()
            .iter()
            .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
            .map(PrintableShipment::from)
            .collect()
    })
}

#[query]
fn shipper_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());

    STATE.with_borrow(|state| {
        state
            .shipments()
            .iter()
            .filter(|shipment| shipment.shipper_id() == customer_id)
            .filter(|shipment| !shipment.status().is_finished())
            .map(PrintableShipment::from)
            .collect()
    })
}

#[query]
fn carrier_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());

    STATE.with_borrow(|state| {
        state
            .shipments()
            .iter()
            .filter(|shipment| shipment.carrier_id() == Some(customer_id))
            .filter(|shipment| !shipment.status().is_finished())
            .map(PrintableShipment::from)
            .collect()
    })
}

#[query]
fn roles() -> (bool, bool) {
    let caller = ic_cdk::caller();

    let carrier = STATE.with_borrow(|state| state.carrier(&caller.into()).is_some());
    let shipper = STATE.with_borrow(|state| state.shipper(&caller.into()).is_some());

    (carrier, shipper)
}

#[query]
fn shipments() -> Vec<PrintableShipment> {
    STATE.with_borrow(|state| {
        state
            .shipments()
            .iter()
            .map(PrintableShipment::from)
            .collect()
    })
}

#[query]
fn shipment(shipment_id: u64) -> Option<PrintableShipment> {
    STATE.with_borrow(|state| {
        state
            .shipment(shipment_id)
            .as_ref()
            .map(PrintableShipment::from)
    })
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
