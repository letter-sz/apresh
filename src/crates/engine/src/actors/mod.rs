use crate::{models::shipment::ShipmentId, ActorId};

pub mod base;
pub mod carrier;
pub mod shipper;

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
/// #[derive(IsActor)]
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
