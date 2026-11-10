//! `fiducials`: an assembled side carries global fiducials.
//!
//! The pick-and-place machine finds the board by two or three fiducial
//! marks. The rule runs only when the `fab:` block sets
//! `assembly_service: true`, and checks every side that has SMT pads.

use std::collections::BTreeSet;

use crate::linter::rules::dfm::{at, outline};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// The fewest global fiducials a side needs to be aligned.
const MIN_FIDUCIALS: usize = 2;

/// Reports an assembled side with fewer than two fiducials.
pub struct FiducialsRule;

const META: RuleMeta = RuleMeta {
    id: "fiducials",
    category: Category::Dfa,
    summary: "With assembly_service set, every side with SMT pads has at least two fiducials.",
    standard: Some("IPC-7351B"),
    source_url: None,
    default_severity: Severity::Warning,
};

fn fiducial_sides(ctx: &LintContext<'_>) -> Vec<String> {
    let kinds: BTreeSet<&str> = ctx
        .circuit
        .components()
        .filter(|c| c.kind.as_deref().is_some_and(|k| k.contains("fiducial")))
        .filter_map(|c| c.component_id.as_deref())
        .collect();
    let placed = ctx.circuit.model().components.iter();
    let fiducials = placed.filter(|c| kinds.contains(c.source_component_id.as_str()));
    fiducials.map(|c| c.layer.to_ascii_lowercase()).collect()
}

fn report(ctx: &LintContext<'_>, side: &str, count: usize) -> Diagnostic {
    let (x, y) = outline(ctx.circuit.model()).map_or((0.0, 0.0), |o| o.centre());
    Diagnostic {
        rule: META.id.to_string(),
        message: format!(
            "{side} side has SMT pads and {count} fiducial(s); the assembler needs at least {MIN_FIDUCIALS}"
        ),
        help: Some(format!(
            "add {MIN_FIDUCIALS} or 3 `<fiducial />` parts on the {side} side, far apart near the board corners"
        )),
        severity: META.default_severity,
        standard: META.standard.map(str::to_string),
        location: Some(at(x, y, Some(side))),
        ..Default::default()
    }
}

impl Rule for FiducialsRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&crate::config::DesignRules>) -> bool {
        design
            .and_then(|d| d.fab.as_ref())
            .is_some_and(|f| f.assembly_service)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        if !self.enabled(ctx.design) {
            return Vec::new();
        }
        let sides: BTreeSet<String> = ctx
            .circuit
            .pads()
            .map(|p| p.layer.as_deref().unwrap_or("top").to_ascii_lowercase())
            .collect();
        let marks = fiducial_sides(ctx);
        let count = |side: &str| marks.iter().filter(|m| *m == side).count();
        let short = sides.into_iter().filter(|s| count(s) < MIN_FIDUCIALS);
        short.map(|s| report(ctx, &s, count(&s))).collect()
    }
}
