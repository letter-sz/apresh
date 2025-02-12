use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::utils::hash_secret;
use crate::ActorId;

use super::info::ShipmentInfo;
use super::{Channel, ChannelKey, Message};

pub enum ShipmentActions<ActorId> {
    Buy(ActorId),
    MarkDelivered {
        secret_key: Option<String>,
        caller: ActorId,
    },
    Cancel {
        shipper: ActorId,
    },
}

impl Shipment {
    pub fn action(&mut self, op: ShipmentActions<ActorId>) -> crate::errors::Result<()> {
        match op {
            ShipmentActions::Buy(carrier_id) => self.buy(carrier_id),
            ShipmentActions::MarkDelivered { secret_key, caller } => {
                self.finalize(secret_key, caller)
            }
            ShipmentActions::Cancel { shipper } => self.cancel(shipper),
        }
    }

    fn cancel(&mut self, shipper: ActorId) -> crate::errors::Result<()> {
        if self.shipper != shipper {
            return Err(crate::errors::Error::NotAuthorizedAsShipper);
        }

        self.status = ShipmentStatus::Cancelled;

        Ok(())
    }

    fn validate_secret(&self, secret: String) -> crate::errors::Result<()> {
        if hash_secret(secret.as_bytes()) != self.hashed_secret {
            return Err(crate::errors::Error::SecretKeyIsInvalid);
        }

        Ok(())
    }

    fn finalize(
        &mut self,
        secret_key: Option<String>,
        caller: ActorId,
    ) -> crate::errors::Result<()> {
        if self.status != ShipmentStatus::InTransit {
            return Err(crate::errors::Error::ShipmentNotReadyToBeFinalized);
        }

        if caller != self.shipper {
            let secret_key = secret_key.ok_or(crate::errors::Error::SecretKeyNotPresent)?;
            self.validate_secret(secret_key)?;
        }

        self.status = ShipmentStatus::DeliveryCompleted;

        Ok(())
    }

    fn assign_carrier(&mut self, carrier_id: ActorId) {
        self.carrier = Some(carrier_id);
        self.status = ShipmentStatus::InTransit;
    }

    fn buy(&mut self, carrier_id: ActorId) -> crate::errors::Result<()> {
        if self.status != ShipmentStatus::Pending || self.carrier.is_some() {
            return Err(crate::errors::Error::CarrierAlreadySet);
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
        matches!(
            self,
            ShipmentStatus::Cancelled | ShipmentStatus::DeliveryCompleted
        )
    }
}

// Shipment, but without principals, so JSON-able
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shipment {
    /// Unique id for the shipment
    id: ShipmentId,
    /// A descriptive name for the shipment
    name: String,
    /// Hashed secret, used to verify the secret in delivery
    hashed_secret: Vec<u8>,
    /// Shipment info
    info: ShipmentInfo,
    /// Shipment status
    status: ShipmentStatus,
    /// Encrypted message from shipper to carrier, could be used to send contact information
    channel: Channel,
    /// Carrier id
    carrier: Option<ActorId>, // TODO: I think we should use some internal id instead of principal here
    /// Shipper id
    shipper: ActorId,
    /// Shipment creation timestamp
    created_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct PrintableShipment {
    id: ShipmentId,
    name: String,
    hashed_secret: Vec<u8>,
    info: ShipmentInfo,
    status: ShipmentStatus,
    channel: Channel,
    carrier: Option<String>,
    shipper: String,
    created_at: u64,
}

impl From<&Shipment> for PrintableShipment {
    fn from(shipment: &Shipment) -> Self {
        Self {
            id: shipment.id,
            name: shipment.name.clone(),
            hashed_secret: shipment.hashed_secret.clone(),
            info: shipment.info.clone(),
            status: shipment.status,
            channel: shipment.channel.clone(),
            carrier: shipment.carrier.map(|id| id.to_string()),
            shipper: shipment.shipper.to_string(),
            created_at: shipment.created_at,
        }
    }
}

impl Shipment {
    pub fn new(
        timestamp: u64,
        shipper: ActorId,
        id: ShipmentId,
        hashed_secret: Vec<u8>,
        channel_key: ChannelKey,
        name: &str,
        info: &ShipmentInfo,
    ) -> Self {
        Self {
            id,
            info: info.clone(),
            name: name.to_string(),
            channel: Channel::new(channel_key),
            hashed_secret: hashed_secret.to_vec(),
            status: ShipmentStatus::default(),
            carrier: None,
            shipper,
            created_at: timestamp,
        }
    }

    pub fn attach_message(&mut self, message: Message) {
        self.channel.push(message);
    }

    pub fn add_guest_to_channel(&mut self, guest_key: ChannelKey) {
        self.channel.add_guest(guest_key);
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
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

    pub fn id(&self) -> ShipmentId {
        self.id
    }

    pub fn _name(&self) -> &str {
        &self.name
    }

    pub fn info(&self) -> &ShipmentInfo {
        &self.info
    }
}
