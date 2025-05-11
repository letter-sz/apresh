use apresh_derive::DeriveKey;
use apresh_store::Record;
use entrypoint::entrypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, DeriveKey, Serialize, Deserialize)]
struct User {
    id: u64,
    pub name: String,
}

#[entrypoint]
fn get_name(#[key] user: User) -> String {
    user.name.clone()
}

#[entrypoint]
fn set_name(#[key] user: User, name: String) {
    user.name = name;
}

#[test]
fn test_entrypoint() {
    let user = User {
        id: 1,
        name: "John".to_string(),
    };
    let key = user.key();
    user.set();
    let result = get_name(key);
    assert_eq!(result, "John");
}

#[test]
fn test_entrypoint_mut() {
    let user = User {
        id: 1,
        name: "John".to_string(),
    };
    let key = user.key();
    user.set();
    set_name(key, "Jane".to_string());
    let result = get_name(key);
    assert_eq!(result, "Jane");
}
