//! `voltage_clearance`: copper at different voltages keeps the IPC-2221B spacing.

mod pairs;

use super::copper::{collect, Copper};
use super::physics::{unreadable, PRECHECK};
use crate::config::DesignRules;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::geometry::read;
use pairs::{voltage, worst_pairs, Worst};

const STANDARD: &str = "IPC-2221B";
const SOURCE: &str = "https://www.atlaspcb.com/tools/conductor-spacing-calculator";

/// Compares every pair of nets with known voltages on one layer and blows the
/// fuse when their copper is closer than IPC-2221B Table 6-1 asks.
pub struct VoltageClearanceRule;

fn label(design: &DesignRules, net: &str, volts: f64) -> String {
    match design.nets.get(net).and_then(|spec| spec.voltage) {
        Some(_) => format!("{net} ({volts} V)"),
        None => net.to_string(),
    }
}

fn violation(design: &DesignRules, w: Worst) -> Diagnostic {
    Diagnostic {
        rule: "voltage_clearance".to_string(),
        message: format!(
            "{} and {} are {:.2} mm apart on {}; IPC-2221B {} asks at least {:.2} mm",
            label(design, &w.high.0, w.high.1),
            label(design, &w.low.0, w.low.1),
            w.gap,
            w.layer,
            w.class,
            w.required
        ),
        help: Some(format!(
            "IPC-2221B Table 6-1 clearance. {PRECHECK} Safety-rated products also need IEC 62368-1 \
             creepage, which Fusible does not check yet."
        )),
        standard: Some(STANDARD.to_string()),
        source_url: Some(SOURCE.to_string()),
        elements: w.elements,
        fix: Some(format!(
            "move the copper at least {:.2} mm apart",
            w.required
        )),
        ..Default::default()
    }
}

const META: RuleMeta = RuleMeta {
    id: "voltage_clearance",
    category: Category::Drc,
    summary: "Copper at different declared voltages keeps the IPC-2221B Table 6-1 spacing.",
    standard: Some(STANDARD),
    source_url: Some(SOURCE),
    default_severity: Severity::Error,
};

impl Rule for VoltageClearanceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        design.is_some_and(|d| d.nets.values().any(|spec| spec.voltage.is_some()))
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let Some(design) = ctx.design.filter(|d| self.enabled(Some(d))) else {
            return Vec::new();
        };
        let geometry = read(ctx.circuit);
        let items: Vec<Copper> = collect(ctx, &geometry)
            .into_iter()
            .filter(|item| voltage(design, &item.net).is_some())
            .collect();
        let found = worst_pairs(design, &items)
            .into_iter()
            .map(|worst| violation(design, worst));
        unreadable(self.name(), &geometry)
            .into_iter()
            .chain(found)
            .collect()
    }
}
