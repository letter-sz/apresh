use crate::{
    state::{CanisterShipments, CanisterState},
    ActorId,
};

use super::StateOp;
use anyhow::anyhow;
use candid::Principal;

pub struct AddMessageOp<'a> {
    pub shipment_id: u64,
    pub message: &'a str,
    pub caller: ActorId,
}

impl<'a> AddMessageOp<'a> {
    pub fn new(shipment_id: u64, message: &'a str, caller: ActorId) -> Self {
        Self {
            shipment_id,
            message,
            caller,
        }
    }
}

impl<'a> StateOp<()> for AddMessageOp<'a> {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        let carrier_id = shipment.carrier_id().ok_or(crate::Error::CarrierNotSet)?;

        if carrier_id != self.caller {
            return Err(crate::Error::NotAuthorizedAsCarrier);
        }

        shipment.attach_message(self.message.to_string());

        Ok(())
    }
}
