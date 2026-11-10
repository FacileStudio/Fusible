//! `dangling_reference`: every referenced port and PCB port must exist.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A trace, pad, or PCB port pointing at an element nobody defines. The
/// netlist adds no endpoint for it, so without this rule the missing copper
/// would vanish silently.
pub struct DanglingReferenceRule;

const META: RuleMeta = RuleMeta {
    id: "dangling_reference",
    category: Category::Netlist,
    summary: "A trace or port points at an element that does not exist.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for DanglingReferenceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        ctx.netlist
            .dangling_refs()
            .iter()
            .map(|r| Diagnostic {
                rule: self.name().to_string(),
                message: format!("{} references {}, which does not exist", r.owner, r.target),
                help: Some(
                    "add the missing element, or fix the reference: its connection cannot be verified"
                        .to_string(),
                ),
                ..Default::default()
            })
            .collect()
    }
}
