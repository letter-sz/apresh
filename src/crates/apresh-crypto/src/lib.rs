use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
pub use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid public key, expected 32 bytes")]
    InvalidPublicKey,
    #[error("Invalid secret key, expected 32 bytes")]
    InvalidSecretKey,
    #[error("Neither public key matches")]
    NeitherPublicKeyMatches,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    from: PublicKey,
    to: PublicKey,
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

impl Message {
    pub fn other_public_key(&self, public_key: &PublicKey) -> Result<PublicKey, Error> {
        if self.from == *public_key {
            Ok(self.to)
        } else if self.to == *public_key {
            Ok(self.from)
        } else {
            Err(Error::NeitherPublicKeyMatches)
        }
    }
}

fn slice_to_array<const N: usize>(slice: &[u8], error: Error) -> Result<[u8; N], Error> {
    if slice.len() != N {
        return Err(error);
    }
    let mut array = [0; N];
    array.copy_from_slice(slice);
    Ok(array)
}

pub fn encrypt_for(
    secret_key: &StaticSecret,
    public_key: &[u8],
    message: &[u8],
    random_nonce: [u8; 12],
) -> Result<Vec<u8>, Error> {
    let public_key = slice_to_array(public_key, Error::InvalidPublicKey)?;
    let public_key = PublicKey::from(public_key);

    let shared_secret = combine(public_key, secret_key);

    let ciphertext = encrypt(&shared_secret, message, &random_nonce);

    let message = Message {
        from: PublicKey::from(secret_key),
        to: public_key,
        nonce: random_nonce,
        ciphertext,
    };

    let message = bincode::serialize(&message).unwrap();

    Ok(message)
}

pub fn parse_keypair(secret_key: &[u8]) -> (PublicKey, StaticSecret) {
    let secret_key = slice_to_array(secret_key, Error::InvalidSecretKey).unwrap();
    let secret_key = StaticSecret::from(secret_key);
    let public_key = PublicKey::from(&secret_key);

    (public_key, secret_key)
}

pub fn extract(secret_key: &StaticSecret, message: &[u8]) -> (Vec<u8>, bool) {
    let message = bincode::deserialize::<Message>(message).unwrap();
    let public_key = PublicKey::from(secret_key);
    let other_public_key = message.other_public_key(&public_key).unwrap();
    let shared_secret = combine(other_public_key, secret_key);

    (
        decrypt(&shared_secret, &message.nonce, &message.ciphertext),
        message.from == public_key,
    )
}

pub fn parse_public_key(secret_key: &StaticSecret, message: &[u8]) -> Result<PublicKey, Error> {
    let message = bincode::deserialize::<Message>(message).unwrap();
    message.other_public_key(&PublicKey::from(secret_key))
}

pub fn generate(randomness: [u8; 32]) -> (PublicKey, StaticSecret) {
    let alice_secret = StaticSecret::from(randomness);
    let alice_public = PublicKey::from(&alice_secret);

    (alice_public, alice_secret)
}

fn combine(public_key: PublicKey, secret_key: &StaticSecret) -> [u8; 32] {
    secret_key.diffie_hellman(&public_key).to_bytes()
}

fn encrypt(shared_secret: &[u8; 32], message: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = shared_secret.into();

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    cipher.encrypt(nonce, message).unwrap()
}

fn decrypt(shared_secret: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = shared_secret.into();

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();

    plaintext.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (alice_public, alice_secret) = generate([0; 32]);
        let (bob_public, bob_secret) = generate([1; 32]);

        let shared_secret_alice = combine(bob_public, &alice_secret);
        let shared_secret_bob = combine(alice_public, &bob_secret);

        assert_eq!(shared_secret_alice, shared_secret_bob);
    }

    #[test]
    fn encrypt_decrypt() {
        let (_alice_public, alice_secret) = generate([0; 32]);
        let (bob_public, _bob_secret) = generate([1; 32]);
        let shared_secret_alice = combine(bob_public, &alice_secret);

        let message = b"Hello, Bob!";
        let nonce = [42; 12];

        let encrypted = encrypt(&shared_secret_alice, message, &nonce);
        let decrypted = decrypt(&shared_secret_alice, &nonce, &encrypted);

        assert_eq!(message.to_vec(), decrypted);
    }
}
