//! `shorted_nets`: a trace must not tie differently named nets together.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Differently named declared nets merged by one trace: GND wired to 3V3
/// describes a dead short, however the netlist resolves it.
pub struct ShortedNetsRule;

const META: RuleMeta = RuleMeta {
    id: "shorted_nets",
    category: Category::Netlist,
    summary: "Two differently named nets must not be tied together by a trace.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for ShortedNetsRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        ctx.netlist
            .shorts()
            .iter()
            .map(|short| Diagnostic {
                rule: self.name().to_string(),
                message: format!(
                    "nets {} are shorted together through {}",
                    short.nets.join(", "),
                    short.trace
                ),
                help: Some(
                    "remove the trace that joins them, or rename the nets if they are meant to be one"
                        .to_string(),
                ),
                ..Default::default()
            })
            .collect()
    }
}
