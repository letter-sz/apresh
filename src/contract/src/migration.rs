use engine::actors::carrier::Carrier;
use engine::actors::shipper::Shipper;
use engine::models::shipment::Shipment;
use engine::state::{CanisterCollections, CanisterShipments};

use crate::stable_state::{principal_to_bytes, CARRIERS, SHIPMENTS, SHIPMENT_COUNTER, SHIPPERS};
use crate::STATE;

pub fn migrate_shippers() {
    STATE.with_borrow(|state| {
        SHIPPERS.with_borrow_mut(|shippers| {
            for (principal, shipper) in state.shippers().iter() {
                let key = principal_to_bytes(principal);
                shippers.insert(key, shipper.clone());
            }
        });
    });
}

pub fn migrate_carriers() {
    STATE.with_borrow(|state| {
        CARRIERS.with_borrow_mut(|carriers| {
            for (principal, carrier) in state.carriers().iter() {
                let key = principal_to_bytes(principal);
                carriers.insert(key, carrier.clone());
            }
        });
    });
}

pub fn migrate_shipments() {
    STATE.with_borrow(|state| {
        SHIPMENTS.with_borrow_mut(|shipments| {
            for (id, shipment) in state.shipments_collection().iter() {
                shipments.insert(*id, shipment.clone());
            }
        });

        SHIPMENT_COUNTER.with_borrow_mut(|counter| {
            counter.set(state.shipment_counter()).unwrap();
        });
    });
}

pub fn load_shippers() {
    STATE.with_borrow_mut(|state| {
        SHIPPERS.with_borrow_mut(|shippers| {
            let shippers: Vec<Shipper> = shippers.values().into_iter().collect();
            state.shippers_mut().insert_multiple(shippers);
        });
    });
}

pub fn load_carriers() {
    STATE.with_borrow_mut(|state| {
        CARRIERS.with_borrow_mut(|carriers| {
            let carriers: Vec<Carrier> = carriers.values().into_iter().collect();
            state.carriers_mut().insert_multiple(carriers);
        });
    });
}

pub fn load_shipments() {
    STATE.with_borrow_mut(|state| {
        SHIPMENTS.with_borrow_mut(|shipments| {
            let shipments: Vec<Shipment> = shipments.values().into_iter().collect();
            state.shipments_mut().insert_multiple(shipments);
        });

        SHIPMENT_COUNTER.with_borrow_mut(|counter| {
            state.set_shipment_counter(*counter.get());
        });
    });
}
