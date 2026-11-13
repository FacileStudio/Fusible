use std::thread;

use tiny_http::{Request, Response};

use super::bind::bind;
use super::desktop::{announce_ready, exit_on_stdin_eof};
use super::guard::admit;
use super::limiter::handlers;
use super::routes::api_route;
use super::serve::serve_static;
use crate::commands::target::BoardTarget;
use crate::dev::kicad::watch::start_kicad_watchers;
use crate::dev::live::hub::stream_events;
use crate::dev::live::refresh::{
    load_bundle, rebuild_trigger, refresh, warn_if_bundle_missing, watch_dir,
};
use crate::dev::live::state::{Slot, State};
use crate::dev::live::watcher::start_file_watcher;
use crate::dev::pair::{enable_sharing, lan_ip};
use crate::project::Project;

/// Returns the percent-decoded value of `key` in the URL's query string; empty counts as absent.
pub(in crate::dev) fn query_param(url: &str, key: &str) -> Option<String> {
    let (_, query) = url.split_once('?')?;
    let value = query
        .split('&')
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
        .filter(|value| !value.is_empty())?;
    percent_decode(value)
}

pub(in crate::dev) fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = bytes
            .get(i + 1..i + 3)
            .and_then(|h| std::str::from_utf8(h).ok());
        match (bytes[i], hex.and_then(|h| u8::from_str_radix(h, 16).ok())) {
            (b'%', Some(byte)) => (out.push(byte), i += 3),
            (b'+', _) => (out.push(b' '), i += 1),
            (byte, _) => (out.push(byte), i += 1),
        };
    }
    String::from_utf8(out).ok()
}

/// Strips any `?query` from a request URL, leaving the path used for routing.
pub(in crate::dev) fn route_path(url: &str) -> &str {
    url.split_once('?').map_or(url, |(path, _)| path)
}

pub(in crate::dev) fn handle(request: Request, state: &State, port: u16) {
    if let Err(status) = admit(&request, state, port) {
        let _ = request.respond(Response::from_string("Forbidden").with_status_code(status));
        return;
    }
    let path = route_path(request.url()).to_string();
    match path.as_str() {
        "/events" => subscribe(request, state),
        p if p.starts_with("/api/") => api_route(request, state, port, p, &Project::current()),
        _ => serve_static(request, state, &path),
    }
}

fn subscribe(request: Request, state: &State) {
    let ready = || {
        state
            .slots
            .iter()
            .map(Slot::payload)
            .filter(|p| !p.is_empty())
    };
    let rx = state.hub.subscribe(|| ready().collect());
    thread::spawn(move || stream_events(request, rx));
}

/// Handles each request on its own thread so a stalled client cannot block the others.
///
/// At most [`limiter::handlers`] requests run at once; the rest get a 503.
fn dispatch(request: Request, state: &State, port: u16) {
    let Some(permit) = handlers().try_acquire() else {
        let _ = request.respond(Response::from_string("Busy").with_status_code(503));
        return;
    };
    let state = state.clone();
    thread::spawn(move || {
        handle(request, &state, port);
        drop(permit);
    });
}

/// Builds every board in the background, so the server answers while compiles run.
fn refresh_all(state: &State) {
    for slot in state.slots.iter() {
        let (slot, state) = (slot.clone(), state.clone());
        thread::spawn(move || refresh(&slot, &state));
    }
}

/// Serves the studio until the process ends.
///
/// With `desktop` it prints `FUSIBLE_READY {json}` once bound, serves the desktop viewer, and
/// exits when stdin closes.
pub fn run_dev_server(
    boards: Vec<BoardTarget>,
    asked: u16,
    desktop: bool,
    share: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let lan = share.then(lan_ip).transpose()?;
    let (server, port) = bind(asked, share)?;
    if port != asked {
        println!("ℹ Port {asked} is in use, using {port} instead.");
    }
    let mut state = State::new(&boards);
    state.desktop = desktop;
    enable_sharing(&mut state, lan, port);
    if desktop {
        announce_ready(port);
        exit_on_stdin_eof();
    }
    load_bundle(&state);
    warn_if_bundle_missing(&state);
    refresh_all(&state);
    println!("⚡ Fusible Dev Server running at http://localhost:{port} (press Ctrl+C to quit)");
    let _watchers = state
        .slots
        .iter()
        .map(|slot| {
            let trigger = rebuild_trigger(slot.clone(), state.clone());
            start_file_watcher(&watch_dir(&slot.entry), trigger)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let _kicad = start_kicad_watchers(&state);
    for request in server.incoming_requests() {
        dispatch(request, &state, port);
    }
    Ok(())
}
