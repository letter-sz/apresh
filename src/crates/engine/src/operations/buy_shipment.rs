use crate::{
    models::shipment::{ChannelKey, ShipmentActions, ShipmentId},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier_id: ActorId,
    shipment_id: ShipmentId,
    channel_key: ChannelKey,
}

impl BuyShipmentOp {
    pub fn new(carrier_id: ActorId, shipment_id: ShipmentId, channel_key: ChannelKey) -> Self {
        Self {
            carrier_id,
            shipment_id,
            channel_key,
        }
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        if state.carrier(&self.carrier_id).is_none() {
            return Err(crate::Error::CarrierNotFound);
        }

        let value = {
            let shipment = state
                .shipment_mut(self.shipment_id)
                .ok_or(crate::Error::ShipmentNotFound)?;

            shipment.action(ShipmentActions::Buy(self.carrier_id))?;
            shipment.add_guest_to_channel(self.channel_key.clone());
            let value = shipment.info().value();
            value
        };

        let carrier = state.carrier_mut(&self.carrier_id).unwrap();
        carrier.add_shipment(self.shipment_id);

        Ok(value)
    }
}
