use apresh_types::{ActorId, Channel, Message, Shipment, ShipmentId, ShipmentKey};

use super::StateOp;
use crate::state::CanisterState;

pub struct ReadMessageOp<'a> {
    pub shipment: &'a Shipment,
    pub caller: ActorId,
}

impl<'a> ReadMessageOp<'a> {
    pub fn new(shipment: &'a Shipment, caller: ActorId) -> Self {
        Self { shipment, caller }
    }
}

impl StateOp<Channel> for ReadMessageOp<'_> {
    type Error = crate::EngineError;
    fn read(&self, state: &CanisterState) -> crate::Result<Channel> {
        let is_shipper = self.shipment.shipper_id() == &self.caller;
        let is_carrier = self.shipment.carrier_id() == &Some(self.caller);

        if !is_shipper && !is_carrier {
            return Err(crate::EngineError::NotAuthorizedAsNeitherCarrierNorShipper);
        }

        Ok(self.shipment.channel().clone())
    }
}
