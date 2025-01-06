use candid::Principal;
use serde::{Deserialize, Serialize};
use actor_derive::IsActor;
use crate::models::shipment::ShipmentId;
use super::{base::ActorBase, ActorRole, Actor};

pub type ShipperId = Principal;

#[derive(Debug, Clone, Deserialize, Serialize, IsActor)]
pub struct Shipper {
    base: ActorBase,
}

impl Shipper {
    pub fn new(id: ShipperId, name: &str) -> Self {
        Self { base: ActorBase::new(id, name.to_string()) }
    }
}
