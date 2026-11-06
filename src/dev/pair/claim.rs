//! Pairing tokens: start one, claim it once, and slow down guessing.

use std::net::IpAddr;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::json;

use super::clock::rfc3339;
use super::store::Device;
use super::token::{ct_eq, random_hex, sha256_hex};
use super::{PairState, Pending};
use crate::dev::limiter::lock;

const TTL: Duration = Duration::from_secs(300);
const WINDOW: Duration = Duration::from_secs(60);
const MAX_FAILURES: usize = 5;
const MAX_PEERS: usize = 256;
const MAX_NAME: usize = 64;

#[derive(Deserialize)]
pub(in crate::dev::pair) struct ClaimBody {
    pub(in crate::dev::pair) token: String,
    pub(in crate::dev::pair) name: Option<String>,
}

/// Makes a fresh pairing token, replacing any pending one, and returns it with its expiry.
pub(in crate::dev::pair) fn start_pairing(
    pair: &PairState,
    now: Instant,
    unix: i64,
) -> (String, i64) {
    let token = random_hex(32);
    let pending = Pending {
        hash: sha256_hex(&token),
        expires: now + TTL,
    };
    *lock(&pair.pending) = Some(pending);
    (token, unix + TTL.as_secs() as i64)
}

fn take_if_matches(pair: &PairState, token: &str, now: Instant) -> bool {
    let hash = sha256_hex(token);
    let mut guard = lock(&pair.pending);
    let matches = guard
        .as_ref()
        .is_some_and(|p| now < p.expires && ct_eq(hash.as_bytes(), p.hash.as_bytes()));
    if matches {
        *guard = None;
    }
    matches
}

fn recent_failures(pair: &PairState, peer: IpAddr, now: Instant) -> usize {
    let mut map = lock(&pair.failures);
    let Some(times) = map.get_mut(&peer) else {
        return 0;
    };
    times.retain(|t| now.duration_since(*t) < WINDOW);
    let count = times.len();
    if count == 0 {
        map.remove(&peer);
    }
    count
}

fn record_failure(pair: &PairState, peer: IpAddr, now: Instant) {
    let mut map = lock(&pair.failures);
    if map.len() >= MAX_PEERS && !map.contains_key(&peer) {
        map.retain(|_, times| times.iter().any(|t| now.duration_since(*t) < WINDOW));
    }
    if map.len() >= MAX_PEERS && !map.contains_key(&peer) {
        let oldest = map.iter().min_by_key(|(_, t)| t.last().copied());
        if let Some(key) = oldest.map(|(k, _)| *k) {
            map.remove(&key);
        }
    }
    map.entry(peer).or_default().push(now);
}

fn reject(pair: &PairState, peer: IpAddr, now: Instant) -> u16 {
    record_failure(pair, peer, now);
    match recent_failures(pair, peer, now) > MAX_FAILURES {
        true => 429,
        false => 401,
    }
}

/// Exchanges a valid pairing token for a device id and token; the token works once.
///
/// The error is the HTTP status to answer with.
pub(in crate::dev::pair) fn claim(
    pair: &PairState,
    body: &ClaimBody,
    peer: IpAddr,
    now: Instant,
    unix: i64,
) -> Result<String, u16> {
    if recent_failures(pair, peer, now) > MAX_FAILURES {
        return Err(429);
    }
    if !take_if_matches(pair, &body.token, now) {
        return Err(reject(pair, peer, now));
    }
    let (device_id, device_token) = (random_hex(16), random_hex(32));
    let name: String = body
        .name
        .as_deref()
        .unwrap_or("phone")
        .chars()
        .take(MAX_NAME)
        .collect();
    let stamp = rfc3339(unix);
    let device = Device {
        device_id: device_id.clone(),
        name,
        token_sha256: sha256_hex(&device_token),
        paired_at: stamp.clone(),
        last_seen: stamp,
    };
    lock(&pair.devices).add(device, now).map_err(|_| 500u16)?;
    Ok(json!({ "device_id": device_id, "device_token": device_token }).to_string())
}
