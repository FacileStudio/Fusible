//! JSON endpoints of the dev server.

pub(super) mod bom;
pub(super) mod bom_read;
pub(super) mod firmware;
pub(super) mod fix_api;
pub(super) mod project;
pub(super) mod rules_api;

#[cfg(test)]
mod api_tests;
#[cfg(test)]
mod bom_tests;
#[cfg(test)]
mod firmware_tests;
#[cfg(test)]
mod rules_api_tests;

use std::path::Path;

use tiny_http::Request;

use crate::dev::api::project::project_json;
use crate::dev::http::assets::http_response;
use crate::dev::http::server::query_param;
use crate::dev::live::state::State;
use crate::project::Project;

pub(in crate::dev) const JSON: &str = "application/json";

pub(in crate::dev) fn respond_json(request: Request, status: u16, body: String) {
    let _ = request.respond(http_response(status, JSON, body.into_bytes()));
}

pub(in crate::dev) fn error_body(message: &str) -> String {
    serde_json::json!({ "error": message }).to_string()
}

/// The status and body for the payload of `board`; `{}` while its first build is running.
pub(in crate::dev) fn circuit_reply(state: &State, board: Option<&str>) -> (u16, String) {
    match state.slot(board) {
        Some(slot) => (
            200,
            Some(slot.payload())
                .filter(|p| !p.is_empty())
                .unwrap_or_else(|| "{}".into()),
        ),
        None => (404, error_body("unknown board")),
    }
}

/// Serves the latest payload of the board named by `?board=`.
pub(in crate::dev) fn circuit(request: Request, state: &State) {
    let board = query_param(request.url(), "board");
    let (status, body) = circuit_reply(state, board.as_deref());
    respond_json(request, status, body);
}

/// Serves the generated body preview of the board named by `?board=`.
pub(in crate::dev) fn body_preview(request: Request, state: &State, project: &Project, glb: bool) {
    let board = query_param(request.url(), "board");
    match state.slot(board.as_deref()) {
        Some(slot) => slot.body.serve(request, glb, project),
        None => respond_json(request, 404, error_body("unknown board")),
    }
}

/// Serves the project tree.
pub(in crate::dev) fn project(request: Request, state: &State, current: &Project) {
    let cfg = current.found().map(|(_, cfg)| cfg);
    let root = current.manifest_dir().unwrap_or(Path::new(""));
    respond_json(request, 200, project_json(state, cfg, root));
}

/// Serves the KiCad status of every board that has a KiCad file.
pub(in crate::dev) fn kicad(request: Request, state: &State) {
    respond_json(request, 200, state.kicad.json());
}

/// Serves the recent runs with their logs.
pub(in crate::dev) fn runs(request: Request, state: &State) {
    respond_json(request, 200, state.runs.snapshot());
}
