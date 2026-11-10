//! Tool `check`: lint one board or all of them and report the fuse.

use serde_json::{json, Value};

use super::args::{object, optional_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str =
    "Run the electrical rules. Returns the fuse (intact or blown) and every violation \
with its message and help. Call after every edit.";

/// Input schema: an optional board name.
pub fn schema() -> Value {
    object(
        &[("board", string("Board name; every board when omitted"))],
        &[],
    )
}

/// Lints the selected boards.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let project = session.project();
    let targets = match report::select(&project, optional_str(args, "board")) {
        Ok(targets) => targets,
        Err(message) => return Outcome::fail(message),
    };
    let reports: Vec<report::BoardReport> = targets
        .iter()
        .map(|t| report::lint_board(&project, t))
        .collect();
    let boards: Vec<Value> = reports
        .iter()
        .map(|r| json!({"name": r.name, "diagnostics": report::diagnostics_json(r)}))
        .collect();
    Outcome::ok(json!({"fuse": report::fuse(&reports), "boards": boards}))
}
