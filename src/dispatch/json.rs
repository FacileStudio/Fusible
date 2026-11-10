//! `check --json` exit-code mapping.

use std::path::PathBuf;
use std::process::ExitCode;

use crate::project::Project;
use crate::{commands, EXIT_TOOL_ERROR};

/// Runs the electrical linter and prints its verdict as one JSON document.
///
/// A tool error prints `{"fuse":"error","error":"..."}` and exits 2.
pub(super) fn check_json(
    project: &Project,
    file: Option<PathBuf>,
    mcu: Option<String>,
) -> ExitCode {
    match commands::check_json::run(project, file, mcu) {
        Ok((document, blown)) => {
            println!("{document}");
            ExitCode::from(u8::from(blown))
        }
        Err(err) => {
            println!(
                "{}",
                serde_json::json!({"fuse": "error", "error": err.to_string()})
            );
            ExitCode::from(EXIT_TOOL_ERROR)
        }
    }
}
