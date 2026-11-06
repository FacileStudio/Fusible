//! Tool `build_body`: build one body and list what it wrote.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::args::{object, required_str, shown, string};
use super::outcome::Outcome;
use crate::bom;
use crate::commands::body::{self, BodyError};
use crate::commands::check::CheckError;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::config::{BodyConfig, EnclosureConfig, FusibleConfig};

pub const DESCRIPTION: &str =
    "Build one body (authored, generated or code) and list its output files. \
Generated bodies are fuse-gated.";

/// Input schema: the body name.
pub fn schema() -> Value {
    object(&[("name", string("Body name"))], &["name"])
}

fn outputs(root: &Path, name: &str, body: &BodyConfig) -> Vec<String> {
    let dir = body
        .enclosure
        .out_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from("dist/bodies"));
    let mut files: Vec<PathBuf> = body.source.iter().cloned().collect();
    let listing = std::fs::read_dir(root.join(dir))
        .into_iter()
        .flatten()
        .flatten();
    files.extend(
        listing
            .map(|e| e.path())
            .filter(|p| p.file_stem().is_some_and(|s| s == name)),
    );
    files.iter().map(|p| shown(root, p)).collect()
}

fn mass(cfg: &FusibleConfig, root: &Path, name: &str) -> Option<f64> {
    let boards: Vec<&str> = cfg.boards.keys().map(String::as_str).collect();
    bom::bodies_of(cfg, &boards, root)
        .into_iter()
        .find(|b| b.name == name)?
        .grams
}

fn built(cfg: &FusibleConfig, root: &Path, name: &str) -> Outcome {
    let outputs = cfg.bodies.get(name).map(|b| outputs(root, name, b));
    Outcome::ok(json!({"outputs": outputs, "mass_g": mass(cfg, root, name), "bbox": null}))
}

/// Builds the named body.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let name = match required_str(args, "name") {
        Ok(name) => name,
        Err(outcome) => return outcome,
    };
    let cfg = match session.config() {
        Ok(Some(cfg)) if cfg.bodies.contains_key(name) => cfg,
        Ok(_) => return Outcome::fail(format!("unknown body \"{name}\"")),
        Err(message) => return Outcome::fail(message),
    };
    match body::run(Some(PathBuf::from(name)), EnclosureConfig::default(), false) {
        Ok(()) => built(&cfg, &session.root, name),
        Err(BodyError::Check(CheckError::FuseBlown(blown))) => Outcome::fail_with(
            json!({"fuse": "blown", "violations": report::violations_json(&blown)}),
        ),
        Err(err) => Outcome::fail(err.to_string()),
    }
}
