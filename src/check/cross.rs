//! Cross-world checks: board-to-board links, then the assembly and the bodies.

use std::path::Path;

use super::CheckError;
use crate::commands::assembly::{self, AssemblyError};
use crate::commands::body::mech_findings;
use crate::commands::links::check_links;
use crate::commands::target::BoardTarget;
use crate::linter::Diagnostic;
use crate::project::Project;

/// Whether `boards` covers every board of the manifest, the condition for any cross check.
pub(super) fn every_board(project: &Project, boards: &[BoardTarget]) -> bool {
    let Some((_, cfg)) = project.found() else {
        return false;
    };
    !cfg.boards.is_empty() && cfg.boards.len() == boards.len()
}

fn violations(result: Result<(), CheckError>) -> Result<Vec<Diagnostic>, CheckError> {
    match result {
        Ok(()) => Ok(Vec::new()),
        Err(CheckError::FuseBlown(blown)) => Ok(blown.violations),
        Err(other) => Err(other),
    }
}

/// The `harness_mismatch` diagnostics of the manifest's `links:`; empty unless `complete`.
pub(super) fn links(project: &Project, complete: bool) -> Result<Vec<Diagnostic>, CheckError> {
    let Some((_, cfg)) = project.found().filter(|_| complete) else {
        return Ok(Vec::new());
    };
    match cfg.links.is_empty() {
        true => Ok(Vec::new()),
        false => violations(check_links(cfg)),
    }
}

/// The assembly rules, when `assembly:` is declared, then the mechanical rules of every body.
///
/// Empty unless `complete`. A body that cannot be built or measured is a tool error.
pub(super) fn assembly(project: &Project, complete: bool) -> Result<Vec<Diagnostic>, CheckError> {
    let Some((manifest, cfg)) = project.found().filter(|_| complete) else {
        return Ok(Vec::new());
    };
    let root = manifest.parent().unwrap_or(Path::new("."));
    let mut found = match cfg.assembly.is_some() {
        true => violations(
            assembly::verify(project, root).map_err(|error| match error {
                AssemblyError::Check(inner) => inner,
                other => CheckError::Assembly(Box::new(other)),
            }),
        )?,
        false => Vec::new(),
    };
    found.extend(mech_findings(cfg, root));
    Ok(found)
}
