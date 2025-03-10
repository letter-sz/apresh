use crate::actors::Actor;
use candid::{CandidType, Principal};
use derive_deref::{Deref, DerefMut};
use serde::Deserialize;
use std::collections::HashMap;

#[cfg(feature = "icp")]
#[derive(CandidType)]
#[derive(Deserialize, Deref, DerefMut)]
pub struct ActorCollection<T: Actor> {
    inner: HashMap<Principal, T>,
}

impl<T: Actor> Default for ActorCollection<T> {
    fn default() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }
}

impl<T: Actor> ActorCollection<T> {
    pub fn get(&self, id: &Principal) -> Option<&T> {
        self.inner.get(id)
    }

    pub fn create(&mut self, actor: T) -> &mut T {
        self.inner.entry(actor.id().0).or_insert(actor)
    }

    pub fn get_mut(&mut self, id: &Principal) -> Option<&mut T> {
        self.inner.get_mut(id)
    }

    #[cfg(feature = "icp")]
    pub fn insert_multiple(&mut self, actors: Vec<T>) {
        for actor in actors {
            self.inner.insert(actor.id().0, actor);
        }
    }
}
