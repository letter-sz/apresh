mod actor_collection;
mod shipments;

use crate::{
    actors::{carrier::Carrier, shipper::Shipper},
    models::shipment::{Shipment, ShipmentId},
    ActorId,
};
use actor_collection::ActorCollection;
use shipments::Shipments;

#[derive(Default)]
pub struct CanisterState {
    pub shippers: ActorCollection<Shipper>,
    pub carriers: ActorCollection<Carrier>,
    pub shipments: Shipments,
    pub shipment_counter: u64,
}

pub trait CanisterActors {
    fn shipper(&self, id: ActorId) -> Option<&Shipper>;
    fn carrier(&self, id: ActorId) -> Option<&Carrier>;
    fn shipper_mut(&mut self, id: ActorId) -> Option<&mut Shipper>;
    fn carrier_mut(&mut self, id: ActorId) -> Option<&mut Carrier>;
    fn create_shipper(&mut self, shipper: Shipper) -> &mut Shipper;
    fn create_carrier(&mut self, carrier: Carrier) -> &mut Carrier;
}

pub trait CanisterShipments {
    fn shipment(&self, id: ShipmentId) -> Option<&Shipment>;
    fn shipment_mut(&mut self, id: ShipmentId) -> Option<&mut Shipment>;
    fn create_shipment(&mut self, shipment: Shipment) -> &mut Shipment;
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
    fn shipper(&self, id: ActorId) -> Option<&Shipper> {
        self.shippers.get(&id)
    }

    fn carrier(&self, id: ActorId) -> Option<&Carrier> {
        self.carriers.get(&id)
    }

    fn shipper_mut(&mut self, id: ActorId) -> Option<&mut Shipper> {
        self.shippers.get_mut(&id)
    }

    fn carrier_mut(&mut self, id: ActorId) -> Option<&mut Carrier> {
        self.carriers.get_mut(&id)
    }

    fn create_shipper(&mut self, shipper: Shipper) -> &mut Shipper {
        let shipper_id = shipper.id();
        self.shippers.create(shipper);
        self.shippers.get_mut(&shipper_id).unwrap()
    }

    fn create_carrier(&mut self, carrier: Carrier) -> &mut Carrier {
        let carrier_id = carrier.id();
        self.carriers.create(carrier);
        self.carriers.get_mut(&carrier_id).unwrap()
    }
}

impl CanisterShipments for CanisterState {
    fn shipment(&self, id: ShipmentId) -> Option<&Shipment> {
        self.shipments.get(&id)
    }

    fn shipment_mut(&mut self, id: ShipmentId) -> Option<&mut Shipment> {
        self.shipments.get_mut(&id)
    }

    fn create_shipment(&mut self, shipment: Shipment) -> &mut Shipment {
        self.shipments.insert(self.shipment_counter, shipment);
        self.shipment_counter += 1;
        self.shipments
            .get_mut(&(self.shipment_counter - 1))
            .unwrap()
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
