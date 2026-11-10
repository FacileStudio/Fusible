//! `hole_spacing`: drill edge to drill edge between holes.
//!
//! Pairs of two vias are left to `fab_profile`'s via-to-via limit; every
//! pair with at least one plated or non-plated hole is held to
//! `min_hole_spacing`.

use super::drills::{drills, Drill, Kind};
use super::index::Index;
use super::limit::{at, declared, violation, Gate};
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Checks the web between neighbouring drills.
pub struct HoleSpacingRule;

const META: RuleMeta = RuleMeta {
    id: "hole_spacing",
    category: Category::Dfm,
    summary: "Holes keep the fab's minimum drill edge to drill edge spacing.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

fn report(fab: &FabProfile, a: &Drill, b: &Drill, gap: f64, min: f64) -> Diagnostic {
    let message = format!(
        "{} {} and {} {} are {} mm apart, below {} minimum {} mm",
        a.kind.word(),
        a.id,
        b.kind.word(),
        b.id,
        mm(gap),
        fab.name,
        mm(min)
    );
    let location = at((a.at.0 + b.at.0) / 2.0, (a.at.1 + b.at.1) / 2.0, None);
    violation(
        &META,
        fab,
        message,
        vec![a.id.clone(), b.id.clone()],
        location,
    )
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let all = drills(ctx.circuit.model());
    let index = Index::new(all.iter().map(|d| &d.shape));
    let mut found = Vec::new();
    for (i, a) in all.iter().enumerate() {
        let later = index.near(&a.shape, min).into_iter().filter(|&j| j > i);
        for b in later.map(|j| &all[j]) {
            let gap = a.shape.gap(&b.shape);
            let vias = a.kind == Kind::Via && b.kind == Kind::Via;
            if !vias && gap < min - 1e-9 {
                found.push(report(fab, a, b, gap, min));
            }
        }
    }
    found
}

impl Rule for HoleSpacingRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_hole_spacing)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let model = ctx.circuit.model();
        let gate = Gate {
            meta: &META,
            key: "min_hole_spacing",
            pick: |f| f.min_hole_spacing,
            relevant: !(model.plated_holes.is_empty() && model.holes.is_empty()),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
