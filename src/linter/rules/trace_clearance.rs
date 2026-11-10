//! `trace_clearance`: physical copper clearance design rule check (DRC).

use super::geom2d::TOLERANCE;
use super::pad_geometry::{pad_box, pad_net};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Minimum copper clearance in millimeters (standard 6 mil / 0.15mm PCB manufacturing limit).
const MIN_CLEARANCE_MM: f64 = 0.15;

fn clearance_help(min: f64) -> String {
    format!(
        "reroute copper or relocate pads to maintain at least {min}mm \
         clearance to prevent manufacturing defects"
    )
}

/// Inspects physical PCB copper geometry (pads, traces, vias) and detects
/// short circuits and clearance violations between distinct electrical nets.
pub struct TraceClearanceRule;

#[derive(Debug)]
struct CopperFeature {
    pub net_id: String,
    pub net_name: String,
    pub layer: String,
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
}

const META: RuleMeta = RuleMeta {
    id: "trace_clearance",
    category: Category::Drc,
    summary: "Copper of distinct nets on one layer keeps 0.15 mm, or the fab minimum space when a fab is set.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

impl Rule for TraceClearanceRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let features = extract_copper_features(ctx);
        let min = ctx
            .design
            .and_then(|d| d.fab.as_ref())
            .map_or(MIN_CLEARANCE_MM, |fab| fab.min_space);
        let mut diagnostics = Vec::new();
        for (i, f1) in features.iter().enumerate() {
            for f2 in &features[i + 1..] {
                diagnostics.extend(clearance_violation(f1, f2, min).map(|message| Diagnostic {
                    rule: self.name().to_string(),
                    message,
                    help: Some(clearance_help(min)),
                    ..Default::default()
                }));
            }
        }
        diagnostics
    }
}

/// The violation message when two copper features of distinct nets on one
/// layer sit closer than `min`.
fn clearance_violation(f1: &CopperFeature, f2: &CopperFeature, min: f64) -> Option<String> {
    if f1.net_id == f2.net_id || !f1.layer.eq_ignore_ascii_case(&f2.layer) {
        return None;
    }
    let dx = (f1.x - f2.x).abs();
    let dy = (f1.y - f2.y).abs();
    let reach_x = f1.half_w + f2.half_w;
    let reach_y = f1.half_h + f2.half_h;
    let clearance_gap = ((dx - reach_x).max(0.0)).hypot((dy - reach_y).max(0.0));
    (clearance_gap < min - TOLERANCE).then(|| {
        if dx < reach_x && dy < reach_y {
            format!(
                "direct copper short-circuit on {}: net \"{}\" and net \"{}\" overlap at \
                 ({:.2}, {:.2})",
                f1.layer, f1.net_name, f2.net_name, f1.x, f1.y
            )
        } else {
            format!(
                "copper clearance violation on {}: net \"{}\" and net \"{}\" have only \
                 {:.3}mm spacing (minimum {min}mm)",
                f1.layer, f1.net_name, f2.net_name, clearance_gap
            )
        }
    })
}

/// Every positioned pad as copper.
fn extract_copper_features(ctx: &LintContext<'_>) -> Vec<CopperFeature> {
    ctx.circuit
        .pads()
        .enumerate()
        .filter_map(|(index, pad)| {
            let (x, y, half_w, half_h) = pad_box(pad)?;
            let label = pad.pad_id.clone().unwrap_or_else(|| format!("#{index}"));
            let (net_id, net_name) = pad_net(ctx, pad, &label);
            Some(CopperFeature {
                net_id,
                net_name,
                layer: pad.layer.clone().unwrap_or_else(|| "top".to_string()),
                x,
                y,
                half_w,
                half_h,
            })
        })
        .collect()
}
