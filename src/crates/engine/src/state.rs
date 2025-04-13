use candid::CandidType;
use serde::Deserialize;
use store::Record;
use types::{ActorId, Carrier, CarrierKey, Shipment, ShipmentId, ShipmentKey, Shipper, ShipperKey};

#[derive(CandidType, Default, Deserialize)]
pub struct CanisterState {
    shipment_counter: u64,
}

impl CanisterState {
    pub fn set_shipment_counter(&mut self, counter: u64) {
        self.shipment_counter = counter;
    }
}

pub trait CanisterActors {
    fn shipper(&self, id: &ActorId) -> Option<Shipper>;
    fn carrier(&self, id: &ActorId) -> Option<Carrier>;
    fn create_shipper(&mut self, shipper: Shipper) -> Shipper;
    fn create_carrier(&mut self, carrier: Carrier) -> Carrier;
}

pub trait CanisterShipments {
    fn shipment(&self, id: ShipmentId) -> Option<Shipment>;
    fn create_shipment(&mut self, shipment: Shipment) -> Result<(), crate::errors::EngineError>;
    fn shipments(&self) -> Vec<Shipment>;
    fn shipment_counter(&self) -> u64;
}

impl CanisterActors for CanisterState {
    fn shipper(&self, id: &ActorId) -> Option<Shipper> {
        Shipper::get(ShipperKey(*id))
    }

    fn carrier(&self, id: &ActorId) -> Option<Carrier> {
        Carrier::get(CarrierKey(*id))
    }

    fn create_shipper(&mut self, shipper: Shipper) -> Shipper {
        let key = ShipperKey(shipper.id());
        Shipper::set(shipper);
        Shipper::get(key).unwrap()
    }

    fn create_carrier(&mut self, carrier: Carrier) -> Carrier {
        let key = CarrierKey(carrier.id());
        Carrier::set(carrier);
        Carrier::get(key).unwrap()
    }
}

impl CanisterShipments for CanisterState {
    fn shipment(&self, id: ShipmentId) -> Option<Shipment> {
        Shipment::get(ShipmentKey(id))
    }

    fn create_shipment(&mut self, shipment: Shipment) -> Result<(), crate::errors::EngineError> {
        assert_eq!(Shipment::get(ShipmentKey(shipment.id())), None);
        Shipment::set(shipment);
        self.shipment_counter = self
            .shipment_counter
            .checked_add(1)
            .ok_or(crate::errors::EngineError::ShipmentLimitReached)?;

        Ok(())
    }

    fn shipments(&self) -> Vec<Shipment> {
        Shipment::range_scan(None, None)
            .into_iter()
            .map(|key| Shipment::get(key).unwrap())
            .collect()
    }

    fn shipment_counter(&self) -> u64 {
        self.shipment_counter
    }
}
