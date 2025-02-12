use crate::{
    models::shipment::{ShipmentActions, ShipmentId}, state::{CanisterActors, CanisterShipments}, ActorId
};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier_id: ActorId,
    shipment_id: ShipmentId,
}

impl BuyShipmentOp {
    pub fn new(carrier_id: ActorId, shipment_id: ShipmentId) -> Self {
        Self {
            carrier_id,
            shipment_id,
        }
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        if state.carrier(&self.carrier_id).is_none() {
            return Err(crate::Error::CarrierNotFound);
        }
        
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        shipment.action(ShipmentActions::Buy(self.carrier_id))?;
        let value = shipment.info().value();

        let carrier = state.carrier_mut(&self.carrier_id).unwrap();
        carrier.add_shipment(self.shipment_id);

        Ok(value)
    }
}
