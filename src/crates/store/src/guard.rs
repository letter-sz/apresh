use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

use crate::{DatabaseKeyable, Writable};

#[must_use]
pub struct Guard<T: Writable> {
    key: Vec<u8>,
    value: Option<T>,
    unchanged: bool,
}

impl<T: Writable> Guard<T> {
    pub fn new_with_key(key: Vec<u8>, value: T) -> Self {
        Self {
            key,
            value: Some(value),
            unchanged: true,
        }
    }

    pub fn new(value: T) -> Self
    where
        T: DatabaseKeyable,
    {
        Self {
            key: value.raw_key(),
            value: Some(value),
            unchanged: true,
        }
    }

    pub fn commit(mut self) {
        self.unchanged = true;
        // TODO: committing can be skipped if the value wasn't ever mutably borrowed
        self.value.take().unwrap().commit(self.key.clone());
    }

    pub fn revert(mut self) {
        self.unchanged = true;
    }
}

impl<T: Writable> Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let Some(value) = &self.value else {
            unreachable!("Guarded value is always some, until right before drop");
        };
        value
    }
}

impl<T: Writable> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.unchanged = false;
        let Some(value) = &mut self.value else {
            unreachable!("Guarded value is always some, until right before drop");
        };
        value
    }
}

impl<T: Writable> Drop for Guard<T> {
    fn drop(&mut self) {
        if !self.unchanged {
            panic!("Guard was neither committed nor reverted");
        }
    }
}

impl<T: Writable + PartialEq> PartialEq for Guard<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Writable + Eq> Eq for Guard<T> {}

impl<T: Writable + Debug> Debug for Guard<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}
