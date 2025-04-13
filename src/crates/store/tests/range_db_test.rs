use apresh_derive::DeriveKey;
use serde::{Deserialize, Serialize};
use store::Record;

#[derive(DeriveKey, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[table(1)]
struct User {
    id: u32,
    name: [u8; 16],
}

#[test]
fn test_range_db() {
    let user = User {
        id: 1,
        name: [42; 16],
    };
    user.clone().set();

    let user2 = User {
        id: 2,
        name: [43; 16],
    };
    user2.clone().set();

    let user3 = User {
        id: 3,
        name: [44; 16],
    };
    user3.clone().set();

    let users = User::range_scan(None, None);
    assert_eq!(users.len(), 3);
    assert_eq!(users[0], UserKey(1));
    assert_eq!(users[1], UserKey(2));
    assert_eq!(users[2], UserKey(3));

    let users = User::range_scan(Some(UserKey(1)), None);
    assert_eq!(users.len(), 3);
    assert_eq!(users[0], UserKey(1));
    assert_eq!(users[1], UserKey(2));
    assert_eq!(users[2], UserKey(3));

    let users = User::range_scan(Some(UserKey(2)), None);
    assert_eq!(users.len(), 2);
    assert_eq!(users[0], UserKey(2));
    assert_eq!(users[1], UserKey(3));

    let users = User::range_scan(None, Some(UserKey(3)));
    assert_eq!(users.len(), 2);
    assert_eq!(users[0], UserKey(1));
    assert_eq!(users[1], UserKey(2));

    let users = User::range_scan(Some(UserKey(2)), Some(UserKey(3)));
    assert_eq!(users.len(), 1);
    assert_eq!(users[0], UserKey(2));
}

#[derive(DeriveKey, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[table(1)]
struct Post {
    id: (u32, u32),
}

#[test]
fn test_range_db_composite_key() {
    let post = Post { id: (1, 1) };
    post.clone().set();

    let post2 = Post { id: (1, 2) };
    post2.clone().set();

    let post3 = Post { id: (2, 1) };
    post3.clone().set();

    let posts = Post::range_scan(None, None);
    assert_eq!(posts.len(), 3);
    assert_eq!(posts[0], PostKey((1, 1)));
    assert_eq!(posts[1], PostKey((1, 2)));
    assert_eq!(posts[2], PostKey((2, 1)));

    let posts = Post::range_scan(Some(PostKey((1, 1))), Some(PostKey((2, 0))));
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0], PostKey((1, 1)));
    assert_eq!(posts[1], PostKey((1, 2)));
}
