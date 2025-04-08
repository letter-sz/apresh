use crate::DB_MEMORY;

pub trait Record: serde::Serialize + serde::de::DeserializeOwned {
    const SCOPE: u8;
    type Key: serde::Serialize + serde::de::DeserializeOwned;

    fn key(&self) -> Self::Key;
}

pub fn get_record<T: Record>(key: T::Key) -> Option<T> {
    let key = bcs::to_bytes(&key).unwrap();
    DB_MEMORY.with_borrow(|db| {
        let record = db.get(&key)?;
        let record_vec = record.to_vec();
        let value = bcs::from_bytes::<T>(&record_vec).unwrap();
        Some(value)
    })
}

pub fn set_record<T: Record>(record: T) {
    let key = record.key();
    let key = bcs::to_bytes(&key).unwrap();
    DB_MEMORY.with_borrow_mut(|db| {
        let value = bcs::to_bytes(&record).unwrap();
        db.insert(key, value);
    })
}
