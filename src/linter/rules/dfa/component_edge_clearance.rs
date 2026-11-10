//! `component_edge_clearance`: part courtyards keep away from the board edge.
//!
//! Assembly rails and depaneling need room at the edge. Parts marked
//! `is_allowed_to_be_off_board` (edge connectors) are exempt. A part without
//! a courtyard is not checked.

use std::collections::{BTreeMap, HashSet};

use super::courtyard::{courtyards, Courtyard};
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::copper::Shape;
use crate::linter::rules::dfm::{at, declared, outline, violation, Gate, Outline, Owners};
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Checks the distance from every courtyard to the board outline.
pub struct ComponentEdgeClearanceRule;

const META: RuleMeta = RuleMeta {
    id: "component_edge_clearance",
    category: Category::Dfa,
    summary: "Part courtyards keep the fab's minimum distance to the board edge.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Warning,
};

fn gap(board: &Outline, courtyard: &Courtyard) -> f64 {
    let sides = courtyard.sides().into_iter();
    let edge = |(a, b)| board.gap(&Shape::Seg { a, b, half: 0.0 });
    sides.map(edge).fold(f64::MAX, f64::min)
}

fn report(fab: &FabProfile, owners: &Owners<'_>, c: &Courtyard, gap: f64, min: f64) -> Diagnostic {
    let name = owners.of_component(&c.part).unwrap_or(&c.part);
    let message = format!(
        "{name} is {} mm from the board edge, below {} minimum {} mm",
        mm(gap),
        fab.name,
        mm(min)
    );
    let (x, y) = c.centre();
    violation(
        &META,
        fab,
        message,
        vec![c.part.clone()],
        at(x, y, Some(&c.layer)),
    )
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let model = ctx.circuit.model();
    let Some(board) = outline(model) else {
        return Vec::new();
    };
    let exempt: HashSet<&str> = model
        .components
        .iter()
        .filter(|c| c.is_allowed_to_be_off_board == Some(true))
        .map(|c| c.pcb_component_id.as_str())
        .collect();
    let mut worst: BTreeMap<String, (f64, Courtyard)> = BTreeMap::new();
    for c in courtyards(model)
        .into_iter()
        .filter(|c| !exempt.contains(c.part.as_str()))
    {
        let g = gap(&board, &c);
        if g < min - 1e-9 && worst.get(&c.part).is_none_or(|w| g < w.0) {
            worst.insert(c.part.clone(), (g, c));
        }
    }
    let owners = Owners::new(ctx);
    let found = worst.into_values();
    found
        .map(|(g, c)| report(fab, &owners, &c, g, min))
        .collect()
}

impl Rule for ComponentEdgeClearanceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_component_to_edge)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let model = ctx.circuit.model();
        let gate = Gate {
            meta: &META,
            key: "min_component_to_edge",
            pick: |f| f.min_component_to_edge,
            relevant: !model.boards.is_empty() && !model.components.is_empty(),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
