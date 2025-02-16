use crate::{
    models::shipment::Message,
    state::{CanisterShipments, CanisterState},
    ActorId,
};

use super::StateOp;
use anyhow::anyhow;
use candid::Principal;

pub struct AddMessageOp {
    pub shipment_id: u64,
    pub message: Message,
    pub caller: ActorId,
}

impl AddMessageOp {
    pub fn new(shipment_id: u64, message: Message, caller: ActorId) -> Self {
        Self {
            shipment_id,
            message,
            caller,
        }
    }
}

impl StateOp<()> for AddMessageOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        let is_carrier = shipment
            .carrier_id()
            .map(|id| id == self.caller)
            .unwrap_or(false);

        if shipment.shipper_id() != self.caller && !is_carrier {
            return Err(crate::Error::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        shipment.attach_message(self.message.clone());

        Ok(())
    }
}
