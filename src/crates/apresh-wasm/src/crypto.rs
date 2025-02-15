use wasm_bindgen::prelude::*;

use apresh_crypto::{PublicKey, StaticSecret};

use crate::utils::randomness;

#[wasm_bindgen]
pub struct MessageAndKey {
    message: Vec<u8>,
    secret: StaticSecret,
}

#[wasm_bindgen]
impl MessageAndKey {
    pub fn message(&self) -> Vec<u8> {
        self.message.clone()
    }

    pub fn secret(&self) -> Vec<u8> {
        self.secret.to_bytes().to_vec()
    }
}

#[wasm_bindgen]
pub fn encrypt_for(public_key: &[u8], message: &[u8]) -> MessageAndKey {
    let (message, secret) =
        apresh_crypto::encrypt_for(public_key, message, randomness(), randomness()).unwrap();

    MessageAndKey { message, secret }
}

#[wasm_bindgen]
pub struct KeyPair {
    public_key: PublicKey,
    secret_key: StaticSecret,
}

#[wasm_bindgen]
impl KeyPair {
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.to_bytes().to_vec()
    }

    pub fn secret_key(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }
}

#[wasm_bindgen]
pub fn keygen() -> KeyPair {
    let (public_key, secret_key) = apresh_crypto::generate(randomness());
    KeyPair {
        public_key,
        secret_key,
    }
}

#[wasm_bindgen]
pub fn extract_public_key(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    apresh_crypto::extract(secret_key, message)
}
