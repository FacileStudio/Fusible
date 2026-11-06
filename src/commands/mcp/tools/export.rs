//! Tool `export`: fuse-gated fabrication, CAD and documentation exports.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::args::{choice, object, optional_str, required_str, shown, string};
use super::outcome::Outcome;
use crate::commands::check::CheckError;
use crate::commands::export::{self, ExportError};
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::commands::target::BoardTarget;

pub const DESCRIPTION: &str =
    "Export fab (gerbers), cad (KiCad, STEP), docs (SVG, PDF) or all files. \
Refuses while the fuse is blown and returns the violations.";

/// Input schema: board, preset.
pub fn schema() -> Value {
    object(
        &[
            ("board", string("Board name; every board when omitted")),
            (
                "preset",
                choice("Export preset", &["fab", "cad", "docs", "all"]),
            ),
        ],
        &["preset"],
    )
}

fn walk(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        match path.is_dir() {
            true => walk(&path, files),
            false => files.push(path),
        }
    }
}

fn written(root: &Path, target: &BoardTarget, preset: &str) -> Vec<String> {
    let parts = match preset {
        "all" => vec!["fab", "cad", "docs"],
        one => vec![one],
    };
    let mut files = Vec::new();
    parts
        .iter()
        .for_each(|part| walk(&root.join(&target.out_dir).join(part), &mut files));
    files.iter().map(|p| shown(root, p)).collect()
}

fn export_one(root: &Path, target: &BoardTarget, preset: &str) -> Result<Vec<String>, Outcome> {
    match export::run(
        Some(target.entry.clone()),
        Some(preset),
        "gerbers",
        None,
        false,
    ) {
        Ok(()) => Ok(written(root, target, preset)),
        Err(ExportError::Check(CheckError::FuseBlown(blown))) => Err(Outcome::fail_with(
            json!({"fuse": "blown", "violations": report::violations_json(&blown)}),
        )),
        Err(err) => Err(Outcome::fail(err.to_string())),
    }
}

fn export_all(session: &Session, args: &Value) -> Result<Outcome, Outcome> {
    let preset = required_str(args, "preset")?;
    let targets = report::select(optional_str(args, "board")).map_err(Outcome::fail)?;
    let mut files = Vec::new();
    for target in &targets {
        files.extend(export_one(&session.root, target, preset)?);
    }
    Ok(Outcome::ok(json!({"files": files})))
}

/// Exports the selected boards.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    export_all(session, args).unwrap_or_else(|outcome| outcome)
}
