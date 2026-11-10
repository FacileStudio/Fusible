//! A description of one registered rule, served at `/api/rules`.

use serde::Serialize;

use super::{Category, Linter, RuleMeta, Severity, CROSS_WORLD};
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

fn info(meta: &'static RuleMeta, enabled: bool) -> RuleInfo {
    RuleInfo {
        id: meta.id,
        category: meta.category,
        standard: meta.standard,
        source_url: meta.source_url,
        severity: match meta.default_severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        },
        enabled,
        summary: meta.summary,
    }
}

impl Linter {
    /// Describes every id the registered rules report under, in
    /// registration order: each rule's own id, then the ids it
    /// [`super::Rule::emits`]. `design` decides which rules have the
    /// declarations they need.
    pub fn describe(&self, design: Option<&DesignRules>) -> Vec<RuleInfo> {
        let rules = self.rules.iter().map(|rule| rule.as_ref());
        rules
            .flat_map(|rule| {
                let metas = std::iter::once(rule.meta()).chain(rule.emits().iter().copied());
                metas.map(move |meta| info(meta, rule.enabled(design)))
            })
            .collect()
    }
}

/// Every rule a manifest may name, as `fusible rules`, `fusible explain` and
/// `/api/rules` list them: the board rules of the default linter, then the
/// cross-world rules of [`CROSS_WORLD`]. Those run on `links:`, `assembly:`
/// and `bodies:`, not on design declarations, so they are always enabled.
pub fn catalog(design: Option<&DesignRules>) -> Vec<RuleInfo> {
    let board = Linter::default().describe(design);
    let cross = CROSS_WORLD.iter().map(|meta| info(meta, true));
    board.into_iter().chain(cross).collect()
}
