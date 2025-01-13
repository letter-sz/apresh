use candid::Principal;

pub fn block_anonymous() -> Result<(), String> {
    if ic_cdk::caller() == Principal::anonymous() {
        Err("Cannot be called anonymously".to_string())
    } else {
        Ok(())
    }
}
