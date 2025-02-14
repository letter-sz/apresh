use candid::Principal;
use icrc_ledger_types::icrc1::transfer::Memo;

pub fn assert_admin() {
    #[cfg(feature = "admin")]
    if ADMIN.with_borrow(|caller| *caller != ic_cdk::caller()) {
        ic_cdk::trap("Not authorized");
    }
}

pub fn assert_whitelisted() {
    #[cfg(feature = "whitelist")]
    if !WHITELIST.with_borrow(|whitelist| whitelist.contains(&ic_cdk::caller())) {
        ic_cdk::trap("Not whitelisted");
    }
    #[cfg(not(feature = "whitelist"))]
    if ic_cdk::caller() == Principal::anonymous() {
        ic_cdk::trap("Caller is anonymous");
    }
}

pub fn memo(purpose: &str, shipment_id: u64) -> Option<Memo> {
    if purpose.len() > 10 {
        // Memo has a limit of 32 bytes.
        // With constrained size it should allow for up to a million shipments.
        unreachable!("Memo purpose is longer than expected");
    }

    let memo = format!("Apresh: {} for {}", purpose, shipment_id)
        .as_bytes()
        .to_vec();

    if memo.len() > 32 {
        return None;
    }

    Some(Memo::from(memo))
}
