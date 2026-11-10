//! `via_aspect_ratio`: board thickness over drill width for every plated drill.
//!
//! Plating fails in a hole that is too deep for its width. The board
//! thickness defaults to 1.6 mm, as in the geometry reader.

use super::drills::{drills, Kind};
use super::limit::{at, declared, violation, Gate};
use crate::config::{DesignRules, FabProfile};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

const DEFAULT_THICKNESS: f64 = 1.6;

/// Checks the depth to width ratio of vias and plated holes.
pub struct ViaAspectRatioRule;

const META: RuleMeta = RuleMeta {
    id: "via_aspect_ratio",
    category: Category::Dfm,
    summary: "Plated drills stay within the fab's maximum board thickness to drill ratio.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, max: f64) -> Vec<Diagnostic> {
    let model = ctx.circuit.model();
    let thickness = model
        .boards
        .first()
        .and_then(|b| b.thickness)
        .unwrap_or(DEFAULT_THICKNESS);
    let plated = drills(model)
        .into_iter()
        .filter(|d| d.kind != Kind::Unplated);
    let too_deep = plated.filter(|d| d.width > 0.0 && thickness / d.width > max + 1e-9);
    too_deep
        .map(|d| {
            let message = format!(
                "{} {} is {:.1}:1 ({} mm board, {} mm drill), above {} maximum {max}:1",
                d.kind.word(),
                d.id,
                thickness / d.width,
                thickness,
                d.width,
                fab.name
            );
            violation(
                &META,
                fab,
                message,
                vec![d.id.clone()],
                at(d.at.0, d.at.1, None),
            )
        })
        .collect()
}

impl Rule for ViaAspectRatioRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.max_aspect_ratio)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let model = ctx.circuit.model();
        let gate = Gate {
            meta: &META,
            key: "max_aspect_ratio",
            pick: |f| f.max_aspect_ratio,
            relevant: !(model.vias.is_empty() && model.plated_holes.is_empty()),
            ctx,
        };
        gate.run(|fab, max| scan(ctx, fab, max))
    }
}
