use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use super::consts::LEDGER_CANISTER_ID;

pub fn get_account_from_principal(principal: Principal) -> Account {
    Account::from(principal)
}

pub fn get_canister_default_account() -> Account {
    get_account_from_principal(ic_cdk::api::id())
}

pub fn get_ledger_principal() -> Principal {
    Principal::from_text(LEDGER_CANISTER_ID).expect("Could not decode the principal.")
}
