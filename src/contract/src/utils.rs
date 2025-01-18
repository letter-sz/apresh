use candid::Principal;
use icrc_ledger_types::icrc1::transfer::Memo;

pub fn block_anonymous() -> Result<(), String> {
    if ic_cdk::caller() == Principal::anonymous() {
        Err("Cannot be called anonymously".to_string())
    } else {
        Ok(())
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
