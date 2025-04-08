use apresh_derive::DeriveKey;
use serde::{Deserialize, Serialize};
use state::db::{get_record, set_record, Record};

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
    set_record(record);
    let record = get_record::<TestRecord>(1).unwrap();
    assert_eq!(record.name, "test");

    let another_record = AnotherRecord {
        id: 1,
        name: (1, 2, 3, 4),
    };
    set_record(another_record);
    let another_record = get_record::<AnotherRecord>(1).unwrap();
    assert_eq!(another_record.name, (1, 2, 3, 4));
}
