//! `GET /api/bodies`: every body of the manifest with its material, mass, cost and model URL.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::UNIX_EPOCH;

use serde_json::{json, Value};

use crate::bom::{bodies_of, materials, resolve_material};
use crate::commands::body::read_report;
use crate::config::{BodyConfig, FusibleConfig};
use crate::dev::project::shown;

fn kind(body: &BodyConfig) -> &'static str {
    match (body.is_code(), body.source.is_some()) {
        (true, _) => "code",
        (false, true) => "authored",
        _ => "generated",
    }
}

/// Hashes the bytes and the modification time of `path`, so an edit changes the model URL.
pub(in crate::dev) fn revision(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    let nanos = std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let mut hasher = DefaultHasher::new();
    (bytes, nanos).hash(&mut hasher);
    Some(format!("{:016x}", hasher.finish()))
}

fn glb_url(name: &str, body: &BodyConfig, root: &Path) -> Value {
    let source = body.source.as_ref().map(|s| root.join(s));
    match source.and_then(|s| revision(&s)) {
        Some(rev) => json!(format!("/api/bodies/{name}.glb?rev={rev}")),
        None => Value::Null,
    }
}

fn cost(grams: f64, body: &BodyConfig) -> f64 {
    let (key, _) = resolve_material(body.material.as_deref(), body.process.as_deref());
    let per_kg = materials().get(&key).map_or(0.0, |m| m.cost_per_kg);
    (grams / 1000.0 * per_kg * 100.0).round() / 100.0
}

fn item(name: &str, body: &BodyConfig, grams: Option<f64>, root: &Path) -> Value {
    let bbox = read_report(root, name, body).and_then(|r| r.bbox);
    json!({
        "name": name,
        "kind": kind(body),
        "source": body.source.as_deref().map(|s| shown(s, root)),
        "material": body.material,
        "process": body.process,
        "mass_g": grams,
        "cost": grams.map(|g| cost(g, body)),
        "bbox": bbox,
        "glb": glb_url(name, body, root),
    })
}

/// The JSON array of `GET /api/bodies` for the manifest `cfg` at `root`.
pub(in crate::dev) fn bodies_json(cfg: &FusibleConfig, root: &Path) -> String {
    let boards: Vec<&str> = cfg.boards.keys().map(String::as_str).collect();
    let printed = bodies_of(cfg, &boards, root);
    let items: Vec<Value> = cfg
        .bodies
        .iter()
        .map(|(name, body)| {
            let grams = printed
                .iter()
                .find(|p| &p.name == name)
                .and_then(|p| p.grams);
            item(name, body, grams, root)
        })
        .collect();
    Value::Array(items).to_string()
}
