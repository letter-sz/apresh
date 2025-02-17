use wasm_bindgen::prelude::*;

use apresh_crypto::{PublicKey, StaticSecret};

use crate::utils::randomness;

#[wasm_bindgen]
pub struct KeyPair {
    public_key: PublicKey,
    secret_key: StaticSecret,
}

#[wasm_bindgen]
impl KeyPair {
    pub fn generate() -> Self {
        let (generated_public, generated_secret) = apresh_crypto::generate(randomness());
        Self {
            public_key: generated_public,
            secret_key: generated_secret,
        }
    }

    pub fn from(secret_key: &[u8]) -> Self {
        let (public_key, secret_key) = apresh_crypto::parse_keypair(secret_key);
        Self {
            public_key,
            secret_key,
        }
    }

    pub fn secret_key(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.to_bytes().to_vec()
    }

    pub fn encrypt_for(&self, public_key: &[u8], message: &[u8]) -> Vec<u8> {
        apresh_crypto::encrypt_for(&self.secret_key, public_key, message, randomness()).unwrap()
    }

    pub fn decrypt(&self, message: &[u8]) -> Vec<u8> {
        apresh_crypto::extract(&self.secret_key, message)
    }
}
