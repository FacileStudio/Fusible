use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

use super::claim::{claim, start_pairing, ClaimBody};
use super::claim_tests::PEER;
use super::tests::temp_store;
use super::PairState;

const OTHER: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 11));

fn body(token: &str) -> ClaimBody {
    ClaimBody {
        token: token.into(),
        name: Some("test".into()),
    }
}

fn ready() -> (PairState, String, Instant) {
    let pair = PairState::new(Some(temp_store()));
    let now = Instant::now();
    let (token, _) = start_pairing(&pair, now, 0);
    (pair, token, now)
}

#[test]
fn one_noisy_peer_does_not_block_another() {
    let (pair, token, now) = ready();
    for _ in 0..8 {
        let _ = claim(&pair, &body("bad"), PEER, now, 0);
    }
    assert!(claim(&pair, &body(&token), OTHER, now, 0).is_ok());
}

#[test]
fn the_failure_map_is_capped() {
    let (pair, _, now) = ready();
    for n in 0..300u32 {
        let peer = IpAddr::from([10, 0, (n / 256) as u8, (n % 256) as u8]);
        let _ = claim(&pair, &body("bad"), peer, now, 0);
    }
    assert!(pair.failures.lock().unwrap().len() <= 256);
}

#[test]
fn concurrent_claims_of_one_token_succeed_once() {
    let (pair, token, now) = ready();
    let pair = std::sync::Arc::new(pair);
    let wins: usize = (0..8)
        .map(|_| {
            let (pair, token) = (pair.clone(), token.clone());
            std::thread::spawn(move || claim(&pair, &body(&token), PEER, now, 0).is_ok())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|h| usize::from(h.join().unwrap()))
        .sum();
    assert_eq!(wins, 1);
}

#[test]
fn failures_older_than_a_minute_are_forgotten() {
    let (pair, token, now) = ready();
    for _ in 0..5 {
        let _ = claim(&pair, &body("bad"), PEER, now, 0);
    }
    let later = now + Duration::from_secs(61);
    assert!(claim(&pair, &body(&token), PEER, later, 0).is_ok());
}
