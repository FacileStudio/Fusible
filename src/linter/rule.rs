//! The `Rule` trait and the metadata every rule declares.

use serde::Serialize;

use super::{Diagnostic, LintContext, Severity};
use crate::config::DesignRules;

/// The family a rule belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// Connectivity of the netlist itself. Never turned off or waived.
    Netlist,
    /// Electrical rules on pins, rails and parts.
    Erc,
    /// Layout geometry: clearances, widths, currents.
    Drc,
    /// Design for manufacturing.
    Dfm,
    /// Design for assembly.
    Dfa,
    /// Pin assignments against the target MCU.
    Firmware,
    /// Bodies, enclosures and their fit.
    Mechanical,
    /// Errors tscircuit reported itself.
    Upstream,
}

/// What a rule is, declared once per rule as `const META: RuleMeta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleMeta {
    /// Stable snake_case id, reused as the diagnostic's error code.
    pub id: &'static str,
    /// The family the rule belongs to.
    pub category: Category,
    /// One sentence saying what the rule checks.
    pub summary: &'static str,
    /// The standard the rule's diagnostics cite, if it has a single one.
    pub standard: Option<&'static str>,
    /// Where to read that standard.
    pub source_url: Option<&'static str>,
    /// The highest severity the rule reports.
    pub default_severity: Severity,
}

/// One verification pass.
pub trait Rule {
    /// The rule's declared metadata.
    fn meta(&self) -> &'static RuleMeta;

    /// Short rule name, reused as the diagnostic's error code.
    fn name(&self) -> &'static str {
        self.meta().id
    }

    /// The highest severity this rule reports: an error-level diagnostic is
    /// lowered to it.
    fn severity(&self) -> Severity {
        self.meta().default_severity
    }

    /// One sentence saying what the rule checks.
    fn summary(&self) -> &'static str {
        self.meta().summary
    }

    /// The standard this rule's diagnostics cite, if it has a single one.
    fn standard(&self) -> Option<&'static str> {
        self.meta().standard
    }

    /// Where to read that standard.
    fn source_url(&self) -> Option<&'static str> {
        self.meta().source_url
    }

    /// Whether the manifest declares the data this rule needs. Rules that read
    /// no declaration are always enabled.
    fn enabled(&self, _design: Option<&DesignRules>) -> bool {
        true
    }

    /// Runs the rule against the lint context. An empty vector = rule passed.
    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic>;
}
