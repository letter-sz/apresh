use candid::{decode_one, encode_one, Principal};
use candid::{CandidType, Encode};
use engine::models::shipment::{PrintableShipment, ShipmentInfo, ShipmentLocation, SizeCategory};
use engine::utils::hash_secret;
use pocket_ic::{PocketIc, PocketIcBuilder, WasmResult};
use rstest::{fixture, rstest};
use std::fs;

const INIT_CYCLES: u128 = 2_000_000_000_000;
const LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
const TEST_PRINCIPAL: Principal = Principal::from_slice(&[1, 2, 3, 4]);
const POOR_PRINCIPAL: Principal = Principal::from_slice(&[5, 6, 7, 8]);
const MINTER_PRINCIPAL: Principal = Principal::from_slice(&[9, 10, 11, 12]);

#[derive(CandidType)]
struct Metadata {
    key: String,
    value: String,
}

#[derive(CandidType)]
struct InitArgs {
    minting_account: Account,
    transfer_fee: u128,
    token_symbol: String,
    token_name: String,
    metadata: Vec<(String, MetadataValue)>,
    initial_balances: Vec<(Account, u128)>,
    archive_options: ArchiveOptions,
    feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType)]
struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType)]
enum MetadataValue {
    Nat(u128),
    Int(i128),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(CandidType)]
struct ArchiveOptions {
    num_blocks_to_archive: u64,
    trigger_threshold: u64,
    controller_id: Principal,
}

#[derive(CandidType)]
struct FeatureFlags {
    icrc2: bool,
}

#[derive(CandidType)]
enum LedgerArg {
    Init(InitArgs),
}

#[derive(CandidType)]
struct IcrcAccount {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType)]
struct IcrcTransferArg {
    from_subaccount: Option<Vec<u8>>,
    to: IcrcAccount,
    amount: u128,
    fee: Option<u128>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType)]
struct ApproveArgs {
    spender: Account,
    amount: u128,
    fee: Option<u128>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    expected_allowance: Option<u128>,
    expires_at: Option<u64>,
}

fn get_ledger_init_args() -> Vec<u8> {
    let init = LedgerArg::Init(InitArgs {
        minting_account: Account {
            owner: MINTER_PRINCIPAL,
            subaccount: None,
        },
        transfer_fee: 10_000,
        token_symbol: "ICRC1".to_string(),
        token_name: "L-ICRC1".to_string(),
        metadata: Vec::new(),
        initial_balances: vec![(
            Account {
                owner: TEST_PRINCIPAL,
                subaccount: None,
            },
            10_000_000_000,
        )],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            trigger_threshold: 2000,
            controller_id: TEST_PRINCIPAL,
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    });

    encode_one(init).unwrap()
}

fn mint_tokens(
    pic: &PocketIc,
    ledger_id: Principal,
    to: Principal,
    amount: u128,
) -> Result<(), String> {
    let mint_args = Encode!(
        &Account {
            owner: to,
            subaccount: None,
        },
        &amount
    )
    .unwrap();

    let result = pic
        .update_call(
            ledger_id,
            MINTER_PRINCIPAL, // minting account
            "icrc1_transfer",
            mint_args,
        )
        .map_err(|e| e.to_string())?;

    match result {
        WasmResult::Reply(_) => Ok(()),
        _ => Err("Failed to mint tokens".to_string()),
    }
}

struct TestEnvironment {
    pic: PocketIc,
    contract_id: Principal,
}

#[fixture]
fn test_env() -> TestEnvironment {
    let pic = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_application_subnet()
        .build();

    let ledger_id = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let ledger_init = get_ledger_init_args();
    let ledger_wasm = fs::read("scripts/icrc1_ledger.wasm.gz").expect("Failed to read ledger wasm");

    println!("Initializing Ledger ID: {}", ledger_id);
    pic.create_canister_with_id(None, None, ledger_id).unwrap();
    pic.add_cycles(ledger_id, INIT_CYCLES);
    pic.install_canister(ledger_id, ledger_wasm, ledger_init, None);

    // Create and install the contract
    let contract_id = pic.create_canister();
    pic.add_cycles(contract_id, INIT_CYCLES);
    let contract_wasm = fs::read("../../target/wasm32-unknown-unknown/release/contract.wasm")
        .expect("Failed to read contract wasm");
    pic.install_canister(contract_id, contract_wasm, vec![], None);

    // Approve contract to spend tokens
    let approve_args = ApproveArgs {
        spender: Account {
            owner: contract_id,
            subaccount: None,
        },
        amount: 1_000_000_000_000, // Large allowance
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        expected_allowance: None,
        expires_at: None,
    };

    pic.update_call(
        ledger_id,
        TEST_PRINCIPAL,
        "icrc2_approve",
        encode_one(approve_args).unwrap(),
    )
    .expect("Failed to approve");

    TestEnvironment { pic, contract_id }
}

fn query_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Vec<u8> {
    let result = pic
        .query_call(contract_id, principal, method, args)
        .unwrap();
    get_reply_bytes(result)
}

fn update_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Vec<u8> {
    let result = pic
        .update_call(contract_id, principal, method, args)
        .unwrap();
    get_reply_bytes(result)
}

struct TestEnvironmentWithShipment {
    test_env: TestEnvironment,
    shipment_id: u64,
}

#[fixture]
fn test_shipment(
    test_env: TestEnvironment,
    #[default("Test Package")] name: String,
) -> TestEnvironmentWithShipment {
    let customer_name = Some("Test Customer".to_string());
    let shipment_name = name;
    let secret = b"test_secret";
    let hashed_secret = hash_secret(secret);

    let shipment_info = ShipmentInfo::new(
        100, // price
        10,  // value
        ShipmentLocation::new("Origin".to_string(), 40.7128, -74.0060),
        ShipmentLocation::new("Destination".to_string(), 34.0522, -118.2437),
        SizeCategory::Envelope,
    );

    let result = update_canister(
        &test_env.pic,
        test_env.contract_id,
        "createShipment",
        Encode!(
            &customer_name,
            &shipment_name,
            &hashed_secret,
            &shipment_info
        )
        .unwrap(),
        TEST_PRINCIPAL,
    );

    let res: Result<u64, String> = decode_one(&result).unwrap();
    TestEnvironmentWithShipment {
        test_env,
        shipment_id: res.unwrap(),
    }
}

// Helper function to extract bytes from WasmResult
fn get_reply_bytes(result: WasmResult) -> Vec<u8> {
    match result {
        WasmResult::Reply(bytes) => bytes,
        _ => panic!("Unexpected result type"),
    }
}

#[rstest]
fn test_create_shipment(test_shipment: TestEnvironmentWithShipment) {
    let TestEnvironmentWithShipment {
        test_env,
        shipment_id,
    } = test_shipment;
    let TestEnvironment { pic, contract_id } = test_env;

    // Verify shipment was created
    let result = query_canister(
        &pic,
        contract_id,
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
fn test_create_shipment_with_zero_funds(
    test_env: TestEnvironment,
    #[case] value: u64,
    #[case] price: u64,
) {
    let TestEnvironment { pic, contract_id } = test_env;

    let customer_name = Some("Poor Customer".to_string());
    let shipment_name = "Unwanted Package".to_string();
    let secret = b"test_secret";
    let hashed_secret = hash_secret(secret);
    let expected_shipment_id = 10_u64;

    let shipment_info = ShipmentInfo::new(
        value,
        price,
        ShipmentLocation::new("Origin".to_string(), 40.7128, -74.0060),
        ShipmentLocation::new("Destination".to_string(), 34.0522, -118.2437),
        SizeCategory::Envelope,
    );

    let result = update_canister(
        &pic,
        contract_id,
        "createShipment",
        Encode!(
            &customer_name,
            &shipment_name,
            &hashed_secret,
            &shipment_info
        )
        .unwrap(),
        POOR_PRINCIPAL,
    );

    // Decode the result and expect an error string
    let create_result: Result<(Vec<u8>, u64), String> = decode_one(&result).unwrap();
    assert!(
        create_result.is_err(),
        "Expected error due to insufficient funds, but got success: {:?}",
        create_result
    );

    let err = create_result.unwrap_err();
    assert!(
        err.contains("InsufficientAllowance"),
        "Expected 'InsufficientAllowance' error, got: {}",
        err
    );

    // Query all shipments to verify none were created
    let result = query_canister(
        &pic,
        contract_id,
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
        contract_id,
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
fn test_get_transfer_fee(test_env: TestEnvironment) {
    let TestEnvironment { pic, contract_id } = test_env;
    let result = pic
        .query_call(
            contract_id,
            Principal::anonymous(),
            "getTransferFee",
            encode_one(()).unwrap(),
        )
        .expect("Failed to get transfer fee");

    let fee = decode_one::<u64>(&get_reply_bytes(result)).unwrap();
    assert_eq!(fee, 10_000);
}

