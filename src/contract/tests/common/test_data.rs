use candid::{decode_one, encode_one, Encode, Principal};
use contract::consts::THIS_CANISTER_ID;
use engine::models::shipment::{PrintableShipment, ShipmentInfo, ShipmentLocation, SizeCategory};
use engine::utils::hash_secret;
use pocket_ic::{PocketIc, WasmResult};
use rstest::{fixture, rstest};

use crate::TEST_PRINCIPAL;

use super::{pocket::*, update_canister};

pub struct TestEnvironmentWithShipment {
    pub pic: PocketIc,
    pub shipment_id: u64,
}

#[fixture]
pub fn test_shipment(
    pic: PocketIc,
    #[default("Test Package")] name: String,
) -> TestEnvironmentWithShipment {
    let customer_name = Some("Test Customer".to_string());
    let shipment_name = name;
    let secret = b"test_secret";
    let hashed_secret = hash_secret(secret);
    let channel_key = vec![0u8; 32];

    let shipment_info = ShipmentInfo::new(
        100, // price
        10,  // value
        ShipmentLocation::new("Origin".to_string(), 40.7128, -74.0060),
        ShipmentLocation::new("Destination".to_string(), 34.0522, -118.2437),
        SizeCategory::Envelope,
    );

    let result = update_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "createShipment",
        Encode!(
            &customer_name,
            &shipment_name,
            &hashed_secret,
            &channel_key,
            &shipment_info
        )
        .unwrap(),
        TEST_PRINCIPAL,
    );

    let res: Result<u64, String> = decode_one(&result).unwrap();
    TestEnvironmentWithShipment {
        pic,
        shipment_id: res.unwrap(),
    }
}
