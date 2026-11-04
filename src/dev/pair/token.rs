//! Random tokens, hashing, and constant-time comparison.

use sha2::{Digest, Sha256};

/// `bytes` random bytes from the OS, as lowercase hex.
pub(super) fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    getrandom::fill(&mut buf).expect("the OS random source is available");
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

/// The SHA-256 of `input` as lowercase hex.
pub(super) fn sha256_hex(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Compares two byte strings without stopping at the first difference.
pub(super) fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Percent-encodes everything but the RFC 3986 unreserved characters.
pub(super) fn encode(text: &str) -> String {
    let keep = |b: &u8| b.is_ascii_alphanumeric() || b"-._~".contains(b);
    text.bytes()
        .map(|b| {
            if keep(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
