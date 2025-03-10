mod actor_collection;
mod icp;
mod shipments;

use crate::{
    actors::{carrier::Carrier, shipper::Shipper},
    models::shipment::{Shipment, ShipmentId},
    ActorId,
};
use actor_collection::ActorCollection;
use candid::CandidType;
use serde::Deserialize;
use shipments::Shipments;

#[cfg(feature = "icp")]
#[derive(CandidType, Default, Deserialize)]
pub struct CanisterState {
    shippers: ActorCollection<Shipper>,
    carriers: ActorCollection<Carrier>,
    shipments: Shipments,
    shipment_counter: u64,
}


impl CanisterState {
    pub fn set_shipment_counter(&mut self, counter: u64) {
        self.shipment_counter = counter;
    }
}

pub trait CanisterCollections {
    fn shippers(&self) -> &ActorCollection<Shipper>;
    fn carriers(&self) -> &ActorCollection<Carrier>;
    fn shipments_collection(&self) -> &Shipments;

    #[cfg(feature = "icp")]
    fn shippers_mut(&mut self) -> &mut ActorCollection<Shipper>;

    #[cfg(feature = "icp")]
    fn carriers_mut(&mut self) -> &mut ActorCollection<Carrier>;

    #[cfg(feature = "icp")]
    fn shipments_mut(&mut self) -> &mut Shipments;
}

pub trait CanisterActors {
    fn shipper(&self, id: &ActorId) -> Option<&Shipper>;
    fn carrier(&self, id: &ActorId) -> Option<&Carrier>;
    fn shipper_mut(&mut self, id: &ActorId) -> Option<&mut Shipper>;
    fn carrier_mut(&mut self, id: &ActorId) -> Option<&mut Carrier>;
    fn create_shipper(&mut self, shipper: Shipper) -> &mut Shipper;
    fn create_carrier(&mut self, carrier: Carrier) -> &mut Carrier;
}

pub trait CanisterShipments {
    fn shipment(&self, id: ShipmentId) -> Option<&Shipment>;
    fn shipment_mut(&mut self, id: ShipmentId) -> Option<&mut Shipment>;
    fn create_shipment(&mut self, shipment: Shipment) -> Result<(), crate::errors::Error>;
    fn shipments(&self) -> Vec<&Shipment>;
    fn shipment_counter(&self) -> u64;
    fn shipper_and_shipment(
        &self,
        actor_id: ActorId,
        shipment_id: ShipmentId,
    ) -> Option<(&Shipper, &Shipment)>;
    fn carrier_and_shipment(
        &self,
        actor_id: ActorId,
        shipment_id: ShipmentId,
    ) -> Option<(&Carrier, &Shipment)>;
}

impl CanisterActors for CanisterState {
    fn shipper(&self, id: &ActorId) -> Option<&Shipper> {
        self.shippers.get(id)
    }

    fn carrier(&self, id: &ActorId) -> Option<&Carrier> {
        self.carriers.get(id)
    }

    fn shipper_mut(&mut self, id: &ActorId) -> Option<&mut Shipper> {
        self.shippers.get_mut(id)
    }

    fn carrier_mut(&mut self, id: &ActorId) -> Option<&mut Carrier> {
        self.carriers.get_mut(id)
    }

    fn create_shipper(&mut self, shipper: Shipper) -> &mut Shipper {
        self.shippers.create(shipper)
    }

    fn create_carrier(&mut self, carrier: Carrier) -> &mut Carrier {
        self.carriers.create(carrier)
    }
}

impl CanisterShipments for CanisterState {
    fn shipment(&self, id: ShipmentId) -> Option<&Shipment> {
        self.shipments.get(&id)
    }

    fn shipment_mut(&mut self, id: ShipmentId) -> Option<&mut Shipment> {
        self.shipments.get_mut(&id)
    }

    fn create_shipment(&mut self, shipment: Shipment) -> Result<(), crate::errors::Error> {
        self.shipments.insert(shipment.id(), shipment);
        self.shipment_counter = self
            .shipment_counter
            .checked_add(1)
            .ok_or(crate::errors::Error::ShipmentLimitReached)?;

        Ok(())
    }

    fn shipments(&self) -> Vec<&Shipment> {
        self.shipments.values().collect()
    }

    fn shipment_counter(&self) -> u64 {
        self.shipment_counter
    }

    fn shipper_and_shipment(
        &self,
        actor_id: ActorId,
        shipment_id: ShipmentId,
    ) -> Option<(&Shipper, &Shipment)> {
        let shippers = &self.shippers;
        let shipments = &self.shipments;

        let shipper = shippers.get(&actor_id)?;
        let shipment = shipments.get(&shipment_id)?;

        Some((shipper, shipment))
    }

    fn carrier_and_shipment(
        &self,
        actor_id: ActorId,
        shipment_id: ShipmentId,
    ) -> Option<(&Carrier, &Shipment)> {
        let carriers = &self.carriers;
        let shipments = &self.shipments;

        let carrier = carriers.get(&actor_id)?;
        let shipment = shipments.get(&shipment_id)?;

        Some((carrier, shipment))
    }
}

impl CanisterCollections for CanisterState {
    fn shippers(&self) -> &ActorCollection<Shipper> {
        &self.shippers
    }

    fn carriers(&self) -> &ActorCollection<Carrier> {
        &self.carriers
    }

    fn shipments_collection(&self) -> &Shipments {
        &self.shipments
    }

    #[cfg(feature = "icp")]
    fn shippers_mut(&mut self) -> &mut ActorCollection<Shipper> {
        &mut self.shippers
    }

    #[cfg(feature = "icp")]
    fn carriers_mut(&mut self) -> &mut ActorCollection<Carrier> {
        &mut self.carriers
    }

    #[cfg(feature = "icp")]
    fn shipments_mut(&mut self) -> &mut Shipments {
        &mut self.shipments
    }
}
