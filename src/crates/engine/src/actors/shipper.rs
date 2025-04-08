use crate::{models::shipment::ShipmentId, ActorId};

use super::{base::ActorBase, Actor, ActorRole};
use apresh_derive::IsActor;
use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Debug, Clone, Deserialize, Serialize, IsActor)]
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
