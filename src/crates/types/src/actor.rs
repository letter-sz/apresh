use crate::ShipmentId;
use apresh_derive::{DeriveKey, IsActor};
use candid::{CandidType, Principal};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

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
