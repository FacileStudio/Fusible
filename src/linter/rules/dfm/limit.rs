//! One fab limit read from the manifest, and the diagnostics of the rules
//! that read it.

use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::physics::PRECHECK;
use crate::linter::{Diagnostic, LintContext, Location, RuleMeta};

/// A board-level limit picker: the field of the profile a rule reads.
pub(in crate::linter::rules) type Pick = fn(&FabProfile) -> Option<f64>;

/// The manifest's fab profile, when one is declared.
pub(in crate::linter::rules) fn profile<'a>(ctx: &LintContext<'a>) -> Option<&'a FabProfile> {
    ctx.design.and_then(|d| d.fab.as_ref())
}

/// Whether the manifest declares a fab whose profile publishes `pick`.
pub(in crate::linter::rules) fn declared(design: Option<&DesignRules>, pick: Pick) -> bool {
    design
        .and_then(|d| d.fab.as_ref())
        .is_some_and(|fab| pick(fab).is_some())
}

/// What a fab-driven rule needs before it runs.
pub(in crate::linter::rules) struct Gate<'a> {
    pub meta: &'static RuleMeta,
    pub key: &'static str,
    pub pick: Pick,
    /// Whether the board holds anything this rule would check.
    pub relevant: bool,
    pub ctx: &'a LintContext<'a>,
}

impl Gate<'_> {
    /// Runs `scan` with the profile and its limit. Without a fab it is silent;
    /// with a profile that lacks the number it reports one `info` when the
    /// board has something to check.
    pub fn run(&self, scan: impl FnOnce(&FabProfile, f64) -> Vec<Diagnostic>) -> Vec<Diagnostic> {
        let Some(fab) = profile(self.ctx) else {
            return Vec::new();
        };
        match (self.pick)(fab) {
            Some(limit) => scan(fab, limit),
            None if self.relevant => vec![missing(self.meta, fab, self.key)],
            None => Vec::new(),
        }
    }
}

pub(in crate::linter::rules) fn missing(
    meta: &RuleMeta,
    fab: &FabProfile,
    key: &str,
) -> Diagnostic {
    Diagnostic {
        rule: meta.id.to_string(),
        message: format!(
            "{} is disabled: profile {} publishes no `{key}`",
            meta.id, fab.name
        ),
        help: Some(format!(
            "write the fab's own number as `{key}` in an inline `fab:` profile to enable it"
        )),
        severity: crate::linter::Severity::Info,
        standard: Some(fab.name.clone()),
        source_url: Some(fab.source.clone()).filter(|url| !url.is_empty()),
        ..Default::default()
    }
}

/// A violation of a fab limit at `location`.
pub(in crate::linter::rules) fn violation(
    meta: &RuleMeta,
    fab: &FabProfile,
    message: String,
    elements: Vec<String>,
    location: Location,
) -> Diagnostic {
    Diagnostic {
        rule: meta.id.to_string(),
        message,
        help: Some(format!(
            "Capability limits of {} as published by the fab. {PRECHECK}",
            fab.name
        )),
        severity: meta.default_severity,
        standard: Some(fab.name.clone()),
        source_url: Some(fab.source.clone()).filter(|url| !url.is_empty()),
        elements,
        location: Some(location),
        ..Default::default()
    }
}

/// A board location, on one layer when `layer` is set.
pub(crate) fn at(x: f64, y: f64, layer: Option<&str>) -> Location {
    Location {
        board: None,
        x,
        y,
        layer: layer.map(str::to_string),
    }
}
