use apresh_store::Guard;
use balances::{balances, Balances};
use candid::Principal;
use icrc_ledger_types::icrc1::transfer::Memo;

use crate::{ADMIN, CANISTER_LOCKED};

fn is_admin() -> bool {
    ADMIN.with_borrow(|caller| *caller == ic_cdk::caller())
}

pub fn assert_admin() {
    #[cfg(feature = "admin")]
    if !is_admin() {
        ic_cdk::trap("Not authorized");
    }
}

pub fn assert_whitelisted() {
    #[cfg(feature = "whitelist")]
    if !crate::WHITELIST.with_borrow(|whitelist| whitelist.contains(&ic_cdk::caller())) {
        ic_cdk::trap("Not whitelisted");
    }
    #[cfg(not(feature = "whitelist"))]
    if ic_cdk::caller() == candid::Principal::anonymous() {
        ic_cdk::trap("Caller is anonymous");
    }

    if !is_admin() && CANISTER_LOCKED.with_borrow(|locked| *locked) {
        ic_cdk::trap("Canister is locked");
    }
}

pub fn memo(purpose: &str, amount: u64) -> Option<Memo> {
    if purpose.len() > 10 {
        // Memo has a limit of 32 bytes.
        // With constrained size it should allow for up to a million shipments.
        unreachable!("Memo purpose is longer than expected");
    }

    let memo = format!("Apresh: {} of {}", purpose, amount)
        .as_bytes()
        .to_vec();

    if memo.len() > 32 {
        return None;
    }

    Some(Memo::from(memo))
}

pub fn callers_balances() -> Guard<Balances> {
    balances_of(ic_cdk::caller())
}

pub fn balances_of(principal: Principal) -> Guard<Balances> {
    let caller_bytes = principal.as_slice().to_vec();
    balances(caller_bytes)
}
