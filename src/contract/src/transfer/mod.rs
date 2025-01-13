mod consts;
mod transfer_in;
mod transfer_out;
mod utils;

use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{Memo, NumTokens},
};

pub use transfer_in::transfer_in;
pub use transfer_out::transfer_out;

pub struct TransferInParams {
    pub from: Account,
    pub params: TransferParams,
}

pub struct TransferOutParams {
    pub to: Account,
    pub params: TransferParams,
}

pub struct TransferParams {
    pub amount: NumTokens,
    pub memo: Option<Memo>,
}
