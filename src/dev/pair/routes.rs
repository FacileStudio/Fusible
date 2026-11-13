//! The pairing endpoints: start, claim, list and revoke.

use std::io::Read;
use std::net::IpAddr;
use std::time::Instant;

use serde_json::json;
use tiny_http::{Method, Request};

use super::claim::{claim, start_pairing, ClaimBody};
use super::clock::{rfc3339, unix_now};
use super::dirs::project_name;
use super::remote::CLAIM;
use super::summary::summary_json;
use super::token::encode;
use super::PairState;

const MAX_BODY: u64 = 4096;
use crate::dev::api::{error_body, respond_json};
use crate::dev::http::limiter::lock;
use crate::dev::live::state::State;
use crate::project::Project;

/// Whether the pairing module serves `path`.
pub(in crate::dev) fn owns(path: &str) -> bool {
    path == "/api/mobile/summary" || path == "/api/pair/start" || path.starts_with("/api/pair/")
}

fn fail(request: Request, status: u16, message: &str) {
    respond_json(request, status, error_body(message));
}

fn handle_start(request: Request, state: &State, pair: &PairState, project: &Project) {
    let Some(host) = state.lan_host.as_deref() else {
        return fail(request, 404, "run fusible studio --share");
    };
    let (token, expires) = start_pairing(pair, Instant::now(), unix_now());
    let name = encode(&project_name(project));
    let uri = format!("fusible://pair?h={host}&t={token}&n={name}");
    respond_json(
        request,
        200,
        json!({ "uri": uri, "expires_at": rfc3339(expires) }).to_string(),
    );
}

fn handle_claim(mut request: Request, pair: &PairState) {
    let mut text = String::new();
    let read = request.as_reader().take(MAX_BODY).read_to_string(&mut text);
    let parsed = read
        .ok()
        .and_then(|_| serde_json::from_str::<ClaimBody>(&text).ok());
    let Some(body) = parsed else {
        return fail(request, 400, "bad claim body");
    };
    let peer = request
        .remote_addr()
        .map_or(IpAddr::from([0, 0, 0, 0]), |addr| addr.ip());
    match claim(pair, &body, peer, Instant::now(), unix_now()) {
        Ok(reply) => respond_json(request, 200, reply),
        Err(429) => fail(request, 429, "too many attempts"),
        Err(status) => fail(request, status, "pairing refused"),
    }
}

fn handle_revoke(request: Request, pair: &PairState, id: &str) {
    let removed = lock(&pair.devices).remove(id);
    let status = if removed { 204 } else { 404 };
    let _ = request.respond(tiny_http::Response::empty(status));
}

/// Routes one pairing request. Only reached once the guards have passed.
pub(in crate::dev) fn serve(request: Request, state: &State, path: &str, project: &Project) {
    let Some(pair) = state.pair.as_deref() else {
        return match path {
            "/api/pair/start" => fail(request, 404, "run fusible studio --share"),
            _ => fail(request, 404, "not sharing"),
        };
    };
    match (request.method().clone(), path) {
        (Method::Get, "/api/mobile/summary") => {
            respond_json(request, 200, summary_json(state, project))
        }
        (Method::Post, "/api/pair/start") => handle_start(request, state, pair, project),
        (Method::Post, CLAIM) => handle_claim(request, pair),
        (Method::Get, "/api/pair/devices") => {
            respond_json(request, 200, json!(lock(&pair.devices).view()).to_string())
        }
        (Method::Delete, p) if p.starts_with("/api/pair/devices/") => {
            handle_revoke(request, pair, &p["/api/pair/devices/".len()..])
        }
        _ => fail(request, 405, "method not allowed"),
    }
}
