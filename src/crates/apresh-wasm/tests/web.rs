//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm::KeyPair;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let alice = KeyPair::generate();
    let bob = KeyPair::generate();

    let alice_serialized = alice.secret_key();
    let alice_deserialized = KeyPair::from(&alice_serialized);
    assert_eq!(alice.public_key(), alice_deserialized.public_key());

    let message = b"Hello, world!";
    let encrypted = alice.encrypt_for(&bob.public_key(), message);

    let decrypted = bob.decrypt(&encrypted);
    assert_eq!(message.to_vec(), decrypted.message());
    assert_eq!(false, decrypted.is_author());

    let self_decrypted = alice.decrypt(&encrypted);
    assert_eq!(message.to_vec(), self_decrypted.message());
    assert_eq!(true, self_decrypted.is_author());
}
