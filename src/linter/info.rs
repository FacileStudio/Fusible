//! A description of one registered rule, served at `/api/rules`.

use serde::Serialize;

use super::{Category, Linter, Rule, Severity};
use crate::config::DesignRules;

/// What a rule checks, which standard backs it, and whether the manifest feeds it.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RuleInfo {
    pub id: &'static str,
    pub category: Category,
    pub standard: Option<&'static str>,
    pub source_url: Option<&'static str>,
    pub severity: &'static str,
    pub enabled: bool,
    pub summary: &'static str,
}

impl Linter {
    /// Describes every registered rule, in registration order. `design` decides
    /// which rules have the declarations they need.
    pub fn describe(&self, design: Option<&DesignRules>) -> Vec<RuleInfo> {
        let info = |rule: &dyn Rule| RuleInfo {
            id: rule.name(),
            category: rule.meta().category,
            standard: rule.standard(),
            source_url: rule.source_url(),
            severity: match rule.severity() {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
            },
            enabled: rule.enabled(design),
            summary: rule.summary(),
        };
        self.rules.iter().map(|rule| info(rule.as_ref())).collect()
    }
}
