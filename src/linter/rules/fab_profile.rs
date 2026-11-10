//! `fab_profile`: the board stays inside what the chosen fab house can build.

mod holes;
mod layout;
mod spacing;

use std::collections::BTreeMap;

use super::physics::{mm, unreadable, PRECHECK};
use crate::config::{DesignRules, FabProfile};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::geometry::{read, Geometry};

const EPSILON: f64 = 1e-9;

/// Checks trace width, spacing, drills, vias and board size against the
/// manifest's `fab:` profile. It stays silent when no `fab:` is declared.
pub struct FabProfileRule;

pub(super) fn violation(
    profile: &FabProfile,
    message: String,
    elements: Vec<String>,
) -> Diagnostic {
    Diagnostic {
        rule: "fab_profile".to_string(),
        message,
        help: Some(format!(
            "Capability limits of {} as published by the fab. {PRECHECK}",
            profile.name
        )),
        standard: Some(profile.name.clone()),
        source_url: Some(profile.source.clone()).filter(|url| !url.is_empty()),
        elements,
        ..Default::default()
    }
}

fn narrow_traces(profile: &FabProfile, geometry: &Geometry) -> Vec<Diagnostic> {
    let mut narrowest: BTreeMap<(&str, &str), f64> = BTreeMap::new();
    for s in &geometry.segments {
        let width = narrowest.entry((&s.trace_id, &s.layer)).or_insert(s.width);
        *width = width.min(s.width);
    }
    let too_thin = narrowest
        .into_iter()
        .filter(|(_, w)| *w < profile.min_trace - EPSILON);
    too_thin
        .map(|((trace, layer), width)| {
            let message = format!(
                "trace {} mm on {layer} is below {} minimum {} mm",
                mm(width),
                profile.name,
                mm(profile.min_trace)
            );
            violation(profile, message, vec![trace.to_string()])
        })
        .collect()
}

const META: RuleMeta = RuleMeta {
    id: "fab_profile",
    category: Category::Dfm,
    summary:
        "Trace width, spacing, drills, vias and board size stay within the declared fab limits.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for FabProfileRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        design.is_some_and(|d| d.fab.is_some())
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let Some(profile) = ctx.design.and_then(|d| d.fab.as_ref()) else {
            return Vec::new();
        };
        let geometry = read(ctx.circuit);
        let found = narrow_traces(profile, &geometry)
            .into_iter()
            .chain(spacing::tight_spacing(ctx, profile, &geometry))
            .chain(holes::check(profile, &geometry));
        unreadable(self.name(), &geometry)
            .into_iter()
            .chain(found)
            .collect()
    }
}
