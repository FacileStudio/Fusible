//! Linter diagnostics and their miette rendering.
//!
//! Two levels:
//! * [`Diagnostic`] — one individual violation (the *where* and *why*);
//! * [`FuseBlown`] — the aggregate error `main` renders when the linter
//!   found at least one violation: a title with the violations as *related
//!   errors*, then exit code 1 so codegen never runs.

use std::fmt;

use serde::Serialize;
use thiserror::Error;

/// How a violation affects the fuse: errors blow it, warnings and infos only
/// print. Variants are ordered from most to least severe, so `max` picks the
/// milder of two levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Info,
}

/// Where a violation sits on a board, in millimetres.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Location {
    /// Board name from the manifest, when the check knows it.
    pub board: Option<String>,
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Copper layer, e.g. `top`, when the violation sits on one layer.
    pub layer: Option<String>,
}

/// The source line that declares the offending element.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceRef {
    /// Path of the source file.
    pub file: String,
    /// 1-based line, when known.
    pub line: Option<u32>,
}

/// A machine-applicable change that clears a violation.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fix {
    /// Set `prop` to `value` on the element matched by `selector`.
    SetProp {
        selector: String,
        prop: String,
        value: serde_json::Value,
    },
    /// Add this TSX element to the board.
    AddElement { tsx: String },
    /// A change a human has to make.
    Note(String),
}

/// A violation detected by a rule.
///
/// `miette::Diagnostic` is implemented **by hand** instead of derived: the
/// `code` (rendered as `error[unconnected_net]`) and the `help` depend on
/// which rule fired. `std::error::Error` is also written by hand, because
/// thiserror would treat the `source` field as the error's cause.
#[derive(Debug, Default, Serialize)]
pub struct Diagnostic {
    /// Stable rule identifier, e.g. `unconnected_net`.
    pub rule: String,
    /// Compiler-style message: what broke and why.
    pub message: String,
    /// Contextual help rendered as `help: ...` under the diagnostic.
    pub help: Option<String>,
    /// Set by the linter from the rule that produced this diagnostic.
    pub severity: Severity,
    /// Standard or guideline the check comes from, e.g. `IPC-2152`.
    pub standard: Option<String>,
    /// Where to read that standard.
    pub source_url: Option<String>,
    /// Ids of the circuit elements involved.
    pub elements: Vec<String>,
    /// A concrete change that clears the violation.
    pub fix: Option<String>,
    /// Where the violation sits on the board.
    pub location: Option<Location>,
    /// The source line that declares the offending element.
    pub source: Option<SourceRef>,
    /// Machine-applicable changes that clear the violation.
    pub fixes: Vec<Fix>,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Diagnostic {}

impl miette::Diagnostic for Diagnostic {
    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new(self.rule.as_str()))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warning => miette::Severity::Warning,
            Severity::Info => miette::Severity::Advice,
        })
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        self.help
            .as_deref()
            .map(|help| Box::new(help) as Box<dyn fmt::Display>)
    }

    fn url(&self) -> Option<Box<dyn fmt::Display + '_>> {
        self.source_url
            .as_deref()
            .map(|url| Box::new(url) as Box<dyn fmt::Display>)
    }
}

/// Aggregates every violation into a single error report.
///
/// Returned by `main` when the linter found something: miette renders this
/// title, then each `Diagnostic` as a related error, and the process exits
/// with code 1 — which is what blocks the generation chain (`codegen::` is
/// never called).
#[derive(Debug, Error)]
#[error("the fuse blew: {} electrical violation(s)", .violations.len())]
pub struct FuseBlown {
    pub violations: Vec<Diagnostic>,
}

impl FuseBlown {
    /// Wraps the violations that blew the fuse.
    pub fn new(violations: Vec<Diagnostic>) -> Self {
        Self { violations }
    }
}

impl miette::Diagnostic for FuseBlown {
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        Some(Box::new(
            self.violations
                .iter()
                .map(|diagnostic| diagnostic as &dyn miette::Diagnostic),
        ))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new(
            "fix the violations above: generation stays blocked while the \
             fuse is blown",
        ))
    }
}
