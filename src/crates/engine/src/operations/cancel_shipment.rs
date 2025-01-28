use crate::{
    models::shipment::{ShipmentActions, ShipmentId},
    ActorId,
};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct CancelShipmentOp {
    shipper: ActorId,
    shipment_id: ShipmentId,
}

impl CancelShipmentOp {
    pub fn new(shipper: ActorId, shipment_id: ShipmentId) -> Self {
        Self {
            shipper,
            shipment_id,
        }
    }
}

impl StateOp<Cost> for CancelShipmentOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        let shipper = state
            .shippers
            .get_mut(&self.shipper)
            .ok_or(crate::Error::ShipperNotFound)?;

        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        shipment.action(ShipmentActions::Cancel {
            shipper: self.shipper,
        })?;

        Ok(shipment.info().value())
    }
}
