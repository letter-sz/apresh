use apresh_store::Record;
use apresh_types::{ActorId, Message, Shipment, ShipmentKey};

use crate::state::CanisterState;

use super::StateOp;
use anyhow::anyhow;
use candid::Principal;

const MAX_MESSAGE_LENGTH: usize = 4096;

pub struct AddMessageOp<'a> {
    pub shipment: &'a mut Shipment,
    pub message: Message,
    pub caller: ActorId,
}

impl<'a> AddMessageOp<'a> {
    pub fn new(shipment: &'a mut Shipment, message: Message, caller: ActorId) -> Self {
        Self {
            shipment,
            message,
            caller,
        }
    }
}

impl StateOp<()> for AddMessageOp<'_> {
    type Error = crate::EngineError;

    fn apply(self, state: &mut CanisterState) -> Result<(), Self::Error> {
        if self.message.len() > MAX_MESSAGE_LENGTH {
            return Err(crate::EngineError::MessageTooLong);
        }

        let is_carrier = self
            .shipment
            .carrier_id()
            .map(|id| id == self.caller)
            .unwrap_or(false);

        if self.shipment.shipper_id() != self.caller && !is_carrier {
            return Err(crate::EngineError::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        self.shipment.attach_message(self.message.clone());

        Ok(())
    }
}
