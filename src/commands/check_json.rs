//! `fusible check --json`: the fuse verdict and every diagnostic as one JSON document.

use std::path::PathBuf;

use serde_json::{json, Value};

use super::check::CheckError;
use super::check_lint::lint_board;
use super::target::resolve_boards;
use crate::dev::pipeline::DevDiagnostic;
use crate::linter::Severity;
use crate::project::Project;

/// The verdict document and whether the fuse blew.
pub type Verdict = (Value, bool);

/// Lints every board and builds `{"fuse", "boards": [{"name", "diagnostics"}]}`.
///
/// Nothing is printed here. The manifest's `links` and `assembly` checks are not part
/// of this document; board diagnostics are.
pub fn run(
    project: &Project,
    file: Option<PathBuf>,
    mcu: Option<String>,
) -> Result<Verdict, CheckError> {
    let mut blown = false;
    let mut boards = Vec::new();
    for board in resolve_boards(project, file)? {
        let target_mcu = mcu.clone().or_else(|| board.mcu.clone());
        let (_, _, found) = lint_board(project, &board.entry, target_mcu.as_deref())?;
        blown |= found.iter().any(|d| d.severity == Severity::Error);
        let stem = board
            .entry
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned());
        let name = board.name.clone().or(stem).unwrap_or(board.title);
        let diagnostics: Vec<DevDiagnostic> = found.into_iter().map(DevDiagnostic::from).collect();
        boards.push(json!({ "name": name, "diagnostics": diagnostics }));
    }
    let fuse = if blown { "blown" } else { "intact" };
    Ok((json!({ "fuse": fuse, "boards": boards }), blown))
}
