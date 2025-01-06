use crate::actors::carrier::CarrierId;
use crate::actors::shipper::ShipperId;
use anyhow::Context;
use candid::{CandidType, Principal};
use hex::FromHex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;

use super::info::ShipmentInfo;

pub enum ShipmentActions<'a> {
    Buy(CarrierId),
    MarkDelivered {
        secret_key: Option<&'a str>,
        caller: Principal,
    },
}

impl Shipment {
    pub fn action(&mut self, op: ShipmentActions) -> anyhow::Result<()> {
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
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("secret verification failed"));
        }
    }

    fn finalize(&mut self, secret_key: Option<&str>, caller: Principal) -> anyhow::Result<()> {
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

    fn assign_carrier(&mut self, carrier_id: CarrierId) {
        self.carrier = Some(carrier_id);
        self.status = ShipmentStatus::InTransit;
    }

    fn buy(&mut self, carrier_id: CarrierId) -> anyhow::Result<()> {
        if self.status != ShipmentStatus::Created {
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
#[derive(Deserialize, Serialize, Debug, Clone, CandidType, PartialEq, Eq, Default)]
pub enum ShipmentStatus {
    /// Shipment is created but not bought
    #[default]
    Created,
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

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct Shipment {
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
    carrier: Option<Principal>,
    /// Shipper id
    shipper: Principal,
    /// Shipment creation timestamp
    created_at: u64,
}

impl Shipment {
    pub fn new(
        timestamp: u64,
        shipper: ShipperId,
        id: ShipmentId,
        hashed_secret: &str,
        name: &str,
        info: ShipmentInfo,
    ) -> Self {
        let shipment = Self {
            id,
            info,
            name: name.to_string(),
            message: None,
            hashed_secret: hashed_secret.to_string(),
            status: ShipmentStatus::default(),
            carrier: None,
            shipper,
            created_at: timestamp,
        };

        shipment
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

    pub fn shipper_id(&self) -> Principal {
        self.shipper
    }

    pub fn carrier_id(&self) -> Option<Principal> {
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
