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

        let carrier_id = shipment.carrier_id().ok_or(crate::Error::CarrierNotSet)?;

        if carrier_id != self.caller {
            return Err(crate::Error::NotAuthorizedAsCarrier);
        }

        shipment.attach_message(self.message.clone());

        Ok(())
    }
}
