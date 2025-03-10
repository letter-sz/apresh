use crate::stable_memory::*;
use candid::Principal;
use engine::{
    actors::{carrier::Carrier, shipper::Shipper},
    models::shipment::Shipment,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell,
};
use std::cell::RefCell;

pub fn principal_to_bytes(principal: &Principal) -> Vec<u8> {
    principal.as_slice().to_vec()
}

pub fn get_memory(id: MemoryId) -> VirtualMemory<DefaultMemoryImpl> {
    MEMORY_MANAGER.with(|m| m.borrow().get(id))
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static SHIPMENTS: RefCell<StableBTreeMap<u64, Shipment, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(SHIPMENTS_ID))
    );

    pub static SHIPPERS: RefCell<StableBTreeMap<Vec<u8>, Shipper, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(SHIPPERS_ID))
    );

    pub static CARRIERS: RefCell<StableBTreeMap<Vec<u8>, Carrier, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(CARRIERS_ID))
    );

    pub static SHIPMENT_COUNTER: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableCell::init(get_memory(SHIPMENT_COUNTER_ID), 0).unwrap()
    );
}
