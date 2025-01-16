use candid::Principal;

use super::StateOp;
use crate::state::CanisterState;

pub struct ReadMessageOp {
    pub shipment_id: u64,
    pub caller: Principal,
}

impl ReadMessageOp {
    pub fn new(shipment_id: u64, caller: Principal) -> Self {
        Self {
            shipment_id,
            caller,
        }
    }
}

impl StateOp<Option<String>> for ReadMessageOp {
    type Error = anyhow::Error;

    fn read(&self, state: &CanisterState) -> Result<Option<String>, Self::Error> {
        Ok(state
            .shipments
            .get(&self.shipment_id)
            .filter(|v| v.shipper_id() == self.caller)
            .and_then(|v| v.message())
            .map(|v| v.to_string()))
    }
}
