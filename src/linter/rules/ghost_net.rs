//! `ghost_net`: every referenced net must be defined.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A `net_id` referenced by ports, pads, or traces but defined by no `source_net`.
pub struct GhostNetRule;

const META: RuleMeta = RuleMeta {
    id: "ghost_net",
    category: Category::Netlist,
    summary: "A port or pad refers to a net that no source net defines.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for GhostNetRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        ctx.netlist
            .ghost_nets()
            .map(|net| Diagnostic {
                rule: self.name().to_string(),
                message: format!(
                    "net \"{}\" is referenced by {} connection(s) but no source_net defines it",
                    net.id,
                    net.reference_count
                ),
                help: Some(
                    "add the missing source_net element, or fix the net_id reference on the port/pad"
                        .to_string(),
                ),
                ..Default::default()
            })
            .collect()
    }
}
