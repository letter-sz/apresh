use super::StateOp;
use crate::state::CanisterState;
use anyhow::anyhow;
use candid::Principal;

pub struct AddMessageOp<'a> {
    pub shipment_id: u64,
    pub message: &'a str,
    pub caller: Principal,
}

impl<'a> AddMessageOp<'a> {
    pub fn new(shipment_id: u64, message: &'a str, caller: Principal) -> Self {
        Self { shipment_id, message, caller }
    }
}

impl<'a> StateOp<()> for AddMessageOp<'a> {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(anyhow!("Shipment not found"))?;

        if shipment.carrier_id().is_some_and(|v| v == self.caller) {
            shipment.attach_message(self.message.to_string());
        } else {
            return Err(anyhow!("Only the carrier can add an encrypted message"));
        }

        Ok(())
    }
}
