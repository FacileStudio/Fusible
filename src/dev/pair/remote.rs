//! The gate for requests that arrive over the network: bearer token, GET only, short allowlist.

use std::time::Instant;

use tiny_http::{Method, Request};

use super::clock::unix_now;
use super::PairState;
use crate::dev::http::limiter::lock;
use crate::dev::live::state::State;

pub(super) const CLAIM: &str = "/api/pair/claim";

const ALLOWED: [&str; 6] = [
    "/api/mobile/summary",
    "/api/circuit",
    "/api/bom",
    "/api/bodies",
    "/api/rules",
    "/api/project",
];

/// Whether `host` is exactly the shared LAN address; always false when not sharing.
pub(in crate::dev) fn is_lan_host(host: Option<&str>, lan_host: Option<&str>) -> bool {
    matches!((host, lan_host), (Some(h), Some(l)) if h.eq_ignore_ascii_case(l))
}

/// Whether `request` must pass the remote gate instead of the loopback rules.
///
/// While sharing, any peer that is not loopback is remote whatever its Host header says,
/// and so is a loopback peer that names the LAN address.
pub(in crate::dev) fn is_remote(request: &Request, host: Option<&str>, state: &State) -> bool {
    if state.pair.is_none() {
        return false;
    }
    let local_peer = request.remote_addr().is_some_and(|a| a.ip().is_loopback());
    !local_peer || is_lan_host(host, state.lan_host.as_deref())
}

fn is_body_glb(path: &str) -> bool {
    let name = path
        .strip_prefix("/api/bodies/")
        .and_then(|n| n.strip_suffix(".glb"));
    name.is_some_and(|n| !n.is_empty() && !n.contains('/'))
}

/// Whether a remote device may read `path`.
pub(super) fn path_allowed(path: &str) -> bool {
    ALLOWED.contains(&path) || is_body_glb(path)
}

/// Decides a remote request: claim needs nothing, anything else needs a good bearer, GET, and an allowed path.
///
/// The error is the HTTP status to answer with.
pub(super) fn check(
    pair: &PairState,
    method: &Method,
    path: &str,
    bearer: Option<&str>,
    now: Instant,
) -> Result<(), u16> {
    if *method == Method::Post && path == CLAIM {
        return Ok(());
    }
    let id = bearer
        .and_then(|token| lock(&pair.devices).authenticate(token))
        .ok_or(401u16)?;
    if *method != Method::Get {
        return Err(405);
    }
    if !path_allowed(path) {
        return Err(403);
    }
    lock(&pair.devices).touch(&id, now, unix_now());
    Ok(())
}

fn bearer_of(request: &Request) -> Option<String> {
    let header = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Authorization"))?;
    let token = header.value.as_str().strip_prefix("Bearer ")?;
    Some(token.trim().to_string())
}

/// Applies [`check`] to a live request.
pub(in crate::dev) fn authorize_remote(request: &Request, state: &State) -> Result<(), u16> {
    let pair = state.pair.as_deref().ok_or(403u16)?;
    let path = crate::dev::http::server::route_path(request.url());
    let bearer = bearer_of(request);
    check(
        pair,
        request.method(),
        path,
        bearer.as_deref(),
        Instant::now(),
    )
}
