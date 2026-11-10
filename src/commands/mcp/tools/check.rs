//! Tool `check`: lint one board or all of them and return the v2 check report.

use serde_json::{json, Value};

use super::args::{object, optional_str, string};
use super::outcome::Outcome;
use crate::check::{self, Scope};
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str =
    "Run the electrical rules. Returns the check report: the fuse (intact or blown), every \
board's diagnostics with message and help, the links and assembly diagnostics, and the waived \
ones. Call after every edit.";

/// Input schema: an optional board name.
pub fn schema() -> Value {
    object(
        &[("board", string("Board name; every board when omitted"))],
        &[],
    )
}

/// Checks the selected boards; a tool error is a failed call with `fuse: "error"`.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let project = session.project();
    let targets = match report::select(&project, optional_str(args, "board")) {
        Ok(targets) => targets,
        Err(message) => return Outcome::fail(message),
    };
    match check::run(&project, Scope::Boards(targets)) {
        Ok(report) => Outcome::ok(serde_json::to_value(&report).unwrap_or(Value::Null)),
        Err(err) => Outcome::fail_with(json!({"fuse": "error", "error": err.to_string()})),
    }
}
