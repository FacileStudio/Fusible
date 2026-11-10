//! `courtyard_clearance`: no two part courtyards share area on one side.
//!
//! tscircuit's own `courtyard_overlap` (an upstream id) compares unturned
//! rectangles. This rule turns each courtyard with its part first, so a
//! rotated part is checked at the size it really takes.

use std::collections::BTreeSet;

use super::courtyard::{courtyards, Courtyard};
use crate::linter::rules::dfm::{at, Owners};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Reports overlapping courtyards of distinct parts.
pub struct CourtyardClearanceRule;

const META: RuleMeta = RuleMeta {
    id: "courtyard_clearance",
    category: Category::Dfa,
    summary: "Part courtyards, turned with their parts, do not overlap on one side.",
    standard: Some("IPC-7351B"),
    source_url: None,
    default_severity: Severity::Error,
};

fn report(owners: &Owners<'_>, a: &Courtyard, b: &Courtyard) -> Diagnostic {
    let name = |c: &Courtyard| owners.of_component(&c.part).unwrap_or(&c.part).to_string();
    let (ca, cb) = (a.centre(), b.centre());
    Diagnostic {
        rule: META.id.to_string(),
        message: format!(
            "courtyards of {} and {} overlap on {}",
            name(a),
            name(b),
            a.layer
        ),
        help: Some("move or turn one of the parts so the courtyards no longer overlap".into()),
        severity: META.default_severity,
        standard: META.standard.map(str::to_string),
        elements: vec![a.part.clone(), b.part.clone()],
        location: Some(at((ca.0 + cb.0) / 2.0, (ca.1 + cb.1) / 2.0, Some(&a.layer))),
        ..Default::default()
    }
}

impl Rule for CourtyardClearanceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let all = courtyards(ctx.circuit.model());
        let owners = Owners::new(ctx);
        let mut seen = BTreeSet::new();
        let mut found = Vec::new();
        for (i, a) in all.iter().enumerate() {
            let others = all[i + 1..]
                .iter()
                .filter(|b| b.part != a.part && b.layer == a.layer);
            for b in others.filter(|b| a.overlaps(b)) {
                let key = (
                    a.part.clone().min(b.part.clone()),
                    a.part.clone().max(b.part.clone()),
                );
                if seen.insert(key) {
                    found.push(report(&owners, a, b));
                }
            }
        }
        found
    }
}
