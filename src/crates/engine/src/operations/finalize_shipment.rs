use crate::{
    models::shipment::{ShipmentActions, ShipmentId}, state::{CanisterActors, CanisterShipments}, ActorId
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

pub struct FinalizeShipmentOp {
    caller: ActorId,
    shipment_id: ShipmentId,
    secret_key: Option<String>,
}

impl FinalizeShipmentOp {
    pub fn new(shipment_id: ShipmentId, secret_key: Option<String>, caller: ActorId) -> Self {
        Self {
            shipment_id,
            secret_key,
            caller,
        }
    }
}

impl StateOp<FinalizeShipmentResult> for FinalizeShipmentOp {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<FinalizeShipmentResult, anyhow::Error> {
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::errors::Error::ShipmentNotFound)?;

        let carrier_id = shipment
            .carrier_id()
            .ok_or(crate::errors::Error::CarrierNotSet)?;

        let value = shipment.info().value();
        let price = shipment.info().price();

        shipment.action(ShipmentActions::MarkDelivered {
            secret_key: self.secret_key.clone(),
            caller: self.caller,
        })?;

        let carrier = state
            .carrier_mut(&carrier_id)
            .ok_or(crate::errors::Error::CarrierNotFound)?;

        Ok(FinalizeShipmentResult {
            carrier_id: carrier.id(),
            value,
            price,
        })
    }
}
