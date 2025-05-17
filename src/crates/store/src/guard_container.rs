use crate::{Guard, Writable};

pub struct DoubleGuard<T: Writable> {
    guards: (Guard<T>, Guard<T>),
}

impl<T: Writable> DoubleGuard<T> {
    pub fn new(guard1: Guard<T>, guard2: Guard<T>) -> Self {
        Self {
            guards: (guard1, guard2),
        }
    }

    pub fn commit_all(self) {
        self.guards.0.commit();
        self.guards.1.commit();
    }

    pub fn revert_all(self) {
        self.guards.0.revert();
        self.guards.1.revert();
    }
}
