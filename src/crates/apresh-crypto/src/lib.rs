use x25519_dalek::{StaticSecret, PublicKey};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

pub fn generate(randomness: [u8; 32]) -> (PublicKey, StaticSecret) {
    let alice_secret = StaticSecret::from(randomness);
    let alice_public = PublicKey::from(&alice_secret);

    (alice_public, alice_secret)
}

pub fn combine(public_key: PublicKey, secret_key: StaticSecret) -> [u8; 32] {
    secret_key.diffie_hellman(&public_key).to_bytes()
}

pub fn encrypt(shared_secret: &[u8; 32], message: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = shared_secret.into();

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    let ciphertext = cipher.encrypt(nonce, message).unwrap();

    // Nonce is not secret, so we can send it along with the ciphertext
    let mut output = Vec::with_capacity(12 + ciphertext.len());
    output.extend_from_slice(nonce.as_slice());
    output.extend(ciphertext);
    output
}

pub fn decrypt(shared_secret: &[u8; 32], ciphertext: &[u8]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = shared_secret.into();

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let plaintext = cipher.decrypt(nonce, &ciphertext[12..]).unwrap();

    plaintext.to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (alice_public, alice_secret) = generate([0; 32]);
        let (bob_public, bob_secret) = generate([1; 32]);

        let shared_secret_alice = combine(bob_public, alice_secret);
        let shared_secret_bob = combine(alice_public, bob_secret);

        assert_eq!(shared_secret_alice, shared_secret_bob);
    }

    #[test]
    fn encrypt_decrypt() {
        let (_alice_public, alice_secret) = generate([0; 32]);
        let (bob_public, _bob_secret) = generate([1; 32]);
        let shared_secret_alice = combine(bob_public, alice_secret);
        
        let message = b"Hello, Bob!";
        let nonce = [42; 12];

        let encrypted = encrypt(&shared_secret_alice, message, &nonce);
        let decrypted = decrypt(&shared_secret_alice, &encrypted);

        assert_eq!(message.to_vec(), decrypted);
    }
}
