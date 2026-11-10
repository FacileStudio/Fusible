//! `check --json`: the v2 report as one JSON document, and its exit code.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::json;

use crate::check::{self, CheckError, Fuse, Scope};
use crate::commands::check::with_mcu;
use crate::commands::mcp::protocol_out;
use crate::commands::target::resolve_boards;
use crate::project::Project;
use crate::{EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

fn verdict(
    project: &Project,
    file: Option<PathBuf>,
    mcu: Option<String>,
) -> Result<(String, Fuse), CheckError> {
    let boards = with_mcu(resolve_boards(project, file)?, mcu);
    let report = check::run(project, Scope::Boards(boards))?;
    let fuse = report.fuse;
    let document = serde_json::to_string(&report).unwrap_or_default();
    Ok((document, fuse))
}

/// Checks the boards and prints the report as one JSON document on stdout.
///
/// Anything else the check prints goes to stderr. A tool error prints
/// `{"fuse":"error","error":"..."}` and exits 2.
pub(super) fn check_json(
    project: &Project,
    file: Option<PathBuf>,
    mcu: Option<String>,
) -> ExitCode {
    let mut out = protocol_out();
    let (document, code) = match verdict(project, file, mcu) {
        Ok((document, Fuse::Intact)) => (document, ExitCode::SUCCESS),
        Ok((document, Fuse::Blown)) => (document, ExitCode::from(EXIT_VIOLATIONS)),
        Err(err) => (
            json!({"fuse": "error", "error": err.to_string()}).to_string(),
            ExitCode::from(EXIT_TOOL_ERROR),
        ),
    };
    let written = writeln!(out, "{document}").and_then(|()| out.flush());
    match written {
        Ok(()) => code,
        Err(_) => ExitCode::from(EXIT_TOOL_ERROR),
    }
}
