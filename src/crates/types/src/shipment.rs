use crate::{ActorId, Channel, ChannelKey, Message, Result, ShipmentError};
use apresh_crypto::hash_secret;
use apresh_derive::DeriveKey;
use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type ShipmentId = u64;

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

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ShipmentVersion {
    Invalid = 0,
    V1 = 1,
}

// Shipment, but without principals, so JSON-able
#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(DeriveKey, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[table(1)]
pub struct Shipment {
    /// Unique id for the shipment
    id: ShipmentId,
    /// Shipment version
    version: ShipmentVersion,
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

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrintableShipment {
    pub id: ShipmentId,
    pub name: String,
    pub hashed_secret: Vec<u8>,
    pub info: ShipmentInfo,
    pub status: ShipmentStatus,
    pub channel: Channel,
    pub carrier: Option<String>,
    pub shipper: String,
    pub created_at: u64,
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
            version: ShipmentVersion::V1,
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

impl Shipment {
    pub fn action(&mut self, op: ShipmentActions<ActorId>) -> Result<()> {
        match op {
            ShipmentActions::Buy(carrier_id) => self.buy(carrier_id),
            ShipmentActions::MarkDelivered { secret_key, caller } => {
                self.finalize(secret_key, caller)
            }
            ShipmentActions::Cancel { shipper } => self.cancel(shipper),
        }
    }

    fn cancel(&mut self, shipper: ActorId) -> Result<()> {
        if self.shipper != shipper {
            return Err(ShipmentError::NotAuthorizedAsShipper);
        }

        if self.status != ShipmentStatus::Pending || self.carrier.is_some() {
            return Err(ShipmentError::ShipmentNotReadyToBeCanceled);
        }

        self.status = ShipmentStatus::Cancelled;

        Ok(())
    }

    fn validate_secret(&self, secret: String) -> Result<()> {
        if hash_secret(secret.as_bytes()) != self.hashed_secret {
            return Err(ShipmentError::SecretKeyIsInvalid);
        }

        Ok(())
    }

    fn finalize(&mut self, secret_key: Option<String>, caller: ActorId) -> Result<()> {
        if self.status != ShipmentStatus::InTransit {
            return Err(ShipmentError::ShipmentNotReadyToBeFinalized);
        }

        if caller != self.shipper {
            let secret_key = secret_key.ok_or(ShipmentError::SecretKeyNotPresent)?;
            self.validate_secret(secret_key)?;
        }

        self.status = ShipmentStatus::DeliveryCompleted;

        Ok(())
    }

    fn assign_carrier(&mut self, carrier_id: ActorId) {
        self.carrier = Some(carrier_id);
        self.status = ShipmentStatus::InTransit;
    }

    fn buy(&mut self, carrier_id: ActorId) -> Result<()> {
        if self.status != ShipmentStatus::Pending {
            return Err(ShipmentError::ShipmentCannotBeBought);
        }

        if self.carrier.is_some() {
            return Err(ShipmentError::CarrierAlreadySet);
        }

        self.assign_carrier(carrier_id);

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType, PartialEq)]
pub struct ShipmentLocation {
    street: String,
    lat: i32,
    lng: i32,
}

impl ShipmentLocation {
    pub const COORDINATES_MULTIPLIER: i32 = 2_000_000_000i32 / 200; // Should give precision < 10cm

    pub const fn new(street: String, lat: f64, lng: f64) -> Self {
        let lat = (lat * Self::COORDINATES_MULTIPLIER as f64) as i32;
        let lng = (lng * Self::COORDINATES_MULTIPLIER as f64) as i32;

        Self { street, lat, lng }
    }
}

// This will be only used to compare to itself, never to compare locations, so it's ok to assume reflexivity.
impl Eq for ShipmentLocation {}

// SIZE CATEGORY
#[derive(Deserialize, Serialize, Debug, Clone, CandidType, PartialEq, Eq)]
pub enum SizeCategory {
    Envelope,
    Parcel {
        max_width: u64,
        max_height: u64,
        max_depth: u64,
    },
}

// INFO

#[derive(Deserialize, Serialize, Debug, Clone, CandidType, PartialEq, Eq)]
pub struct ShipmentInfo {
    /// Shipment value, used in insurance
    value: u64,
    /// Shipment price for a delivery
    price: u64,
    /// Shipment source location
    source: ShipmentLocation,
    /// Shipment destination location
    destination: ShipmentLocation,
    /// Shipment size category
    size_category: SizeCategory,
}

impl ShipmentInfo {
    pub fn price(&self) -> u64 {
        self.price
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn size_category(&self) -> &SizeCategory {
        &self.size_category
    }

    pub const fn new(
        value: u64,
        price: u64,
        source: ShipmentLocation,
        destination: ShipmentLocation,
        size_category: SizeCategory,
    ) -> Self {
        Self {
            value,
            price,
            source,
            destination,
            size_category,
        }
    }
}
