use sha2::{Digest, Sha256};

pub fn sha256_string(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    format!("{digest:x}")
}

pub fn sha256_bytes(input: &[u8]) -> String {
    let digest = Sha256::digest(input);
    format!("{digest:x}")
}
