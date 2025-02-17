use candid::Principal;

use super::StateOp;
use crate::{
    models::shipment::{Channel, Message, ShipmentId},
    state::{CanisterShipments, CanisterState},
    ActorId,
};

pub struct ReadMessageOp {
    pub shipment_id: ShipmentId,
    pub caller: ActorId,
}

impl ReadMessageOp {
    pub fn new(shipment_id: ShipmentId, caller: ActorId) -> Self {
        Self {
            shipment_id,
            caller,
        }
    }
}

impl StateOp<Channel> for ReadMessageOp {
    type Error = crate::Error;
    fn read(&self, state: &CanisterState) -> crate::Result<Channel> {
        let shipment = state
            .shipment(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        let is_shipper = shipment.shipper_id() == self.caller;
        let is_carrier = shipment.carrier_id() == Some(self.caller);

        if !is_shipper && !is_carrier {
            return Err(crate::Error::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        Ok(shipment.channel().clone())
    }
}
