//! One board: parsed once, lowered to a netlist once, linted, then levelled and waived.

use std::path::{Path, PathBuf};

use super::levels::Levels;
use super::report::WaivedDiagnostic;
use super::CheckError;
use crate::commands::source::load_circuit_source;
use crate::config::Date;
use crate::linter::{Diagnostic, LintContext, Linter, Severity};
use crate::netlist::Netlist;
use crate::parser::{self, Circuit};
use crate::project::Project;

/// A linted board: its circuit, netlist and the diagnostics left after levels and waivers.
pub struct Checked {
    pub circuit: Circuit,
    pub netlist: Netlist,
    pub diagnostics: Vec<Diagnostic>,
    pub waived: Vec<WaivedDiagnostic>,
}

impl Checked {
    /// Whether any diagnostic left blows the fuse.
    pub fn blown(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
}

/// What one board needs besides its source: the board name and the MCU it targets.
#[derive(Clone, Copy)]
pub(super) struct Input<'a> {
    pub name: Option<&'a str>,
    pub mcu: Option<&'a str>,
    pub json_path: &'a Path,
    pub source: &'a str,
}

/// Parses and lints `input` once, applying the levels and waivers of `levels`.
pub(super) fn check_with(
    project: &Project,
    levels: &mut Levels<'_>,
    input: Input<'_>,
) -> Result<Checked, CheckError> {
    let circuit = parser::parse(input.source, input.json_path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let ctx = LintContext::new(&circuit, &netlist)
        .with_mcu(input.mcu)
        .with_design(project.design_rules());
    let found = Linter::default().run_with_context(&ctx);
    let mut waived = Vec::new();
    let diagnostics = levels.apply(found, Some(&circuit), input.name, &mut waived);
    Ok(Checked {
        circuit,
        netlist,
        diagnostics,
        waived,
    })
}

/// Checks the Circuit JSON text `source` of a board with the project's levels and waivers.
pub fn check_source(
    project: &Project,
    mcu: Option<&str>,
    json_path: &Path,
    source: &str,
) -> Result<Checked, CheckError> {
    let mut levels = Levels::new(&project.config, Date::today())?;
    let input = Input {
        name: None,
        mcu,
        json_path,
        source,
    };
    check_with(project, &mut levels, input)
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
