pub mod actor;
pub mod channel;
pub mod error;
pub mod shipment;

use apresh_crypto::hash_secret;
use apresh_derive::{DeriveKey, IsActor};
use candid::{CandidType, Principal};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

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

pub type Result<T> = std::result::Result<T, ShipmentError>;

#[derive(thiserror::Error, Debug)]
pub enum ShipmentError {
    #[error("Shipment not found")]
    ShipmentNotFound,
    #[error("Carrier not found")]
    CarrierNotFound,
    #[error("Carrier not set")]
    CarrierNotSet,
    #[error("Shipper not found")]
    ShipperNotFound,
    #[error("Carrier already set")]
    CarrierAlreadySet,
    #[error("Shipment already bought")]
    ShipmentNotReadyToBeCanceled,
    #[error("Shipment not ready to be finalized")]
    ShipmentNotReadyToBeFinalized,
    #[error("Not authorized to finalize shipment")]
    NotAuthorizedToFinalizeShipment,
    #[error("Secret key is invalid")]
    SecretKeyIsInvalid,
    #[error("Secret key not present")]
    SecretKeyNotPresent,
    #[error("Not authorized as carrier")]
    NotAuthorizedAsCarrier,
    #[error("Not authorized as shipper")]
    NotAuthorizedAsShipper,
    #[error("Not authorized as neither carrier nor shipper")]
    NotAuthorizedAsNeitherCarrierNorShipper,
    #[error("Message too long")]
    MessageTooLong,
    #[error("Shipment limit reached")]
    ShipmentLimitReached,
    #[error("Shipment cannot be bought")]
    ShipmentCannotBeBought,
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

pub type Message = Vec<u8>;
pub type ChannelKey = Vec<u8>;

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    host_key: ChannelKey,
    messages: Vec<Message>,
    guest_keys: Vec<ChannelKey>, // Guests understand host but not each other
}

impl Channel {
    pub fn new(host_key: ChannelKey) -> Self {
        Self {
            host_key,
            messages: Vec::new(),
            guest_keys: Vec::new(),
        }
    }

    pub fn add_guest(&mut self, guest_key: ChannelKey) {
        self.guest_keys.push(guest_key);
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(
    Deref, DerefMut, serde::Deserialize, serde::Serialize, Debug, Clone, Copy, Eq, PartialEq,
)]
pub struct ActorId(pub candid::Principal);

impl From<Principal> for ActorId {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<ActorId> for Principal {
    fn from(actor_id: ActorId) -> Self {
        actor_id.0
    }
}

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(DeriveKey, Debug, Clone, Deserialize, Serialize, IsActor, Eq, PartialEq)]
#[table(11)]
pub struct Shipper {
    id: ActorId,
    base: ActorBase,
}

impl Shipper {
    pub fn new(id: ActorId, name: &str) -> Self {
        Self {
            id,
            base: ActorBase::new(id.0, name.to_string()),
        }
    }

    pub fn id(&self) -> ActorId {
        self.base.id()
    }
}

/// Base structure that provides common functionality for all actors in the system.
///
/// The `ActorBase` serves as the foundation for all actor types (Shipper, Carrier, etc.)
/// by implementing common properties and behaviors. This structure is designed to be
/// embedded within specific actor implementations
///
/// # Properties
/// - `id`: Unique identifier for the actor (Principal)
/// - `name`: Human-readable name for the actor
/// - `active_shipments`: List of shipments currently in progress
/// - `shipments_history`: Archive of completed or cancelled shipments
///
#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ActorBase {
    /// The version of the actor.
    /// This is used to ensure backwards compatibility with older versions when migrating data.
    version: ActorVersion,

    /// The unique principal identifier of the actor.
    /// This is used for authentication and authorization throughout the system.
    id: ActorId,

    /// The human-readable name of the actor.
    /// This should be a meaningful identifier for the actor in the system.
    name: String,

    /// List of shipments that are currently active/in-progress.
    /// A shipment is considered active when it has been created but not yet
    /// delivered or cancelled.
    active_shipments: Vec<ShipmentId>,

    /// Historical record of all completed or cancelled shipments.
    /// This list maintains the full history of an actor's involvement
    /// in various shipments, regardless of their final status.
    shipments_history: Vec<ShipmentId>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ActorVersion {
    Invalid = 0,
    V1 = 1,
}

impl ActorBase {
    /// Creates a new ActorBase instance with the given principal ID and name.
    ///
    /// # Arguments
    /// * `id` - The Principal identifier for the actor
    /// * `name` - The display name for the actor
    ///
    /// # Returns
    /// A new ActorBase instance with empty shipment lists
    pub fn new(id: Principal, name: String) -> Self {
        Self {
            version: ActorVersion::V1,
            id: ActorId(id),
            name,
            active_shipments: vec![],
            shipments_history: vec![],
        }
    }

    /// Returns the Principal identifier of the actor.
    pub fn id(&self) -> ActorId {
        self.id
    }

    /// Returns the display name of the actor.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a slice of all active shipment IDs.
    pub fn get_active_shipments(&self) -> &[ShipmentId] {
        &self.active_shipments
    }

    /// Returns a slice of all historical shipment IDs.
    pub fn get_shipments_history(&self) -> &[ShipmentId] {
        &self.shipments_history
    }

    /// Adds a new shipment to the actor's active shipments list.
    ///
    /// This method is called when the actor becomes involved with a new shipment,
    /// either by creating it or accepting it for delivery.
    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.push(shipment_id);
    }

    /// Moves a shipment from active to history.
    ///
    /// This method is called when a shipment is completed or cancelled. It:
    /// 1. Removes the shipment from the active list
    /// 2. Adds it to the history list
    ///
    /// TODO: Add validation to ensure the shipment exists in active_shipments
    pub fn archive_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.retain(|&x| x != shipment_id);
        self.shipments_history.push(shipment_id);
    }
}

/// The role of the actor in the shipping system.
/// Each actor has a specific role that determines their capabilities and responsibilities
/// within the system. Currently, there are two primary roles:
/// - Shipper: Initiates shipments and manages shipping requests
/// - Carrier: Handles the transportation and delivery of shipments
pub enum ActorRole {
    /// The shipper actor who initiates and manages shipments.
    /// Shippers can:
    /// - Create new shipments
    /// - Track their active shipments
    /// - View shipment history
    Shipper,
    /// The carrier actor who handles transportation.
    /// Carriers can:
    /// - Accept shipments for delivery
    /// - Update shipment status
    /// - Manage their delivery queue
    Carrier,
}

#[allow(dead_code)]
/// Core trait that defines the behavior and properties of all actors in the system.
///
/// An actor represents a participant in the shipping system, such as a shipper or carrier.
/// Each actor has a unique identity (Principal), a name, and maintains lists of their
/// active and completed shipments.
///
/// # Implementation
/// This trait is typically implemented using the `IsActor` derive macro, which requires
/// the implementing struct to have a `base: ActorBase` field.
///
/// # Example
/// ```rust
/// use engine::actors::{base::ActorBase, Actor, ActorRole};
/// use engine::{models::shipment::ShipmentId, ActorId};
///
/// #[derive(apresh_derive::IsActor)]
/// struct Carrier {
///     base: ActorBase,
/// }
/// ```
pub trait Actor {
    /// Returns the unique Principal identifier of the actor.
    /// This is used for authentication and tracking ownership of actions.
    fn id(&self) -> ActorId;

    /// Returns the human-readable name of the actor.
    /// This name is used for display purposes and user interaction.
    fn name(&self) -> &str;

    /// Returns the role of the actor (Shipper or Carrier).
    /// The role determines what operations the actor can perform in the system.
    fn role(&self) -> ActorRole;

    /// Adds a new shipment to the actor's active shipments list.
    /// This is called when an actor becomes involved with a new shipment,
    /// either by creating it (Shipper) or accepting it (Carrier).
    fn add_shipment(&mut self, shipment_id: ShipmentId);

    /// Moves a shipment from active to history when it's completed or cancelled.
    /// This helps maintain a clear record of all shipments an actor has been involved with.
    fn archive_shipment(&mut self, shipment_id: ShipmentId);

    /// Returns a slice of all active shipment IDs associated with this actor.
    /// Active shipments are those that are in progress and not yet completed or cancelled.
    fn get_active_shipments(&self) -> &[ShipmentId];

    /// Returns a slice of all historical shipment IDs associated with this actor.
    /// This includes both successfully completed and cancelled shipments.
    fn get_shipments_history(&self) -> &[ShipmentId];
}

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(DeriveKey, Debug, Clone, Deserialize, Serialize, IsActor, Eq, PartialEq)]
#[table(12)]
pub struct Carrier {
    id: ActorId,
    base: ActorBase,
}

impl Carrier {
    pub fn new(id: ActorId, name: &str) -> Self {
        Self {
            id,
            base: ActorBase::new(id.0, name.to_string()),
        }
    }

    pub fn id(&self) -> ActorId {
        self.base.id()
    }

    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.add_shipment(shipment_id);
    }
}
