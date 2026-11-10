//! `unrouted`: a board whose connections have no copper must blow the fuse.
//!
//! A router lays one tree of copper per net, so the check is per net: every
//! port of a net must sit on the same island of routed copper (traces, vias,
//! pads and pours of that net). Which `pcb_trace` carries which
//! `source_trace` id does not matter.

mod graph;
mod join;
mod nets;
mod pads;
mod pour;
mod scan;

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

use graph::Graph;
use nets::{labels, splits, Split};
use scan::is_unrouted;

const HELP: &str = "route the board; fab files from an unrouted board are unusable";
const UNROUTED: &str = "board has nets but no routed traces";

/// tscircuit keeps going when it skips or fails the autorouter. Without this
/// rule the fuse stays intact on a board whose copper was never routed.
pub struct UnroutedRule;

const META: RuleMeta = RuleMeta {
    id: "unrouted",
    category: Category::Drc,
    summary: "Blows the fuse when a board has nets but no routed trace, or copper does not reach every port of a net.",
    standard: Some("tscircuit"),
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for UnroutedRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        if is_unrouted(ctx) {
            return vec![self.diagnostic(UNROUTED.to_string(), Vec::new())];
        }
        if ctx.circuit.model().count("pcb_trace") == 0 {
            return Vec::new();
        }
        let names = labels(ctx);
        let found = splits(ctx, &mut Graph::new(ctx)).into_iter().map(|split| {
            let label = |id: &&str| names.get(*id).cloned().unwrap_or_else(|| id.to_string());
            let ports: Vec<String> = split.stranded.iter().map(label).collect();
            self.diagnostic(message(&split, &ports), ids(&split))
        });
        found.collect()
    }
}

fn ids(split: &Split<'_>) -> Vec<String> {
    split.stranded.iter().map(|id| id.to_string()).collect()
}

fn message(split: &Split<'_>, ports: &[String]) -> String {
    format!(
        "net \"{}\": routed copper does not reach {} ({} of {} ports)",
        split.net,
        ports.join(", "),
        ports.len(),
        split.total
    )
}

impl UnroutedRule {
    fn diagnostic(&self, message: String, elements: Vec<String>) -> Diagnostic {
        Diagnostic {
            rule: self.name().to_string(),
            message,
            help: Some(HELP.to_string()),
            elements,
            ..Default::default()
        }
    }
}
