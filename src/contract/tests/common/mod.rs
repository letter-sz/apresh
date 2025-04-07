pub mod candid_types;
pub mod pocket;
pub mod queries;
pub mod test_data;

use candid::Principal;

pub use candid_types::*;
pub use pocket::*;
pub use queries::*;
pub use test_data::*;

pub const INIT_CYCLES: u128 = 2_000_000_000_000;
pub const TEST_PRINCIPAL: Principal = Principal::from_slice(&[1, 2, 3, 4]);
pub const POOR_PRINCIPAL: Principal = Principal::from_slice(&[5, 6, 7, 8]);
pub const MINTER_PRINCIPAL: Principal = Principal::from_slice(&[9, 10, 11, 12]);

pub const CONTRACT_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/contract.wasm");
pub const LEDGER_WASM: &[u8] = include_bytes!("../../scripts/icrc1_ledger.wasm.gz");
