//! `drill_to_copper`: the drill bit keeps away from copper of other nets.
//!
//! A plated drill's own net is exempt; a non-plated drill has none, so every
//! piece of copper counts. A plated drill whose net cannot be resolved is
//! skipped, as `copper_drill` leaves it out of the copper set.

use std::collections::HashMap;

use super::drills::{drills, Drill, Kind};
use super::index::Index;
use super::limit::{at, declared, violation, Gate};
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::copper::{collect, Copper};
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::geometry::read;

/// Checks the drill edge to foreign copper distance of every hole.
pub struct DrillToCopperRule;

const META: RuleMeta = RuleMeta {
    id: "drill_to_copper",
    category: Category::Dfm,
    summary: "Every drill keeps the fab's minimum distance to copper of other nets.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

/// The closest foreign copper to `drill` under `min`: its index and gap.
fn closest(
    drill: &Drill,
    net: Option<&str>,
    items: &[Copper],
    index: &Index,
    min: f64,
) -> Option<(usize, f64)> {
    let foreign = index.near(&drill.shape, min).into_iter().filter(|&j| {
        let c = &items[j];
        c.id != drill.id && net != Some(c.net.as_str())
    });
    let gaps = foreign.map(|j| (j, drill.shape.gap(&items[j].shape)));
    gaps.filter(|(_, gap)| *gap < min - 1e-9)
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

/// The net a drill belongs to: `Some(None)` for a non-plated drill, `None`
/// when a plated drill's net is unknown.
fn drill_net<'a>(drill: &Drill, nets: &HashMap<&str, &'a str>) -> Option<Option<&'a str>> {
    match drill.kind {
        Kind::Unplated => Some(None),
        _ => nets.get(drill.id.as_str()).map(|net| Some(*net)),
    }
}

fn report(fab: &FabProfile, drill: &Drill, copper: &Copper, gap: f64, min: f64) -> Diagnostic {
    let message = format!(
        "{} {} drill is {} mm from {} ({}) on {}, below {} minimum {} mm",
        drill.kind.word(),
        drill.id,
        mm(gap),
        copper.id,
        copper.net,
        copper.layer,
        fab.name,
        mm(min)
    );
    let location = at(drill.at.0, drill.at.1, Some(&copper.layer));
    let elements = vec![drill.id.clone(), copper.id.clone()];
    violation(&META, fab, message, elements, location)
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let items = collect(ctx, &read(ctx.circuit));
    let nets: HashMap<&str, &str> = items
        .iter()
        .map(|c| (c.id.as_str(), c.net.as_str()))
        .collect();
    let index = Index::new(items.iter().map(|c| &c.shape));
    let found = drills(ctx.circuit.model()).into_iter().filter_map(|drill| {
        let net = drill_net(&drill, &nets)?;
        let (j, gap) = closest(&drill, net, &items, &index, min)?;
        Some(report(fab, &drill, &items[j], gap, min))
    });
    found.collect()
}

impl Rule for DrillToCopperRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_drill_to_copper)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let model = ctx.circuit.model();
        let gate = Gate {
            meta: &META,
            key: "min_drill_to_copper",
            pick: |f| f.min_drill_to_copper,
            relevant: !(model.vias.is_empty()
                && model.plated_holes.is_empty()
                && model.holes.is_empty()),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
