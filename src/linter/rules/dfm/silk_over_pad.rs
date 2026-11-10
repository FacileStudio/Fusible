//! `silk_over_pad`: silkscreen keeps off pad copper.
//!
//! Ink on a pad spoils the solder joint; most fabs clip it, so the legend
//! printed is not the legend drawn. One warning per silkscreen element.

use std::collections::BTreeMap;

use super::index::Index;
use super::limit::{at, declared, violation, Gate};
use super::silk::{pads, silk, Item};
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::copper::Shape;
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Checks the distance from silkscreen strokes and text to pads.
pub struct SilkOverPadRule;

const META: RuleMeta = RuleMeta {
    id: "silk_over_pad",
    category: Category::Dfm,
    summary: "Silkscreen keeps the fab's minimum distance to pad copper.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Warning,
};

fn anchor(shape: &Shape) -> (f64, f64) {
    match shape {
        Shape::Seg { a, .. } => *a,
        Shape::Rect { c, .. } => *c,
    }
}

fn report(fab: &FabProfile, ink: &Item, pad: &Item, gap: f64, min: f64) -> Diagnostic {
    let message = format!(
        "silkscreen {} on {} is {} mm from pad {}, below {} minimum {} mm",
        ink.id,
        ink.layer,
        mm(gap),
        pad.id,
        fab.name,
        mm(min)
    );
    let (x, y) = anchor(&ink.shape);
    let location = at(x, y, Some(&ink.layer));
    violation(
        &META,
        fab,
        message,
        vec![ink.id.clone(), pad.id.clone()],
        location,
    )
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let copper = pads(ctx);
    let index = Index::new(copper.iter().map(|p| &p.shape));
    let mut worst: BTreeMap<String, (f64, Item, usize)> = BTreeMap::new();
    for ink in silk(ctx.circuit.model()) {
        let same_side = index
            .near(&ink.shape, min)
            .into_iter()
            .filter(|&j| copper[j].layer == ink.layer);
        for j in same_side {
            let gap = ink.shape.gap(&copper[j].shape);
            let closer = worst.get(&ink.id).is_none_or(|w| gap < w.0);
            if gap < min - 1e-9 && closer {
                worst.insert(ink.id.clone(), (gap, ink.clone(), j));
            }
        }
    }
    let found = worst.into_values();
    found
        .map(|(gap, ink, j)| report(fab, &ink, &copper[j], gap, min))
        .collect()
}

impl Rule for SilkOverPadRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_silk_to_pad)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let model = ctx.circuit.model();
        let gate = Gate {
            meta: &META,
            key: "min_silk_to_pad",
            pick: |f| f.min_silk_to_pad,
            relevant: !(model.silkscreen_paths.is_empty() && model.silkscreen_texts.is_empty()),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
