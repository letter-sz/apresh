use apresh_derive::DeriveKey;
use apresh_store::DatabaseKeyable;
use apresh_store::Record;
use serde::{Deserialize, Serialize};

#[derive(DeriveKey, Serialize, Deserialize)]
#[table(1)]
struct User {
    id: u32,
    name: [u8; 16],
}

#[derive(DeriveKey, Serialize, Deserialize, Debug, PartialEq)]
#[table(2)]
struct Post {
    id: u32,
    user_id: UserKey,
    title: String,
}

#[test]
fn test_foreign_key() {
    let user = User {
        id: 1,
        name: [42; 16],
    };
    let user_key = user.key();
    user.set();

    Post {
        id: 1,
        user_id: user_key,
        title: "Hello, world!".to_string(),
    }
    .set();

    // Get correct post
    let post_key = PostKey(1);
    let post = Post::get(post_key).unwrap();
    assert_eq!(post, *post_key.get().unwrap());
    assert_eq!(post.user_id, UserKey(1));
    assert_eq!(post.title, "Hello, world!".to_string());

    // Get correct user
    let user = post.user_id.get().unwrap();
    assert_eq!(user.id, 1);
    assert_eq!(user.name, [42; 16]);
}
