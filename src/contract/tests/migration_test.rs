use candid::{decode_one, encode_one, Encode, Principal};
use contract::consts::THIS_CANISTER_ID;
use engine::models::shipment::{PrintableShipment, ShipmentInfo, ShipmentLocation, SizeCategory};
use engine::utils::hash_secret;
use pocket_ic::{PocketIc, WasmResult};
use rstest::{fixture, rstest};

use common::*;

mod common;

fn migrate_sequence(pic: &PocketIc, principal: Principal) {
    // Verify shipment was created
    update_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "lockCanister",
        encode_one(()).unwrap(),
        TEST_PRINCIPAL,
    );

    update_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "migrateShippers",
        encode_one(()).unwrap(),
        TEST_PRINCIPAL,
    );

    update_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "migrateCarriers",
        encode_one(()).unwrap(),
        TEST_PRINCIPAL,
    );

    update_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "migrateShipments",
        encode_one(()).unwrap(),
        TEST_PRINCIPAL,
    );
}

#[rstest]
fn test_create_shipment(test_shipment: TestEnvironmentWithShipment) {
    let TestEnvironmentWithShipment { pic, shipment_id } = test_shipment;

    migrate_sequence(&pic, TEST_PRINCIPAL);

    pic.upgrade_canister(
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        CONTRACT_WASM.to_vec(),
        vec![],
        None,
    )
    .unwrap();

    // Verify shipment was created
    let result = query_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "shipment",
        encode_one(shipment_id).unwrap(),
        TEST_PRINCIPAL,
    );

    let shipment = decode_one::<Option<PrintableShipment>>(&result).unwrap();
    assert!(shipment.is_some());
    let shipment = shipment.unwrap();
    assert_eq!(shipment.name, "Test Package");
}
