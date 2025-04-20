use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

pub const DB_MEMORY_ID: MemoryId = MemoryId::new(7);

pub fn get_memory(id: MemoryId) -> VirtualMemory<DefaultMemoryImpl> {
    MEMORY_MANAGER.with(|m| m.borrow().get(id))
}
thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub(crate) static DB_MEMORY: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(get_memory(DB_MEMORY_ID))
    );
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum StoreError {
    #[error("Store error: {0}")]
    Other(String),
}

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

#[test]
fn test_vec_ord() {
    assert!(vec![1, 2] < vec![1, 3]);
    assert!(vec![1] < vec![1, 2]);
    assert!(vec![1, 2] < vec![2]);
}
