mod actors;
mod models;
mod operations;
mod qr;
mod state;
mod transfer;
mod utils;
mod vetkd;

use actors::{carrier::Carrier, shipper::Shipper};
use candid::Principal;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    qrcode::QrCodeOptions,
    shipment::{Shipment, ShipmentInfo, ShipmentLocation, ShipmentStatus, SizeCategory},
};
use operations::{
    AddMessageOp, BuyShipmentOp, CreateShipmentOp, FinalizeShipmentOp, ReadMessageOp, StateOp,
};
use state::STATE;
use transfer::{transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams};
use utils::block_anonymous;

pub use vetkd::{encrypted_ibe_decryption_key_for_caller, ibe_encryption_key};

#[init]
fn init() {
    ic_cdk::print("Initializing the shipment service");

    // Define a set of realistic coordinates for shipment locations
    let locations = [
        ("A", 40.7128, -74.0060),  // New York, USA
        ("B", 34.0522, -118.2437), // Los Angeles, USA
        ("C", 51.5074, -0.1278),   // London, UK
        ("D", 48.8566, 2.3522),    // Paris, France
        ("E", 35.6895, 139.6917),  // Tokyo, Japan
        ("F", -33.8688, 151.2093), // Sydney, Australia
    ];

    let names: [&str; 10] = [
        "John Doe",
        "Jane Doe",
        "Alice Smith",
        "Bob Smith",
        "Charlie Brown",
        "Daisy Brown",
        "Eve Green",
        "Frank Green",
        "Grace Black",
        "Harry Black",
    ];

    let default_principal =
        Principal::from_text("ryssj-xcbz7-gbw4s-p7fio-lolnx-5nr7a-yxufe-cvpfg-6iujw-2ypsz-rqe")
            .expect("Failed to create principal");

    let shippers = names
        .iter()
        .map(|name| Shipper::new(default_principal, name))
        .collect::<Vec<_>>();

    let packages_names = [
        "Package 1",
        "Package 2",
        "Package 3",
        "Package 4",
        "Package 5",
        "Package 6",
        "Package 7",
        "Package 8",
        "Package 9",
        "Package 10",
    ];

    for (i, (shipper, package_name)) in shippers
        .into_iter()
        .zip(packages_names.into_iter())
        .enumerate()
    {
        let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
        let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

        let create_shipment_op = CreateShipmentOp::new(
            shipper,
            "hashed_secret",
            package_name,
            ShipmentInfo::new(
                100u64 + i as u64,
                10u64 + i as u64,
                ShipmentLocation::new(origin_label.to_string(), *origin_lat, *origin_lng),
                ShipmentLocation::new(dest_label.to_string(), *dest_lat, *dest_lng),
                SizeCategory::Envelope,
            ),
            ic_cdk::api::time(),
        );

        let shipment_id = STATE
            .with_borrow_mut(|state| create_shipment_op.apply(state))
            .map_err(|e| e.to_string())
            .expect("Failed to create shipment");

        ic_cdk::print(format!(
            "Shipment created: {:?}, shipment_id: {}",
            create_shipment_op, shipment_id
        ));
    }
}

#[update(name = "addEncryptedMessage")]
async fn add_encrypted_message(message: String, shipment_id: u64) -> Result<(), String> {
    let caller: Principal = ic_cdk::caller();

    STATE
        .with_borrow_mut(|state| AddMessageOp::new(shipment_id, &message, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[update(name = "readEncryptedMessage")]
async fn read_encrypted_message(shipment_id: u64) -> Result<Option<String>, String> {
    let caller: Principal = ic_cdk::caller();

    STATE
        .with_borrow_mut(|state| ReadMessageOp::new(shipment_id, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[update(name = "finalizeShipment")]
async fn finalize_shipment(shipment_id: u64, secret_key: Option<String>) -> Result<(), String> {
    let caller = ic_cdk::caller();

    let finalize_shipment_op = FinalizeShipmentOp::new(shipment_id, secret_key.as_deref(), caller);

    let finalize_shipment_result = STATE
        .with_borrow_mut(|state| finalize_shipment_op.apply(state))
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_out_carrier_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(
                finalize_shipment_result.value() + finalize_shipment_result.price(),
            ),
            memo: None,
        },
        to: (*finalize_shipment_result.carrier_id()).into(),
    };

    let transfer_out_carrier_result = transfer_out(transfer_out_carrier_args)
        .await
        .map_err(|e| e.to_string());

    // if let Err(e) = transfer::transfer_out(transfer_out_carrier_args).await {
    //     ic_cdk::trap(&e.to_string())
    // }

    ic_cdk::print(format!("Shipment finalized: {:?}", shipment_id).as_str());

    Ok(())
}

#[update(name = "buyShipment")]
async fn buy_shipment(carrier_name: String, shipment_id: u64) -> Result<(), String> {
    block_anonymous()?;

    let carrier_id = ic_cdk::caller();
    let carrier = Carrier::new(carrier_id, carrier_name.as_str());
    let buy_shipment_op = BuyShipmentOp::new(carrier, shipment_id);

    let shipment_cost = STATE
        .with_borrow_mut(|state| buy_shipment_op.apply(state))
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_in_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(shipment_cost),
            memo: None,
        },
        from: carrier_id.into(),
    };

    let transfer_in_result = transfer_in(transfer_in_args)
        .await
        .map_err(|e| e.to_string());

    ic_cdk::print(format!("Shipment bought: {:?}", shipment_id).as_str());

    Ok(())
}

#[query(name = "generateQr")]
async fn generate_qr(link: String, size: usize) -> Result<Vec<u8>, String> {
    qr::generate(QrCodeOptions {
        gradient: false,
        link,
        size,
        transparent: false,
    })
    .map_err(|e| e.to_string())
}

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: String,
    shipment_name: String,
    hashed_secret: String,
    qr_options: QrCodeOptions,
    shipment_info: ShipmentInfo,
) -> Result<(Vec<u8>, u64), String> {
    ic_cdk::print("Creating a shipment");

    let customer_id = ic_cdk::caller();
    let amount = NumTokens::from(shipment_info.price());
    let shipper = Shipper::new(customer_id, customer_name.as_str());

    let transfer_in_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: None,
        },
        from: shipper.id().into(),
    };

    transfer_in(transfer_in_args)
        .await
        .map_err(|e| e.to_string())?;

    let created_at = ic_cdk::api::time();

    let create_shipment_op = CreateShipmentOp::new(
        shipper,
        &hashed_secret,
        &shipment_name,
        shipment_info,
        created_at,
    );

    let shipment_id = STATE
        .with_borrow_mut(|state| create_shipment_op.apply(state))
        .map_err(|e| e.to_string())?;

    let qr_code = qr::generate(qr_options).unwrap_or_else(|err| ic_cdk::trap(&err.to_string()));

    ic_cdk::print(format!("Shipment created: {:?}", shipment_id).as_str());
    Ok((qr_code, shipment_id))
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .filter(|shipment| *shipment.status() == ShipmentStatus::Created)
            .map(Shipment::from)
            .collect()
    })
}

#[query(name = "listUserShipments")]
fn get_user_shipments() -> (Vec<Shipment>, Vec<Shipment>) {
    let customer_id = ic_cdk::caller();

    let shippers = STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .filter(|shipment| shipment.shipper_id() == customer_id)
            .map(Shipment::from)
            .collect()
    });

    let carriers = STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .filter(|shipment| shipment.carrier_id() == Some(customer_id))
            .map(Shipment::from)
            .collect()
    });

    (shippers, carriers)
}

#[query]
fn roles() -> (bool, bool) {
    let caller = ic_cdk::caller();

    let carrier = STATE.with_borrow(|state| state.carriers.get(&caller).is_some());
    let shipper = STATE.with_borrow(|state| state.shippers.get(&caller).is_some());

    (carrier, shipper)
}

#[query]
fn shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .map(|shipment| Shipment::from(shipment))
            .collect()
    })
}

ic_cdk::export_candid!();
