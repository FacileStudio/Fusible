//! `unconnected_net`: a net must not be left floating.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Minimum number of endpoints for a net to count as connected.
const MIN_CONNECTIONS: usize = 2;

/// A net tied to fewer than two pads/ports is a routing omission: on the
/// real board that signal goes nowhere. The fuse blows before the bug can
/// reach the firmware.
pub struct UnconnectedNetRule;

const META: RuleMeta = RuleMeta {
    id: "unconnected_net",
    category: Category::Netlist,
    summary: "A net with fewer than two endpoints is floating.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for UnconnectedNetRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        ctx.netlist
            .floating_nets()
            .filter(|net| net.is_defined)
            .map(|net| Diagnostic {
                rule: self.name().to_string(),
                message: format!(
                    "net \"{}\" has only {} connection(s) (minimum {MIN_CONNECTIONS}): it is floating",
                    net.display_name(),
                    net.endpoints.len()
                ),
                help: Some(
                    "every net must connect at least two pads/ports — complete the routing in tscircuit"
                        .to_string(),
                ),
                ..Default::default()
            })
            .collect()
    }
}
