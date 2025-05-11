use apresh_store::Record;
use apresh_derive::DeriveKey;
use entrypoint::entrypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, DeriveKey, Serialize, Deserialize)]
struct User {
    id: u64,
    pub name: String,
}


#[entrypoint]
fn my_function(#[key] user: User) -> String {
    user.name.clone()
}

#[test]
fn test_entrypoint() {
    let user = User { id: 1, name: "John".to_string() };
    let key = user.key();
    user.set();
    let result = my_function(key);
    assert_eq!(result, "John");
}
