//! One board: parsed once, lowered to a netlist once, linted.

use std::path::{Path, PathBuf};

use super::CheckError;
use crate::commands::source::load_circuit_source;
use crate::linter::{Diagnostic, LintContext, Linter, Severity};
use crate::netlist::Netlist;
use crate::parser::{self, Circuit};
use crate::project::Project;

/// A linted board: its circuit, netlist and diagnostics.
pub struct Checked {
    pub circuit: Circuit,
    pub netlist: Netlist,
    pub diagnostics: Vec<Diagnostic>,
}

impl Checked {
    /// Whether any diagnostic left blows the fuse.
    pub fn blown(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
}

/// What one board needs besides its source: the MCU it targets.
#[derive(Clone, Copy)]
pub(super) struct Input<'a> {
    pub mcu: Option<&'a str>,
    pub json_path: &'a Path,
    pub source: &'a str,
}

/// Parses and lints `input` once.
pub(super) fn check_with(project: &Project, input: Input<'_>) -> Result<Checked, CheckError> {
    let circuit = parser::parse(input.source, input.json_path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let ctx = LintContext::new(&circuit, &netlist)
        .with_mcu(input.mcu)
        .with_design(project.design_rules());
    let diagnostics = Linter::default().run_with_context(&ctx);
    Ok(Checked {
        circuit,
        netlist,
        diagnostics,
    })
}

/// Checks the Circuit JSON text `source` of a board against the project's design rules.
pub fn check_source(
    project: &Project,
    mcu: Option<&str>,
    json_path: &Path,
    source: &str,
) -> Result<Checked, CheckError> {
    let input = Input {
        mcu,
        json_path,
        source,
    };
    check_with(project, input)
}

/// Loads the circuit at `entry`, compiling TSX when needed, and checks it.
///
/// Returns the Circuit JSON path and text with the result.
pub fn check_entry(
    project: &Project,
    entry: &Path,
    mcu: Option<&str>,
) -> Result<(PathBuf, String, Checked), CheckError> {
    let (json_path, source) = load_circuit_source(entry)?;
    let checked = check_source(project, mcu, &json_path, &source)?;
    Ok((json_path, source, checked))
}
