use crate::{
    models::shipment::{Shipment, ShipmentId, ShipmentInfo},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::{CanisterState, StateOp};
use crate::actors::Actor;

#[derive(Debug)]
pub struct CreateShipmentOp<'a> {
    creator: ActorId,
    hashed_secret: &'a [u8],
    shipment_name: &'a str,
    info: &'a ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator: ActorId,
        hashed_secret: &'a [u8],
        shipment_name: &'a str,
        info: &'a ShipmentInfo,
        timestamp: u64,
    ) -> Self {
        Self {
            creator,
            hashed_secret,
            shipment_name,
            info,
            timestamp,
        }
    }
}

impl<'a> StateOp<ShipmentId> for CreateShipmentOp<'a> {
    type Error = crate::errors::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<ShipmentId> {
        let new_shipment_id = state.shipment_counter();

        if new_shipment_id == u64::MAX {
            return Err(crate::Error::ShipmentLimitReached);
        }

        let mut shipper = state
            .shipper_mut(&self.creator)
            .ok_or(crate::Error::ShipperNotFound)?;

        let shipment = Shipment::new(
            self.timestamp,
            shipper.id(),
            new_shipment_id,
            &self.hashed_secret,
            self.shipment_name,
            &self.info,
        );

        shipper.add_shipment(new_shipment_id);
        state.create_shipment(shipment);

        Ok(new_shipment_id)
    }
}
