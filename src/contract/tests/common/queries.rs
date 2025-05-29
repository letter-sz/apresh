use candid::Principal;
use pocket_ic::{PocketIc, WasmResult};

pub fn query_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Result<Vec<u8>, String> {
    pic.query_call(contract_id, principal, method, args)
        .map(get_reply_bytes)
        .map_err(|e| e.to_string())
}

pub fn update_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Result<Vec<u8>, String> {
    pic.update_call(contract_id, principal, method, args)
        .map(get_reply_bytes)
        .map_err(|e| e.to_string())
}

pub fn get_reply_bytes(result: WasmResult) -> Vec<u8> {
    match result {
        WasmResult::Reply(bytes) => bytes,
        _ => panic!("Unexpected result type"),
    }
}
