use candid::{CandidType, Principal};
use derive_deref::{Deref, DerefMut};

pub mod actors;
mod errors;
pub mod models;
pub mod operations;
pub mod state;
pub mod utils;
pub use errors::{Error, Result};

#[derive(
    CandidType,
    Deref,
    DerefMut,
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
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
