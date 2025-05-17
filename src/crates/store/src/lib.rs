mod guard;
mod guard_container;
mod traits;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

pub use guard::*;
pub use guard_container::*;
pub use traits::*;

pub const DB_MEMORY_ID: MemoryId = MemoryId::new(7);
pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(6);

pub fn get_memory(id: MemoryId) -> VirtualMemory<DefaultMemoryImpl> {
    MEMORY_MANAGER.with(|m| m.borrow().get(id))
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub(crate) static DB_MEMORY: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(DB_MEMORY_ID))
    );
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum StoreError {
    #[error("Store error: {0}")]
    Other(String),
}

#[test]
fn test_vec_ord() {
    assert!(vec![1, 2] < vec![1, 3]);
    assert!(vec![1] < vec![1, 2]);
    assert!(vec![1, 2] < vec![2]);
}
