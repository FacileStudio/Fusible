//! `rail_budget`: the loads a `nets:` rail declares against what its source can deliver.
//!
//! ```yaml
//! nets:
//!   VDD3V3: { voltage: 3.3, limit: 0.6, loads: { U1: 0.35, MODEM1: 2.0 } }
//! ```

use super::{diagnostic, Board, Pin};
use crate::config::{DesignRules, NetSpec};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A rail whose declared loads exceed its declared source limit.
pub struct RailBudgetRule;

const META: RuleMeta = RuleMeta {
    id: "rail_budget",
    category: Category::Erc,
    summary: "The loads declared on a rail stay within the current its source can deliver.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

fn stray(net: &str, pins: &[&Pin<'_>], spec: &NetSpec) -> Vec<Diagnostic> {
    let present = |part: &str| pins.iter().any(|p| p.refdes() == part);
    spec.loads
        .keys()
        .filter(|part| !present(part))
        .map(|part| {
            let message = format!("`nets.{net}.loads` names {part}, which has no pin on \"{net}\"");
            let help = "name loads by the designator of a part on the rail, or remove the entry";
            diagnostic(&META, Severity::Error, message, help, vec![part.clone()])
        })
        .collect()
}

fn budget(net: &str, spec: &NetSpec) -> Option<Diagnostic> {
    let total: f64 = spec.loads.values().sum();
    let Some(limit) = spec.limit else {
        let message = format!(
            "\"{net}\" declares {total:.3} A of loads but no `limit` to check them against"
        );
        let help = "set `limit:` to the current the rail's regulator or source can deliver";
        return Some(diagnostic(&META, Severity::Info, message, help, Vec::new()));
    };
    if total <= limit {
        return None;
    }
    let parts: Vec<String> = spec
        .loads
        .iter()
        .map(|(p, a)| format!("{p} {a} A"))
        .collect();
    let message = format!(
        "\"{net}\" loads draw {total:.3} A ({}) but its source delivers {limit} A",
        parts.join(", ")
    );
    let help = "use a stronger regulator, split the rail, or lower the loads";
    let elements = spec.loads.keys().cloned().collect();
    Some(diagnostic(&META, Severity::Error, message, help, elements))
}

impl Rule for RailBudgetRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        let budgeted = |s: &NetSpec| s.limit.is_some() || !s.loads.is_empty();
        design.is_some_and(|d| d.nets.values().any(budgeted))
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let mut found = Vec::new();
        for (net, pins) in board.nets() {
            let Some(spec) = board.spec(net).filter(|s| !s.loads.is_empty()) else {
                continue;
            };
            let name = pins[0].net_name;
            found.extend(stray(name, &pins, spec));
            found.extend(budget(name, spec));
        }
        found
    }
}
