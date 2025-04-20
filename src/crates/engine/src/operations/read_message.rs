use apresh_types::{ActorId, Channel, Message, ShipmentId, ShipmentKey};

use super::StateOp;
use crate::state::CanisterState;

pub struct ReadMessageOp {
    pub shipment: ShipmentKey,
    pub caller: ActorId,
}

impl ReadMessageOp {
    pub fn new(shipment_id: ShipmentId, caller: ActorId) -> Self {
        Self {
            shipment: ShipmentKey(shipment_id),
            caller,
        }
    }
}

impl StateOp<Channel> for ReadMessageOp {
    type Error = crate::EngineError;
    fn read(&self, state: &CanisterState) -> crate::Result<Channel> {
        let shipment = self
            .shipment
            .get()
            .ok_or(crate::EngineError::ShipmentNotFound)?;

        let is_shipper = shipment.shipper_id() == self.caller;
        let is_carrier = shipment.carrier_id() == Some(self.caller);

        if !is_shipper && !is_carrier {
            return Err(crate::EngineError::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        Ok(shipment.channel().clone())
    }
}
