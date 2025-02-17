mod crypto;
mod utils;

use wasm_bindgen::prelude::*;

use apresh_qr::{generate, QrCodeOptions};
use engine::utils::hash_secret;

use utils::set_panic_hook;

pub use crypto::*;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn get_secret_hash(s: &str) -> Vec<u8> {
    hash_secret(s.as_bytes())
}

#[wasm_bindgen]
pub fn generate_qr(link: String, size: usize) -> Result<Vec<u8>, String> {
    generate(QrCodeOptions {
        gradient: false,
        link,
        size,
        transparent: false,
    })
    .map_err(|e| e.to_string())
}
