//! Pairing tokens: start one, claim it once, and slow down guessing.

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

fn token_matches(pair: &PairState, token: &str, now: Instant) -> bool {
    let guard = lock(&pair.pending);
    let hash = sha256_hex(token);
    guard
        .as_ref()
        .is_some_and(|p| now < p.expires && ct_eq(hash.as_bytes(), p.hash.as_bytes()))
}

fn recent_failures(pair: &PairState, now: Instant) -> usize {
    let mut failures = lock(&pair.failures);
    failures.retain(|t| now.duration_since(*t) < WINDOW);
    failures.len()
}

fn reject(pair: &PairState, now: Instant) -> u16 {
    lock(&pair.failures).push(now);
    if recent_failures(pair, now) > MAX_FAILURES {
        *lock(&pair.pending) = None;
        return 429;
    }
    401
}

/// Exchanges a valid pairing token for a device id and token; the token works once.
///
/// The error is the HTTP status to answer with.
pub(in crate::dev::pair) fn claim(
    pair: &PairState,
    body: &ClaimBody,
    now: Instant,
    unix: i64,
) -> Result<String, u16> {
    if recent_failures(pair, now) > MAX_FAILURES {
        *lock(&pair.pending) = None;
        return Err(429);
    }
    if !token_matches(pair, &body.token, now) {
        return Err(reject(pair, now));
    }
    *lock(&pair.pending) = None;
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
