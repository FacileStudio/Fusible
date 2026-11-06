use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;

use super::claim::start_pairing;
use super::lan::is_private;
use super::remote::is_lan_host;
use super::token::{ct_eq, encode, random_hex, sha256_hex};
use super::PairState;

pub(super) fn temp_store() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("fusible-pair-{}", random_hex(8)));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("devices.json")
}

#[test]
fn ct_eq_compares_content_and_length() {
    assert!(ct_eq(b"abc", b"abc"));
    assert!(!ct_eq(b"abc", b"abd"));
    assert!(!ct_eq(b"abc", b"abcd"));
    assert!(ct_eq(b"", b""));
}

#[test]
fn sha256_matches_the_known_vector() {
    assert_eq!(
        sha256_hex("abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn random_hex_has_the_asked_length_and_differs() {
    let (a, b) = (random_hex(32), random_hex(32));
    assert_eq!(a.len(), 64);
    assert_ne!(a, b);
}

#[test]
fn lan_host_is_accepted_only_when_sharing() {
    assert!(is_lan_host(
        Some("192.168.1.20:3020"),
        Some("192.168.1.20:3020")
    ));
    assert!(!is_lan_host(Some("192.168.1.20:3020"), None));
    assert!(!is_lan_host(
        Some("192.168.1.21:3020"),
        Some("192.168.1.20:3020")
    ));
    assert!(!is_lan_host(None, Some("192.168.1.20:3020")));
}

#[test]
fn only_private_ipv4_counts_as_lan() {
    let v4 = |a, b, c, d| IpAddr::V4(Ipv4Addr::new(a, b, c, d));
    assert!(is_private(v4(10, 0, 0, 2)));
    assert!(is_private(v4(172, 16, 0, 2)));
    assert!(is_private(v4(172, 31, 255, 1)));
    assert!(is_private(v4(192, 168, 1, 20)));
    assert!(!is_private(v4(172, 32, 0, 1)));
    assert!(!is_private(v4(8, 8, 8, 8)));
    assert!(!is_private(v4(127, 0, 0, 1)));
    assert!(!is_private(IpAddr::V6("::1".parse().unwrap())));
}

#[test]
fn names_are_percent_encoded() {
    assert_eq!(encode("My board_1"), "My%20board_1");
    assert_eq!(encode("é&"), "%C3%A9%26");
}

#[test]
fn start_replaces_the_pending_token() {
    let pair = PairState::new(None);
    let now = Instant::now();
    let (first, expires) = start_pairing(&pair, now, 1000);
    let (second, _) = start_pairing(&pair, now, 1000);
    assert_eq!(expires, 1300);
    assert_ne!(first, second);
    let pending = pair.pending.lock().unwrap();
    assert_eq!(pending.as_ref().unwrap().hash, sha256_hex(&second));
}
