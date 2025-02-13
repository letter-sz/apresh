mod utils;

use engine::utils::hash_secret;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn get_secret_hash(s: &str) -> Vec<u8> {
    hash_secret(s.as_bytes())
}
