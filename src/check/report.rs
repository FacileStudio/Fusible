//! The verdict of one check run, version 2: what `--json`, the studio and MCP all read.

use serde::Serialize;

use crate::config::Waiver;
use crate::linter::{Diagnostic, Severity};

/// The `version` field of every report this build writes.
pub const REPORT_VERSION: u32 = 2;

/// Whether the design may go on to exports and codegen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fuse {
    /// No error-level diagnostic is left.
    #[default]
    Intact,
    /// At least one error-level diagnostic is left.
    Blown,
}

/// The diagnostics of one board, after rule levels and waivers.
#[derive(Debug, Serialize)]
pub struct BoardReport {
    /// Manifest name of the board, else the entry file stem.
    pub name: String,
    /// Every diagnostic still in force, errors and warnings alike.
    pub diagnostics: Vec<Diagnostic>,
}

/// A diagnostic a manifest waiver took out of the verdict.
#[derive(Debug, Serialize)]
pub struct WaivedDiagnostic {
    /// The board it came from, or `None` for links and assembly.
    pub board: Option<String>,
    /// The waived diagnostic, unchanged.
    pub diagnostic: Diagnostic,
    /// The waiver's reason.
    pub reason: String,
    /// The last day the waiver applies, `YYYY-MM-DD`.
    pub until: Option<String>,
}

/// A manifest waiver as the report lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WaiverEntry {
    /// The rule it waives.
    pub rule: String,
    /// The element id or `REFDES.PIN` it names.
    #[serde(rename = "match")]
    pub matches: String,
    /// Why the exception holds.
    pub reason: String,
    /// The last day it applies, `YYYY-MM-DD`.
    pub until: Option<String>,
}

impl From<&Waiver> for WaiverEntry {
    fn from(waiver: &Waiver) -> Self {
        Self {
            rule: waiver.rule.clone(),
            matches: waiver.matches.clone(),
            reason: waiver.reason.clone(),
            until: waiver.until.map(|d| d.to_string()),
        }
    }
}

/// Everything one `check` found, across boards, links and assembly.
#[derive(Debug, Serialize)]
pub struct Report {
    /// Always [`REPORT_VERSION`].
    pub version: u32,
    /// Blown when any board, link or assembly diagnostic is an error.
    pub fuse: Fuse,
    /// One entry per checked board, in check order.
    pub boards: Vec<BoardReport>,
    /// `harness_mismatch` diagnostics of the manifest's `links:`.
    pub links: Vec<Diagnostic>,
    /// Assembly and body diagnostics, run only once every board and link passed.
    pub assembly: Vec<Diagnostic>,
    /// Diagnostics an active waiver took out.
    pub waived: Vec<WaivedDiagnostic>,
    /// One `unused_waiver` warning per active waiver that matched nothing.
    pub unused_waivers: Vec<Diagnostic>,
    /// Waivers past their `until` day; they no longer apply.
    pub expired_waivers: Vec<WaiverEntry>,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            version: REPORT_VERSION,
            fuse: Fuse::Intact,
            boards: Vec::new(),
            links: Vec::new(),
            assembly: Vec::new(),
            waived: Vec::new(),
            unused_waivers: Vec::new(),
            expired_waivers: Vec::new(),
        }
    }
}

impl Report {
    /// Whether any board, link or assembly diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        let boards = self.boards.iter().flat_map(|b| &b.diagnostics);
        boards
            .chain(&self.links)
            .chain(&self.assembly)
            .any(|d| d.severity == Severity::Error)
    }

    /// Sets `fuse` from the diagnostics.
    pub(super) fn seal(&mut self) {
        self.fuse = match self.has_errors() {
            true => Fuse::Blown,
            false => Fuse::Intact,
        };
    }
}
