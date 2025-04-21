use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

use crate::Record;

#[must_use]
pub struct Guard<T: Record>(T, bool);

impl<T: Record> Guard<T> {
    pub fn new(value: T) -> Self {
        Self(value, false)
    }

    pub fn consume(self) {}

    pub fn revert(mut self) {
        self.1 = false;
    }
}

impl<T: Record> Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Record> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1 = true;
        &mut self.0
    }
}

impl<T: Record> Drop for Guard<T> {
    fn drop(&mut self) {
        if self.1 {
            self.0.set_by_ref();
        }
    }
}

impl<T: Record + PartialEq> PartialEq for Guard<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Record + Eq> Eq for Guard<T> {}

impl<T: Record + Debug> Debug for Guard<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
