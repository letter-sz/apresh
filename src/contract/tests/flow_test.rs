use apresh_crypto::hash_secret;
use apresh_types::{PrintableShipment, ShipmentInfo, ShipmentLocation, SizeCategory};
use candid::{decode_one, encode_one, Encode, Principal};
use common::pocket::pic;
use common::*;
use contract::consts::THIS_CANISTER_ID;
use pocket_ic::PocketIc;
use rstest::rstest;

mod common;

// fn mint_tokens(
//     pic: &PocketIc,
//     ledger_id: Principal,
//     to: Principal,
//     amount: u128,
// ) -> Result<(), String> {
//     let mint_args = Encode!(
//         &Account {
//             owner: to,
//             subaccount: None,
//         },
//         &amount
//     )
//     .unwrap();

//     let result = pic
//         .update_call(
//             ledger_id,
//             MINTER_PRINCIPAL, // minting account
//             "icrc1_transfer",
//             mint_args,
//         )
//         .map_err(|e| e.to_string())?;

//     match result {
//         WasmResult::Reply(_) => Ok(()),
//         _ => Err("Failed to mint tokens".to_string()),
//     }
// }

#[rstest]
fn test_create_shipment(test_shipment: TestEnvironmentWithShipment) {
    let TestEnvironmentWithShipment { pic, shipment_id } = test_shipment;

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

#[rstest]
#[case(1_u64, 1_u64)]
#[case(0_u64, 1_u64)]
#[case(0_u64, 1_000_000_000_u64)]
#[rstest]
fn test_create_shipment_with_funds(pic: PocketIc, #[case] value: u64, #[case] price: u64) {
    use common::POOR_PRINCIPAL;

    let customer_name = Some("Poor Customer".to_string());
    let shipment_name = "Unwanted Package".to_string();
    let secret = b"test_secret";
    let hashed_secret = hash_secret(secret);
    let expected_shipment_id = 10_u64;
    let channel_key = vec![0u8; 32];

    let shipment_info = ShipmentInfo::new(
        value,
        price,
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
        POOR_PRINCIPAL,
    );

    assert!(
        result.is_err(),
        "Expected error due to insufficient funds, but got success: {:?}",
        result
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("Insufficient balance"),
        "Expected 'Insufficient balance' error, got: {}",
        err
    );

    // Query all shipments to verify none were created
    let result = query_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "shipments",
        encode_one(()).unwrap(),
        POOR_PRINCIPAL,
    );

    let shipments = decode_one::<Vec<PrintableShipment>>(&result).unwrap();
    assert!(
        !shipments.iter().any(|s| s.name == shipment_name),
        "Found shipment '{}' when it should not exist",
        shipment_name
    );

    // Also verify the specific shipment doesn't exist
    let result = query_canister(
        &pic,
        Principal::from_text(THIS_CANISTER_ID).unwrap(),
        "shipment",
        encode_one(expected_shipment_id).unwrap(),
        POOR_PRINCIPAL,
    );

    let shipment = decode_one::<Option<PrintableShipment>>(&result).unwrap();
    assert!(
        shipment.is_none() || shipment.unwrap().name != shipment_name,
        "Found shipment when it should not exist"
    );
}

#[rstest]
fn test_get_transfer_fee(pic: PocketIc) {
    let result = pic
        .query_call(
            Principal::from_text(THIS_CANISTER_ID).unwrap(),
            Principal::anonymous(),
            "getTransferFee",
            encode_one(()).unwrap(),
        )
        .expect("Failed to get transfer fee");

    let fee = decode_one::<u64>(&get_reply_bytes(result)).unwrap();
    assert_eq!(fee, 10_000);
}
