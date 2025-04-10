use crate::DB_MEMORY;

pub trait Record: serde::Serialize + serde::de::DeserializeOwned {
    const SCOPE: u8;
    type Key: serde::Serialize + serde::de::DeserializeOwned;

    fn key(&self) -> Self::Key;

    fn set(self) {
        let key = self.key();
        let mut key = bcs::to_bytes(&key).unwrap();
        key.insert(0, Self::SCOPE);
        DB_MEMORY.with_borrow_mut(|db| {
            let value = bcs::to_bytes(&self).unwrap();
            db.insert(key, value);
        })
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
            if let Some(start) = start {
                if let Some(end) = end {
                    db.keys_range(start..end)
                } else {
                    db.keys_range(start..)
                }
            } else if let Some(end) = end {
                db.keys_range(..end)
            } else {
                db.keys_range(..)
            }
            .map(|key| bcs::from_bytes::<Self::Key>(&key[1..]).unwrap())
            .collect()
        })
    }
}
