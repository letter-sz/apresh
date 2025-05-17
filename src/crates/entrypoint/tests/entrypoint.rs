use apresh_derive::DeriveKey;
use apresh_store::{DatabaseKeyable, Record};
use entrypoint::entrypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, DeriveKey, Serialize, Deserialize)]
struct User {
    id: u64,
    pub name: String,
}

#[entrypoint]
fn get_name(#[key] user: User) -> Result<String, String> {
    Ok(user.name.clone())
}

#[entrypoint]
fn set_name(#[key] user: User, name: String) -> Result<(), String> {
    user.name = name;
    Ok(())
}

#[entrypoint]
fn maybe_set_name(#[key] user: User, name: String, fail: bool) -> Result<(), String> {
    user.name = name;

    if fail {
        return Err(String::from("Fail"));
    }

    Ok(())
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
    assert_eq!(result, Ok("John".to_string()));
}

#[test]
fn test_entrypoint_mut() {
    let user = User {
        id: 1,
        name: "John".to_string(),
    };
    let key = user.key();
    user.set();
    set_name(key, "Jane".to_string()).unwrap();
    let result = get_name(key);
    assert_eq!(result, Ok("Jane".to_string()));
}

#[test]
fn test_entrypoint_mut_failing() {
    let user = User {
        id: 1,
        name: "John".to_string(),
    };
    let key = user.key();
    user.set();

    // Set without fail, should change
    maybe_set_name(key, "Jane".to_string(), false).unwrap();
    let result = get_name(key);
    assert_eq!(result, Ok("Jane".to_string()));

    // Set with fail, should not change
    let r = maybe_set_name(key, "James".to_string(), true);
    assert_eq!(r, Err("Fail".to_string()));
    let result = get_name(key);
    assert_eq!(result, Ok("Jane".to_string()));
}
