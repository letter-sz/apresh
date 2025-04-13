use candid::Principal;

mod memory;
use ic_stable_structures::{
    memory_manager::{MemoryId, VirtualMemory},
    DefaultMemoryImpl,
};
pub use memory::*;
use store::MEMORY_MANAGER;

pub fn principal_to_bytes(principal: &Principal) -> Vec<u8> {
    principal.as_slice().to_vec()
}

pub fn get_memory(id: MemoryId) -> VirtualMemory<DefaultMemoryImpl> {
    MEMORY_MANAGER.with(|m| m.borrow().get(id))
}

// thread_local! {
//     pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
//         MemoryManager::init(DefaultMemoryImpl::default())
//     );

//     pub static SHIPMENTS: RefCell<StableBTreeMap<u64, Shipment, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
//         StableBTreeMap::init(get_memory(SHIPMENTS_ID))
//     );

//     pub static SHIPPERS: RefCell<StableBTreeMap<Vec<u8>, Shipper, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
//         StableBTreeMap::init(get_memory(SHIPPERS_ID))
//     );

//     pub static CARRIERS: RefCell<StableBTreeMap<Vec<u8>, Carrier, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
//         StableBTreeMap::init(get_memory(CARRIERS_ID))
//     );

//     pub static SHIPMENT_COUNTER: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
//         StableCell::init(get_memory(SHIPMENT_COUNTER_ID), 0).unwrap()
//     );

//     pub(crate) static DB_MEMORY: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
//         StableBTreeMap::init(get_memory(DB_MEMORY_ID))
//     );

// }
