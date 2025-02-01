use sha2::Digest;
use sha2::Sha256;

pub fn hash_secret(secret: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.finalize().to_vec()
}
