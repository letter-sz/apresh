use candid::Principal;
use pocket_ic::{PocketIc, WasmResult};

pub fn query_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Vec<u8> {
    let result = pic
        .query_call(contract_id, principal, method, args)
        .unwrap();
    get_reply_bytes(result)
}

pub fn update_canister(
    pic: &PocketIc,
    contract_id: Principal,
    method: &str,
    args: Vec<u8>,
    principal: Principal,
) -> Vec<u8> {
    let result = pic
        .update_call(contract_id, principal, method, args)
        .unwrap();
    get_reply_bytes(result)
}

pub fn get_reply_bytes(result: WasmResult) -> Vec<u8> {
    match result {
        WasmResult::Reply(bytes) => bytes,
        _ => panic!("Unexpected result type"),
    }
}
