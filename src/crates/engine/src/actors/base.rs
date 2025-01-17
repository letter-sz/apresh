use crate::{models::shipment::ShipmentId, ActorId};

use candid::Principal;
use serde::{Deserialize, Serialize};

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
/// # Usage
/// ```rust
/// #[derive(IsActor)]
/// struct Carrier {
///     base: ActorBase,
/// }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActorBase {
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
