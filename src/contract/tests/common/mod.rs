pub mod candid_types;
pub mod pocket;
pub mod queries;
pub mod test_data;

use candid::Principal;

pub use candid_types::*;
pub use queries::*;
pub use test_data::*;

pub const INIT_CYCLES: u128 = 2_000_000_000_000;
pub const ADMIN_PRINCIPAL: Principal = Principal::from_slice(&[9, 9, 9, 9]);
pub const TEST_PRINCIPAL: Principal = Principal::from_slice(&[1, 2, 3, 4]);
pub const POOR_PRINCIPAL: Principal = Principal::from_slice(&[9, 10, 11, 12]);
pub const MINTER_PRINCIPAL: Principal = Principal::from_slice(&[13, 14, 15, 16]);

pub const CONTRACT_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/contract.wasm");
pub const LEDGER_WASM: &[u8] = include_bytes!("../../scripts/icrc1_ledger.wasm.gz");
