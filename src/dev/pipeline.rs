//! The studio payload of one board, built from the shared check pipeline.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use serde::Serialize;

use crate::check::{check_source, CheckError, WaivedDiagnostic};
use crate::commands::check::load_circuit_source;
use crate::commands::target::board_for_entry;
use crate::linter::Diagnostic;
use crate::project::Project;

/// The studio payload pushed over SSE and served at `/api/circuit`.
///
/// `board` is the manifest name of the board, absent for a file outside the manifest.
///
/// `revision` is a 16-hex-char hash of the Circuit JSON source text, or empty when
/// the source could not be loaded.
#[derive(Debug, Serialize)]
pub struct DevPayload {
    pub board_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board: Option<String>,
    pub fuse_blown: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub waived: Vec<WaivedDiagnostic>,
    pub circuit_json: Option<serde_json::Value>,
    pub revision: String,
}

/// Builds the studio payload for `entry`, returning the Circuit JSON source text alongside it.
///
/// The board settings and design rules come from `project`.
pub fn execute_pipeline(project: &Project, entry: &Path) -> (DevPayload, Option<String>) {
    let board = board_for_entry(project, entry);
    let (mut payload, source) = match load_circuit_source(entry) {
        Ok((json_path, source)) => {
            let mcu = board.mcu.as_deref();
            let payload = analyze(project, board.title, mcu, &json_path, &source);
            (payload, Some(source))
        }
        Err(err) => (source_failure(board.title, err.to_string()), None),
    };
    payload.board = board.name;
    (payload, source)
}

/// Hashes the Circuit JSON source text into a stable 16-hex-char revision.
pub fn revision(source: &str) -> String {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn diagnostic(rule: &str, message: String, help: &str) -> Diagnostic {
    Diagnostic {
        rule: rule.to_string(),
        message,
        help: Some(help.to_string()),
        ..Default::default()
    }
}

fn source_failure(board_name: String, message: String) -> DevPayload {
    DevPayload {
        board_name,
        board: None,
        fuse_blown: true,
        diagnostics: vec![diagnostic(
            "source_error",
            message,
            "Ensure your circuit file compiles and is valid JSON/TSX",
        )],
        waived: Vec::new(),
        circuit_json: None,
        revision: String::new(),
    }
}

fn failure(error: CheckError) -> Diagnostic {
    match error {
        CheckError::Parse(err) => {
            diagnostic("parse_error", err.to_string(), "Check circuit JSON syntax")
        }
        other => diagnostic(
            "config_error",
            other.to_string(),
            "Fix fusible.yml: its rules and waivers must name known rule ids",
        ),
    }
}

fn analyze(
    project: &Project,
    board_name: String,
    mcu: Option<&str>,
    json_path: &Path,
    source: &str,
) -> DevPayload {
    let mut payload = DevPayload {
        board_name,
        board: None,
        fuse_blown: true,
        diagnostics: Vec::new(),
        waived: Vec::new(),
        circuit_json: serde_json::from_str(source).ok(),
        revision: revision(source),
    };
    match check_source(project, mcu, json_path, source) {
        Ok(checked) => {
            payload.fuse_blown = checked.blown();
            payload.diagnostics = checked.diagnostics;
            payload.waived = checked.waived;
        }
        Err(err) => payload.diagnostics.push(failure(err)),
    }
    payload
}
