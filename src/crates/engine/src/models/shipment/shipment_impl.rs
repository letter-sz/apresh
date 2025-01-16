use anyhow::Context;
use candid::CandidType;
use hex::FromHex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;

use super::info::ShipmentInfo;

pub enum ShipmentActions<'a, ActorId> {
    Buy(ActorId),
    MarkDelivered {
        secret_key: Option<&'a str>,
        caller: ActorId,
    },
}

impl<ActorId> InternalShipment<ActorId>
where
    ActorId: Copy,
    ActorId: Eq,
{
    pub fn action(&mut self, op: ShipmentActions<ActorId>) -> anyhow::Result<()> {
        match op {
            ShipmentActions::Buy(carrier_id) => self.buy(carrier_id),
            ShipmentActions::MarkDelivered { secret_key, caller } => {
                self.finalize(secret_key, caller)
            }
        }
    }

    fn validate_secret(&self, secret: &str) -> anyhow::Result<()> {
        let hex = Vec::from_hex(self.hashed_secret.clone()).context("invalid hex")?;

        let mut hasher = Sha256::new();
        hasher.update(secret);
        let result = hasher.finalize();

        if result[..] == hex {
            Ok(())
        } else {
            Err(anyhow::anyhow!("secret verification failed"))
        }
    }

    fn finalize(&mut self, secret_key: Option<&str>, caller: ActorId) -> anyhow::Result<()> {
        if self.status != ShipmentStatus::InTransit {
            return Err(anyhow::anyhow!("shipment is not ready to be finalized"));
        }

        if caller != self.shipper {
            let secret_key = secret_key.ok_or(anyhow::anyhow!("missing secret"))?;

            self.validate_secret(secret_key)?;
        }

        self.status = ShipmentStatus::DeliveryCompleted;

        Ok(())
    }

    fn assign_carrier(&mut self, carrier_id: ActorId) {
        self.carrier = Some(carrier_id);
        self.status = ShipmentStatus::InTransit;
    }

    fn buy(&mut self, carrier_id: ActorId) -> anyhow::Result<()> {
        if self.status != ShipmentStatus::Pending {
            return Err(anyhow::anyhow!(
                "shipment is not created, invalid operation"
            ));
        }

        self.assign_carrier(carrier_id);

        Ok(())
    }
}

pub type ShipmentId = u64;

/// Shipment status
#[derive(Deserialize, Serialize, Debug, Clone, Copy, CandidType, PartialEq, Eq, Default)]
pub enum ShipmentStatus {
    /// Shipment is created but not bought
    #[default]
    Pending,
    /// Shipment is bought by carrier
    Bought,
    /// Shipment has pickup scheduled
    PickupScheduled,
    /// Shipment has been picked up
    PickupCompleted,
    /// Shipment is in transit
    InTransit,
    /// Shipment has delivery scheduled
    DeliveryScheduled,
    /// Shipment has been delivered
    DeliveryCompleted,
    /// Shipment has been cancelled
    Cancelled,
}

impl ShipmentStatus {
    pub fn is_finished(&self) -> bool {
        match self {
            ShipmentStatus::Cancelled | ShipmentStatus::DeliveryCompleted => true,
            _ => false,
        }
    }
}

// Shipment, but without principals, so JSON-able
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InternalShipment<ActorId> {
    /// Shipment id
    id: ShipmentId,
    /// Shipment name
    name: String,
    /// Hashed secret, used to verify the secret in delivery
    hashed_secret: String,
    /// Shipment info
    info: ShipmentInfo,
    /// Shipment status
    status: ShipmentStatus,
    /// Encrypted message from shipper to carrier, could be used to send contact information
    message: Option<String>,
    /// Carrier id
    carrier: Option<ActorId>, // TODO: I think we should use some internal id instead of principal here
    /// Shipper id
    shipper: ActorId,
    /// Shipment creation timestamp
    created_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct Shipment {
    id: ShipmentId,
    name: String,
    hashed_secret: String,
    info: ShipmentInfo,
    status: ShipmentStatus,
    message: Option<String>,
    carrier: Option<String>,
    shipper: String,
    created_at: u64,
}

impl<ActorId> From<&InternalShipment<ActorId>> for Shipment
where
    ActorId: ToString,
    ActorId: Copy,
{
    fn from(shipment: &InternalShipment<ActorId>) -> Self {
        Self {
            id: shipment.id,
            name: shipment.name.clone(),
            hashed_secret: shipment.hashed_secret.clone(),
            info: shipment.info.clone(),
            status: shipment.status,
            message: shipment.message.clone(),
            carrier: shipment.carrier.map(|id| id.to_string()),
            shipper: shipment.shipper.to_string(),
            created_at: shipment.created_at,
        }
    }
}

impl<ActorId> InternalShipment<ActorId>
where
    ActorId: Copy,
{
    pub fn new(
        timestamp: u64,
        shipper: ActorId,
        id: ShipmentId,
        hashed_secret: &str,
        name: &str,
        info: ShipmentInfo,
    ) -> Self {
        Self {
            id,
            info,
            name: name.to_string(),
            message: None,
            hashed_secret: hashed_secret.to_string(),
            status: ShipmentStatus::default(),
            carrier: None,
            shipper,
            created_at: timestamp,
        }
    }

    pub fn attach_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn status(&self) -> &ShipmentStatus {
        &self.status
    }

    pub fn shipper_id(&self) -> ActorId {
        self.shipper
    }

    pub fn carrier_id(&self) -> Option<ActorId> {
        self.carrier
    }

    pub fn _id(&self) -> ShipmentId {
        self.id
    }

    pub fn _name(&self) -> &str {
        &self.name
    }

    pub fn info(&self) -> &ShipmentInfo {
        &self.info
    }
}
