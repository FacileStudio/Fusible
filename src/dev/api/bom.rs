//! `GET /api/bom` and `POST /api/bom`: the priced parts list and its editable file.

use std::io::Read;

use serde_json::json;
use tiny_http::{Method, Request};

use super::bom_read::file_for_read;
use crate::bom::{self, BomError, BomFile, Part};
use crate::dev::api::{error_body, respond_json};
use crate::dev::live::state::State;
use crate::dev::run::api::forbidden;
use crate::project::Project;

const MAX_REQUEST: u64 = 256 * 1024;

fn parts_of(state: &State) -> (Vec<Part>, usize) {
    let scoped = state.slots.len() > 1;
    let mut parts = Vec::new();
    let mut pending = 0;
    for slot in state.slots.iter() {
        let scope = slot.name.as_deref().filter(|_| scoped);
        match slot.source().map(|source| bom::detect(&source, scope)) {
            Some(Ok(found)) => parts.extend(found),
            _ => pending += 1,
        }
    }
    (parts, pending)
}

/// The priced report of every served board with `file` applied, and how many boards are still building.
pub(in crate::dev) fn report_of(
    state: &State,
    file: &BomFile,
    project: &Project,
) -> (bom::Report, usize) {
    let (parts, pending) = parts_of(state);
    let root = project.manifest_dir();
    let cfg = &project.config;
    let names: Vec<&str> = state
        .slots
        .iter()
        .filter_map(|s| s.name.as_deref())
        .collect();
    let bodies = bom::bodies_of(cfg, &names, root.unwrap_or(std::path::Path::new(".")));
    (bom::build(parts, &bodies, file), pending)
}

fn reply(state: &State, project: &Project, file: &BomFile, editable: bool) -> String {
    let (report, pending) = report_of(state, file, project);
    json!({ "report": report, "file": file, "editable": editable, "pending": pending }).to_string()
}

fn read_body(request: &mut Request) -> Result<String, String> {
    let mut body = String::new();
    let read = request
        .as_reader()
        .take(MAX_REQUEST)
        .read_to_string(&mut body);
    read.map(|_| body)
        .map_err(|_| "unreadable request body".to_string())
}

pub(in crate::dev) fn save(path: &std::path::Path, body: &str) -> Result<BomFile, (u16, String)> {
    let file: BomFile = serde_json::from_str(body).map_err(|e| (400, format!("bad BOM: {e}")))?;
    file.save(path).map_err(|e| match e {
        BomError::Invalid(message) => (400, message),
        other => (500, other.to_string()),
    })?;
    Ok(file)
}

fn update(mut request: Request, state: &State, port: u16, project: &Project) {
    if forbidden(&request, port) {
        return respond_json(request, 403, error_body("forbidden"));
    }
    let Some((manifest, _)) = project.found() else {
        return respond_json(
            request,
            409,
            error_body("no fusible.yml: nowhere to save bom.yml"),
        );
    };
    let body = match read_body(&mut request) {
        Ok(body) => body,
        Err(message) => return respond_json(request, 400, error_body(&message)),
    };
    match save(&bom::bom_path(manifest), &body) {
        Ok(file) => respond_json(request, 200, reply(state, project, &file, true)),
        Err((status, message)) => respond_json(request, status, error_body(&message)),
    }
}

fn read(request: Request, state: &State, project: &Project) {
    let Some((manifest, _)) = project.found() else {
        let empty = reply(state, project, &BomFile::default(), false);
        return respond_json(request, 200, empty);
    };
    match file_for_read(&bom::bom_path(manifest)) {
        Ok(file) => respond_json(request, 200, reply(state, project, &file, true)),
        Err(err) => respond_json(request, 422, error_body(&err.to_string())),
    }
}

/// Serves the report on GET and saves the edited file on POST.
pub(in crate::dev) fn serve(request: Request, state: &State, port: u16, project: &Project) {
    match request.method() {
        Method::Get => read(request, state, project),
        Method::Post => update(request, state, port, project),
        _ => respond_json(request, 405, error_body("use GET or POST")),
    }
}
