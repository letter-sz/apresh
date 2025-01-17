use crate::{models::shipment::ShipmentId, ActorId};

use super::{base::ActorBase, Actor, ActorRole};
use actor_derive::IsActor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, IsActor)]
pub struct Shipper {
    base: ActorBase,
}

impl Shipper {
    pub fn new(id: ActorId, name: &str) -> Self {
        Self {
            base: ActorBase::new(id.0, name.to_string()),
        }
    }

    pub fn id(&self) -> ActorId {
        self.base.id()
    }
}
