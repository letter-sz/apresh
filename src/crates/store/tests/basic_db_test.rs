use apresh_derive::DeriveKey;
use apresh_store::Record;
use serde::{Deserialize, Serialize};

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
struct ReferenceableRecord {
    id: u32,
    name: (u8, u16, u32, u64),
}

#[test]
fn test_db() {
    // Create a record that will be inserted into the database
    let record = TestRecord {
        id: 1,
        name: "test".to_string(),
    };

    // Test that record can be inserted and retrieved
    assert_eq!(record.key(), 1);
    record.set();
    let record = TestRecord::get(1).unwrap();
    assert_eq!(record.name, "test");

    // Create another record with the same key.
    let another_record = ReferenceableRecord {
        id: 1,
        name: (1, 2, 3, 4),
    };

    // Test that the record can be inserted and retrieved
    assert_eq!(another_record.key(), ReferenceableRecordKey(1));
    another_record.set();

    // Test that the record can be as usual
    let another_record = ReferenceableRecord::get(ReferenceableRecordKey(1)).unwrap();
    assert_eq!(another_record.name, (1, 2, 3, 4));

    // Test that the record can be retrieved by the foreign key
    let another_record = ReferenceableRecordKey(1).get().unwrap();
    assert_eq!(another_record.name, (1, 2, 3, 4));
}
