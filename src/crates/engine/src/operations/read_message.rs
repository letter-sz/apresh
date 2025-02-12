use candid::Principal;

use super::StateOp;
use crate::{
    models::shipment::ShipmentId,
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

impl StateOp<Option<String>> for ReadMessageOp {
    type Error = crate::Error;

    fn read(&self, state: &CanisterState) -> crate::Result<Option<String>> {
        Ok(state
            .shipment(self.shipment_id)
            .filter(|&v| v.shipper_id() == self.caller)
            .and_then(|v| v.message())
            .map(|v| v.to_string()))
    }
}
