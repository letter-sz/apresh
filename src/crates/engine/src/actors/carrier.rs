use crate::{models::shipment::ShipmentId, ActorId};

use super::{base::ActorBase, Actor, ActorRole};
use actor_derive::IsActor;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[cfg(feature = "icp")]
#[derive(CandidType)]
#[derive(Debug, Clone, Deserialize, Serialize, IsActor)]
pub struct Carrier {
    base: ActorBase,
}

impl Carrier {
    pub fn new(id: ActorId, name: &str) -> Self {
        Self {
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
