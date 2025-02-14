use candid::Principal;

use super::StateOp;
use crate::{
    models::shipment::{Message, ShipmentId},
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

impl StateOp<Vec<Message>> for ReadMessageOp {
    type Error = crate::Error;
    fn read(&self, state: &CanisterState) -> crate::Result<Vec<Message>> {
        let shipment = state
            .shipment(self.shipment_id)
            .filter(|&v| v.shipper_id() == self.caller)
            .ok_or(crate::Error::NotAuthorizedAsShipper)?;

        Ok(shipment.messages().clone())
    }
}
