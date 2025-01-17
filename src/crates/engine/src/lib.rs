use derive_deref::{Deref, DerefMut};

pub mod actors;
mod errors;
pub mod models;
pub mod operations;
pub mod state;

pub use errors::{Error, Result};

#[derive(
    Deref, DerefMut, serde::Deserialize, serde::Serialize, Debug, Clone, Copy, Eq, PartialEq,
)]
pub struct ActorId(pub candid::Principal);
