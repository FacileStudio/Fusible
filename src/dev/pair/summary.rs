//! `GET /api/mobile/summary`: one small document with everything the phone home screen shows.

use serde_json::{json, Value};

use super::dirs::project_name;
use super::token::sha256_hex;
use crate::bom::BomFile;
use crate::dev::bom::report_of;
use crate::dev::project::body_item;
use crate::dev::state::{Slot, State};
use crate::project::Project;

fn payload_of(slot: &Slot) -> Value {
    serde_json::from_str(&slot.payload()).unwrap_or(Value::Null)
}

/// Counts errors and warnings in a payload.
///
/// Diagnostics without a `severity` field fall back to the fuse: they are errors when it is blown, else warnings.
pub(super) fn counts(payload: &Value, fuse_blown: bool) -> (usize, usize) {
    let list = payload["diagnostics"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let severity = |d: &Value| d["severity"].as_str().map(str::to_string);
    if list.iter().any(|d| severity(d).is_some()) {
        let of = |wanted: &str| {
            list.iter()
                .filter(|d| severity(d).as_deref() == Some(wanted))
                .count()
        };
        return (of("error"), of("warning"));
    }
    if fuse_blown {
        (list.len(), 0)
    } else {
        (0, list.len())
    }
}

fn board_entry(slot: &Slot) -> Value {
    let name = slot.name.clone().or_else(|| {
        let stem = slot.entry.file_stem();
        stem.map(|s| s.to_string_lossy().into_owned())
    });
    let (errors, warnings) = counts(&payload_of(slot), slot.health().fuse_blown);
    json!({ "name": name, "fuse_blown": slot.health().fuse_blown, "errors": errors, "warnings": warnings })
}

fn revision(state: &State) -> String {
    let all: Vec<String> = state
        .slots
        .iter()
        .map(|slot| {
            payload_of(slot)["revision"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        })
        .collect();
    sha256_hex(&all.join(",")).chars().take(16).collect()
}

fn bom_total(state: &State, project: &Project) -> Value {
    let Some((manifest, _)) = project.found() else {
        return Value::Null;
    };
    match BomFile::load(&crate::bom::bom_path(manifest)) {
        Ok(file) => {
            let (report, _) = report_of(state, &file, project);
            json!({ "amount": report.totals.total, "currency": report.currency })
        }
        Err(_) => Value::Null,
    }
}

fn bodies(project: &Project) -> Vec<Value> {
    let Some((manifest, cfg)) = project.found() else {
        return Vec::new();
    };
    let root = manifest.parent().unwrap_or(std::path::Path::new("."));
    let item = |(name, body): (&String, &crate::config::BodyConfig)| {
        let full = body_item(name, body, root);
        json!({ "name": full["name"], "kind": full["kind"] })
    };
    cfg.bodies.iter().map(item).collect()
}

fn last_run(state: &State) -> Value {
    let runs: Vec<Value> = serde_json::from_str(&state.runs.snapshot()).unwrap_or_default();
    runs.last().cloned().unwrap_or(Value::Null)
}

/// Builds the summary document from the same state the circuit, BOM and run routes read.
pub(in crate::dev::pair) fn summary_json(state: &State, project: &Project) -> String {
    json!({
        "project": project_name(project),
        "revision": revision(state),
        "boards": state.slots.iter().map(board_entry).collect::<Vec<_>>(),
        "bom_total": bom_total(state, project),
        "bodies": bodies(project),
        "last_run": last_run(state),
    })
    .to_string()
}
