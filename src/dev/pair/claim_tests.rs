use std::time::{Duration, Instant};

use super::claim::{claim, start_pairing, ClaimBody};
use super::tests::temp_store;
use super::PairState;

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
fn a_token_claims_once_and_returns_a_device_token() {
    let (pair, token, now) = ready();
    let reply = claim(&pair, &body(&token), now, 0).unwrap();
    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
    assert_eq!(json["device_id"].as_str().unwrap().len(), 32);
    let device_token = json["device_token"].as_str().unwrap();
    assert_eq!(device_token.len(), 64);
    assert!(pair
        .devices
        .lock()
        .unwrap()
        .authenticate(device_token)
        .is_some());
    assert_eq!(claim(&pair, &body(&token), now, 0), Err(401));
}

#[test]
fn an_expired_token_is_refused() {
    let (pair, token, now) = ready();
    let late = now + Duration::from_secs(301);
    assert_eq!(claim(&pair, &body(&token), late, 0), Err(401));
}

#[test]
fn a_wrong_token_is_refused() {
    let (pair, _, now) = ready();
    assert_eq!(claim(&pair, &body("00"), now, 0), Err(401));
}

#[test]
fn six_bad_claims_give_429_and_clear_the_pending_token() {
    let (pair, token, now) = ready();
    for _ in 0..5 {
        assert_eq!(claim(&pair, &body("bad"), now, 0), Err(401));
    }
    assert_eq!(claim(&pair, &body("bad"), now, 0), Err(429));
    assert!(pair.pending.lock().unwrap().is_none());
    assert_eq!(claim(&pair, &body(&token), now, 0), Err(429));
}

#[test]
fn failures_older_than_a_minute_are_forgotten() {
    let (pair, token, now) = ready();
    for _ in 0..5 {
        let _ = claim(&pair, &body("bad"), now, 0);
    }
    let later = now + Duration::from_secs(61);
    assert!(claim(&pair, &body(&token), later, 0).is_ok());
}

#[test]
fn devices_survive_a_reload() {
    let path = temp_store();
    let pair = PairState::new(Some(path.clone()));
    let now = Instant::now();
    let (token, _) = start_pairing(&pair, now, 0);
    let reply = claim(&pair, &body(&token), now, 0).unwrap();
    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
    let again = PairState::new(Some(path));
    let secret = json["device_token"].as_str().unwrap();
    assert!(again.devices.lock().unwrap().authenticate(secret).is_some());
    assert_eq!(again.devices.lock().unwrap().view().len(), 1);
}
