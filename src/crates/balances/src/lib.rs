mod balances;

use std::cell::RefCell;

use apresh_store::{BALANCES_MEMORY_ID, Guard, get_memory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, memory_manager::VirtualMemory};

pub use balances::*;

thread_local! {
    static BALANCES: RefCell<StableBTreeMap<Vec<u8>, Balances, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(BALANCES_MEMORY_ID))
    );
}

pub fn balances(caller_bytes: Vec<u8>) -> Guard<Balances> {
    BALANCES.with_borrow_mut(|balances| {
        let balances = balances.get(&caller_bytes).unwrap_or_default();
        Guard::new_with_key(caller_bytes, balances)
    })
}
