use sha2::{Sha256, Digest};

pub struct Helpers;

impl Helpers {
    pub fn string_to_sha256(value: &str) -> String {
        let mut hashes = Sha256::new();
        hashes.update(value.as_bytes());
        let finalized_hash = hashes.finalize();
        hex::encode(finalized_hash)
    }
}