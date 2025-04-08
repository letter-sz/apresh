use apresh_derive::DeriveKey;
use serde::{Deserialize, Serialize};
use state::db::Record;

#[derive(Serialize, Deserialize)]
struct TestRecord {
    id: u64,
    name: String,
}

impl Record for TestRecord {
    const SCOPE: u8 = 0;
    type Key = u64;

    fn key(&self) -> Self::Key {
        self.id
    }
}

#[derive(DeriveKey, Serialize, Deserialize)]
#[table(9)]
struct AnotherRecord {
    id: u32,
    name: (u8, u16, u32, u64),
}

#[test]
fn test_db() {
    let record = TestRecord {
        id: 1,
        name: "test".to_string(),
    };
    record.set();
    let record = TestRecord::get(1).unwrap();
    assert_eq!(record.name, "test");

    let another_record = AnotherRecord {
        id: 1,
        name: (1, 2, 3, 4),
    };
    another_record.set();
    let another_record = AnotherRecord::get(1).unwrap();
    assert_eq!(another_record.name, (1, 2, 3, 4));
}
