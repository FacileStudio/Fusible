//! `edge_clearance`: copper keeps away from the routed board edge.
//!
//! Traces, pads, via pads and plated-hole pads count. Copper pours are left
//! out: tscircuit clips them to their own `boardEdgeMargin`.

use std::collections::BTreeMap;

use super::limit::{at, declared, violation, Gate};
use super::outline::outline;
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::copper::{collect, Copper, Shape};
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::geometry::read;

/// Checks the distance from every piece of copper to the board outline.
pub struct EdgeClearanceRule;

const META: RuleMeta = RuleMeta {
    id: "edge_clearance",
    category: Category::Dfm,
    summary: "Copper keeps the fab's minimum distance to the routed board edge.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

fn anchor(copper: &Copper) -> (f64, f64) {
    match copper.shape {
        Shape::Seg { a, .. } => a,
        Shape::Rect { c, .. } => c,
    }
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let Some(board) = outline(ctx.circuit.model()) else {
        return Vec::new();
    };
    let items = collect(ctx, &read(ctx.circuit));
    let mut worst: BTreeMap<&str, (f64, &Copper)> = BTreeMap::new();
    for copper in &items {
        let gap = board.gap(&copper.shape);
        let closer = worst.get(copper.id.as_str()).is_none_or(|(w, _)| gap < *w);
        if gap < min - 1e-9 && closer {
            worst.insert(&copper.id, (gap, copper));
        }
    }
    let found = worst.into_values();
    found
        .map(|(gap, copper)| report(fab, copper, gap, min))
        .collect()
}

fn report(fab: &FabProfile, copper: &Copper, gap: f64, min: f64) -> Diagnostic {
    let message = format!(
        "{} ({}) on {} is {} mm from the board edge, below {} minimum {} mm",
        copper.id,
        copper.net,
        copper.layer,
        mm(gap),
        fab.name,
        mm(min)
    );
    let (x, y) = anchor(copper);
    let location = at(x, y, Some(&copper.layer));
    violation(&META, fab, message, vec![copper.id.clone()], location)
}

impl Rule for EdgeClearanceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_edge_clearance)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let gate = Gate {
            meta: &META,
            key: "min_edge_clearance",
            pick: |f| f.min_edge_clearance,
            relevant: !ctx.circuit.model().boards.is_empty(),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
