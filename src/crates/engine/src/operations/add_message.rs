use store::Record;
use types::{ActorId, Message, Shipment};

use crate::state::{CanisterShipments, CanisterState};

use super::StateOp;
use anyhow::anyhow;
use candid::Principal;

const MAX_MESSAGE_LENGTH: usize = 4096;

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
    type Error = crate::EngineError;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        if self.message.len() > MAX_MESSAGE_LENGTH {
            return Err(crate::EngineError::MessageTooLong);
        }

        let mut shipment = state
            .shipment(self.shipment_id)
            .ok_or(crate::EngineError::ShipmentNotFound)?;

        let is_carrier = shipment
            .carrier_id()
            .map(|id| id == self.caller)
            .unwrap_or(false);

        if shipment.shipper_id() != self.caller && !is_carrier {
            return Err(crate::EngineError::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        shipment.attach_message(self.message.clone());
        Shipment::set(shipment);

        Ok(())
    }
}
