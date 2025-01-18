use crate::{
    models::shipment::{ShipmentActions, ShipmentId},
    ActorId,
};

use super::StateOp;
use crate::{actors::Actor, state::CanisterState};
use anyhow::anyhow;

pub struct FinalizeShipmentResult {
    carrier_id: ActorId,
    value: u64,
    price: u64,
}

impl FinalizeShipmentResult {
    pub fn carrier_id(&self) -> &ActorId {
        &self.carrier_id
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn price(&self) -> u64 {
        self.price
    }
}

pub struct FinalizeShipmentOp<'a> {
    caller: ActorId,
    shipment_id: ShipmentId,
    secret_key: Option<&'a str>,
}

impl<'a> FinalizeShipmentOp<'a> {
    pub fn new(shipment_id: ShipmentId, secret_key: Option<&'a str>, caller: ActorId) -> Self {
        Self {
            shipment_id,
            secret_key,
            caller,
        }
    }
}

impl<'a> StateOp<FinalizeShipmentResult> for FinalizeShipmentOp<'a> {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<FinalizeShipmentResult, anyhow::Error> {
        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(crate::errors::Error::ShipmentNotFound)?;

        let carrier_id = shipment
            .carrier_id()
            .ok_or(crate::errors::Error::CarrierNotSet)?;

        let carrier = state
            .carriers
            .get_mut(&carrier_id)
            .ok_or(crate::errors::Error::CarrierNotFound)?;

        shipment.action(ShipmentActions::MarkDelivered {
            secret_key: self.secret_key,
            caller: self.caller,
        })?;

        Ok(FinalizeShipmentResult {
            carrier_id: carrier.id(),
            value: shipment.info().value(),
            price: shipment.info().price(),
        })
    }
}
