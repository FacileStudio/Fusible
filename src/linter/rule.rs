//! The `Rule` trait: one verification pass.

use super::{summaries, Diagnostic, LintContext, Severity};
use crate::config::DesignRules;

/// One verification pass.
pub trait Rule {
    /// Short rule name, reused as the diagnostic's error code.
    fn name(&self) -> &'static str;

    /// Whether this rule blows the fuse or only warns. Errors by default.
    fn severity(&self) -> Severity {
        Severity::Error
    }

    /// One sentence saying what the rule checks.
    fn summary(&self) -> &'static str {
        summaries::summary_of(self.name())
    }

    /// The standard this rule's diagnostics cite, if it has a single one.
    fn standard(&self) -> Option<&'static str> {
        None
    }

    /// Where to read that standard.
    fn source_url(&self) -> Option<&'static str> {
        None
    }

    /// Whether the manifest declares the data this rule needs. Rules that read
    /// no declaration are always enabled.
    fn enabled(&self, _design: Option<&DesignRules>) -> bool {
        true
    }

    /// Runs the rule against the lint context. An empty vector = rule passed.
    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic>;
}
