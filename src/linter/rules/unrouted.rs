//! `unrouted`: a board whose connections have no copper must blow the fuse.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

use super::unrouted_scan::{is_unrouted, unrouted_traces};

const HELP: &str = "route the board; fab files from an unrouted board are unusable";
const UNROUTED: &str = "board has nets but no routed traces";

/// tscircuit keeps going when it skips or fails the autorouter. Without this
/// rule the fuse stays intact on a board whose copper was never routed.
pub struct UnroutedRule;

const META: RuleMeta = RuleMeta {
    id: "unrouted",
    category: Category::Drc,
    summary: "Blows the fuse when a board has nets but no routed trace, or a source_trace has no pcb_trace.",
    standard: Some("tscircuit"),
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for UnroutedRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let mut found = Vec::new();
        if is_unrouted(ctx) {
            found.push(self.diagnostic(UNROUTED.to_string()));
        }
        let missing = unrouted_traces(ctx);
        if !missing.is_empty() {
            let message = format!(
                "source_trace without a routed pcb_trace: {}",
                missing.join(", ")
            );
            found.push(self.diagnostic(message));
        }
        found
    }
}

impl UnroutedRule {
    fn diagnostic(&self, message: String) -> Diagnostic {
        Diagnostic {
            rule: self.name().to_string(),
            message,
            help: Some(HELP.to_string()),
            ..Default::default()
        }
    }
}
