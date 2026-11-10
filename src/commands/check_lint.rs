//! Loads, parses and lints one board without printing anything.

use std::path::{Path, PathBuf};

use super::check::CheckError;
use super::source::load_circuit_source;
use crate::linter::{Diagnostic, LintContext, Linter};
use crate::netlist::Netlist;
use crate::parser;
use crate::project::Project;

/// The Circuit JSON path and text of a board with every diagnostic the fuse found.
pub type Linted = (PathBuf, String, Vec<Diagnostic>);

/// Lints the board at `entry` against the manifest's design rules.
pub fn lint_board(
    project: &Project,
    entry: &Path,
    mcu: Option<&str>,
) -> Result<Linted, CheckError> {
    let (json_path, source) = load_circuit_source(entry)?;
    let circuit = parser::parse(&source, &json_path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let ctx = LintContext::new(&circuit, &netlist)
        .with_mcu(mcu)
        .with_design(project.design_rules());
    let diagnostics = Linter::default().run_with_context(&ctx);
    Ok((json_path, source, diagnostics))
}
