use candid::{encode_one, Principal};
use contract::consts::*;
use pocket_ic::{PocketIc, PocketIcBuilder};
use rstest::fixture;

use crate::{Account, ApproveArgs, INIT_CYCLES, MINTER_PRINCIPAL, TEST_PRINCIPAL};

use super::{ArchiveOptions, FeatureFlags, InitArgs, LedgerArg};

#[fixture]
pub fn pic() -> PocketIc {
    let pic = PocketIcBuilder::new()
        .with_nns_subnet()
        .with_application_subnet()
        .build();

    let contract_wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/contract.wasm").to_vec();

    // Create and install the contract
    let contract_id = Principal::from_text(THIS_CANISTER_ID).unwrap();
    pic.create_canister_with_id(None, None, contract_id)
        .unwrap();
    pic.add_cycles(contract_id, INIT_CYCLES);
    pic.install_canister(contract_id, contract_wasm, vec![], None);

    init_ledger(&pic, contract_id);

    pic
}

fn init_ledger(pic: &PocketIc, contract_id: Principal) {
    let ledger_init = LedgerArg::Init(InitArgs {
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
            100_00000000,
        )],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            trigger_threshold: 2000,
            controller_id: TEST_PRINCIPAL,
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    });

    // Load the wasm files for canisters
    let ledger_wasm = include_bytes!("../../scripts/icrc1_ledger.wasm.gz").to_vec();

    // Create and install the ledger
    let ledger_id = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    pic.create_canister_with_id(None, None, ledger_id).unwrap();
    pic.add_cycles(ledger_id, INIT_CYCLES);
    pic.install_canister(
        ledger_id,
        ledger_wasm,
        encode_one(ledger_init).unwrap(),
        None,
    );

    // Approve contract to spend tokens
    let approve_args = ApproveArgs {
        spender: Account {
            owner: contract_id,
            subaccount: None,
        },
        amount: 10000_00000000, // Large allowance
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
}
