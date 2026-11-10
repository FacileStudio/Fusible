//! The fuse gate every generated or code body passes before a file is written.

use super::BodyError;
use crate::commands::check;
use crate::commands::target::BoardTarget;
use crate::project::Project;

/// How a body run checks its boards: against the project's rules, or not at all with `skip_lint`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Gate<'a> {
    pub project: &'a Project,
    pub skip_lint: bool,
}

/// Lints the board unless `skip_lint` is set, and returns its Circuit JSON.
pub(super) fn fuse_gate(gate: Gate, board: &BoardTarget) -> Result<String, BodyError> {
    let Gate { project, skip_lint } = gate;
    if skip_lint {
        println!("⚠️  Skipping electrical rule verification (--skip-lint requested).\n");
        return Ok(check::load_circuit_source(&board.entry)?.1);
    }
    println!("🔍 Verifying electrical rules before generating bodies (fuse check)...");
    let (_, source) = check::lint_source(project, &board.entry, board.mcu.clone())?;
    println!("⚡ Fuse intact — proceeding with body generation.\n");
    Ok(source)
}
