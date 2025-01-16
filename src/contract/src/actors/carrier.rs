use engine::models::shipment::ShipmentId;

use super::{base::ActorBase, Actor, ActorRole};
use actor_derive::IsActor;
use candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, IsActor)]
pub struct Carrier {
    base: ActorBase,
}

impl Carrier {
    pub fn new(id: Principal, name: &str) -> Self {
        Self {
            base: ActorBase::new(id, name.to_string()),
        }
    }

    pub fn id(&self) -> Principal {
        self.base.id()
    }

    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.add_shipment(shipment_id);
    }
}
