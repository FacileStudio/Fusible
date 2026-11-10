//! Dispatch of `fusible flash` and `fusible bringup`.

use std::path::PathBuf;
use std::process::ExitCode;

use crate::commands;
use crate::commands::check::CheckError;
use crate::commands::flash::FlashError;
use crate::project::Project;
use crate::{fail, tool_exit, EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

/// Flashes a firmware; a blown fuse exits 1 before any tool runs.
pub fn flash(project: &Project, firmware: Option<String>) -> ExitCode {
    match commands::flash::run(project, firmware) {
        Ok(()) => ExitCode::SUCCESS,
        Err(FlashError::Check(CheckError::FuseBlown(blown))) => fail(blown, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Writes the bring-up sheet.
pub fn bringup(project: &Project, board: Option<PathBuf>, output: Option<PathBuf>) -> ExitCode {
    tool_exit(commands::bringup::run(project, board, output).map(|_| ()))
}
