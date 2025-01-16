use engine::models::shipment::{ShipmentActions, ShipmentId};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier: Carrier,
    shipment_id: ShipmentId,
}

impl BuyShipmentOp {
    pub fn new(carrier: Carrier, shipment_id: ShipmentId) -> Self {
        Self {
            carrier,
            shipment_id,
        }
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<Cost, Self::Error> {
        let carrier = match state.carriers.get_mut(&self.carrier.id()) {
            Some(carrier) => carrier,
            None => state.carriers.create(self.carrier.clone()),
        };

        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(anyhow!("Shipment not found"))?;

        shipment.action(ShipmentActions::Buy(self.carrier.id()))?;
        carrier.add_shipment(self.shipment_id);

        Ok(shipment.info().value())
    }
}
