use std::collections::HashMap;
use candid::Principal;
use crate::actors::Actor;

pub struct ActorCollection<T: Actor> {
  inner: HashMap<Principal, T>,
}

impl<T: Actor> Default for ActorCollection<T> {
  fn default() -> Self {
      Self { inner: HashMap::default() }
  }
}

impl<T: Actor> ActorCollection<T> {
  pub fn get(&self, id: &Principal) -> Option<&T> {
    self.inner.get(id)
  }

  pub fn create(&mut self, actor: T) -> &mut T {
    self.inner.entry(actor.id()).or_insert(actor)
  }

  pub fn get_mut(&mut self, id: &Principal) -> Option<&mut T> {
    self.inner.get_mut(id)
  }
}
