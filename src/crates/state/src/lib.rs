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

pub mod db;
mod memory;
pub use memory::*;

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

    pub(crate) static DB_MEMORY: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(DB_MEMORY_ID))
    );

}

#[macro_export]
macro_rules! get {
    ($type:ty, $key:expr) => {
        if std::any::TypeId::of::<$type>() == std::any::TypeId::of::<Shipment>() {
            Ok(SHIPMENTS.with_borrow(|s| s.get(&$key)))
        } else {
            Err("Invalid type for get macro".to_string())
        }
    };
}

#[macro_export]
macro_rules! set {
    ($type:ty, $key:expr, $value:expr) => {
        if std::any::TypeId::of::<$type>() == std::any::TypeId::of::<Shipment>() {
            Ok(SHIPMENTS.with_borrow_mut(|s| s.insert($key, $value)))
        } else {
            Err("Invalid type for set macro".to_string())
        }
    };
}

#[cfg(test)]
mod tests {
    use engine::models::shipment::{ShipmentInfo, ShipmentLocation, SizeCategory};

    use super::*;

    #[test]
    fn test_reference() {
        let shipment = get!(Shipment, 1);

        assert_eq!(Ok(None), shipment);

        set!(
            Shipment,
            1,
            Shipment::new(
                1,
                engine::ActorId(Principal::anonymous()),
                1,
                vec![],
                vec![],
                "Test Shipment",
                &ShipmentInfo::new(
                    1,
                    1,
                    ShipmentLocation::new("Test Source".to_string(), 0.0, 0.0),
                    ShipmentLocation::new("Test Destination".to_string(), 0.0, 0.0),
                    SizeCategory::Envelope
                )
            )
        )
        .unwrap();

        let shipment = get!(Shipment, 1).unwrap().unwrap();

        assert_eq!(shipment.id(), 1);
    }
}
