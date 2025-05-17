use crate::{Guard, StoreError, DB_MEMORY};

pub trait Writable: serde::Serialize + serde::de::DeserializeOwned {
    fn commit(self, key: Vec<u8>) {
        DB_MEMORY.with_borrow_mut(|db| {
            let value = bcs::to_bytes(&self).unwrap();
            db.insert(key, value);
        })
    }
}

pub trait DatabaseKeyable: Writable {
    const SCOPE: u8;
    type Key: serde::Serialize + serde::de::DeserializeOwned + Clone;

    fn key(&self) -> Self::Key;
    fn raw_key(&self) -> Vec<u8> {
        let mut key = bcs::to_bytes(&self.key()).unwrap();
        key.insert(0, Self::SCOPE);
        key
    }
}

pub trait Record: DatabaseKeyable {
    fn set(self) {
        let key = self.raw_key();
        self.commit(key);
    }

    fn get(key: Self::Key) -> Option<Self> {
        let mut key = bcs::to_bytes(&key).unwrap();
        key.insert(0, Self::SCOPE);
        DB_MEMORY.with_borrow(|db| {
            let record = db.get(&key)?;
            let record_vec = record.to_vec();
            let value = bcs::from_bytes::<Self>(&record_vec).unwrap();
            Some(value)
        })
    }

    fn get_guard(key: Self::Key) -> Option<Guard<Self>> {
        Self::get(key.clone()).map(|value| Guard::new(value))
    }

    fn delete(key: Self::Key) {
        let mut key = bcs::to_bytes(&key).unwrap();
        key.insert(0, Self::SCOPE);
        DB_MEMORY.with_borrow_mut(|db| {
            db.remove(&key);
        })
    }

    fn range_scan(start: Option<Self::Key>, end: Option<Self::Key>) -> Vec<Self::Key> {
        let start = start.map(|key| {
            let mut key = bcs::to_bytes(&key).unwrap();
            key.insert(0, Self::SCOPE);
            key
        });
        let end = end.map(|key| {
            let mut key = bcs::to_bytes(&key).unwrap();
            key.insert(0, Self::SCOPE);
            key
        });

        DB_MEMORY.with_borrow(|db| {
            let range_iterator = if let Some(start) = start {
                if let Some(end) = end {
                    db.keys_range(start..end)
                } else {
                    db.keys_range(start..vec![Self::SCOPE + 1])
                }
            } else if let Some(end) = end {
                db.keys_range(vec![Self::SCOPE]..end)
            } else {
                db.keys_range(vec![Self::SCOPE]..vec![Self::SCOPE + 1])
            };

            let keys = range_iterator.collect::<Vec<_>>();
            dbg!(&keys);

            keys.clone()
                .into_iter()
                .map(|key| {
                    bcs::from_bytes::<Self::Key>(&key[1..])
                        .map_err(|_| {
                            StoreError::Other(format!(
                                "Failed to deserialize key: {:?} {:?}",
                                key, keys
                            ))
                        })
                        .unwrap()
                })
                .collect()
        })
    }
}
