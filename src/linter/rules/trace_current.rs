//! `trace_current`: a declared net must carry its current without overheating (IPC-2152).

mod report;

use super::ipc2152::rise;
use super::physics::{is_external, unreadable};
use crate::config::DesignRules;
use crate::linter::{Diagnostic, LintContext, Rule, Severity};
use crate::parser::geometry::{read, Segment};
use report::{diagnostic, violation};

const DEFAULT_MAX_RISE: f64 = 10.0;
const STANDARD: &str = "IPC-2152";
const SOURCE: &str =
    "https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html";

/// Estimates the temperature rise of every net that declares a `current` and
/// blows the fuse when it exceeds the net's `max_rise` (10 degrees by default).
pub struct TraceCurrentRule;

pub(super) struct Load<'a> {
    net: &'a str,
    amps: f64,
    max_rise: f64,
}

pub(super) struct Hottest<'a> {
    segment: &'a Segment,
    rise: f64,
    oz: f64,
}

fn loads(design: &DesignRules) -> Vec<Load<'_>> {
    let declared = design.nets.iter();
    let with_current = declared.filter_map(|(net, spec)| Some((net, spec, spec.current?)));
    with_current
        .map(|(net, spec, amps)| Load {
            net,
            amps,
            max_rise: spec.max_rise.unwrap_or(DEFAULT_MAX_RISE),
        })
        .collect()
}

fn weight(design: &DesignRules, layer: &str) -> f64 {
    match is_external(layer) {
        true => design.copper.outer_oz,
        false => design.copper.inner_oz,
    }
}

impl TraceCurrentRule {
    fn check_net(
        design: &DesignRules,
        load: &Load<'_>,
        segments: &[&Segment],
    ) -> Option<Diagnostic> {
        if segments.is_empty() {
            return Some(diagnostic(
                format!(
                    "{} declares {} A but has no routed traces to check",
                    load.net, load.amps
                ),
                Severity::Warning,
            ));
        }
        let all: Vec<Hottest<'_>> = segments
            .iter()
            .map(|&segment| {
                let oz = weight(design, &segment.layer);
                let rise = rise(load.amps, segment.width, !is_external(&segment.layer), oz);
                Hottest { segment, rise, oz }
            })
            .collect();
        let hottest = all.iter().max_by(|x, y| x.rise.total_cmp(&y.rise))?;
        (hottest.rise > load.max_rise).then(|| violation(load, hottest, &all))
    }
}

impl Rule for TraceCurrentRule {
    fn name(&self) -> &'static str {
        "trace_current"
    }

    fn standard(&self) -> Option<&'static str> {
        Some(STANDARD)
    }

    fn source_url(&self) -> Option<&'static str> {
        Some(SOURCE)
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        design.is_some_and(|d| !loads(d).is_empty())
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let Some(design) = ctx.design else {
            return Vec::new();
        };
        let loads = loads(design);
        if loads.is_empty() {
            return Vec::new();
        }
        let geometry = read(ctx.circuit);
        let declared = loads
            .iter()
            .filter(|l| ctx.circuit.nets().any(|n| n.name.as_deref() == Some(l.net)));
        let found = declared.filter_map(|load| {
            let on_net: Vec<&Segment> = geometry
                .segments
                .iter()
                .filter(|s| s.net_names.iter().any(|n| n == load.net))
                .collect();
            Self::check_net(design, load, &on_net)
        });
        unreadable(self.name(), &geometry)
            .into_iter()
            .chain(found)
            .collect()
    }
}
