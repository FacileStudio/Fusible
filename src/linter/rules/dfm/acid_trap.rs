//! `acid_trap`: a trace that bends back on itself at an acute angle.
//!
//! Etchant pools in the narrow wedge of a bend under 90 degrees and can eat
//! the copper there. The rule reads no fab limit. A bend whose segments are
//! too short to open a wedge, a segment that runs back over the previous
//! one included, is not reported. Pad entries are not checked yet.

use crate::linter::rules::geom2d::Point;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::geometry::{read, Segment};

use super::limit::at;

const MIN_LENGTH: f64 = 1e-6;

/// Reports acute bends between consecutive segments of one trace.
pub struct AcidTrapRule;

const META: RuleMeta = RuleMeta {
    id: "acid_trap",
    category: Category::Dfm,
    summary: "Consecutive trace segments meet at 90 degrees or more, leaving no acid trap.",
    standard: None,
    source_url: None,
    default_severity: Severity::Warning,
};

fn sub(p: Point, q: Point) -> Point {
    (p.0 - q.0, p.1 - q.1)
}

/// The angle in degrees between `prev` and `next` at their shared point, or
/// `None` when they do not join, or when either is too short to open a
/// wedge: the copper fills the inner corner up to `half width / tan(angle / 2)`
/// from the vertex.
fn bend(prev: &Segment, next: &Segment) -> Option<f64> {
    let joined = prev.trace_id == next.trace_id
        && prev.layer == next.layer
        && sub(prev.b, next.a).0.hypot(sub(prev.b, next.a).1) < MIN_LENGTH;
    let (u, w) = (sub(prev.a, prev.b), sub(next.b, next.a));
    let (lu, lw) = (u.0.hypot(u.1), w.0.hypot(w.1));
    if !joined || lu < MIN_LENGTH || lw < MIN_LENGTH {
        return None;
    }
    let cos = ((u.0 * w.0 + u.1 * w.1) / (lu * lw)).clamp(-1.0, 1.0);
    let angle = cos.acos();
    let filled = prev.width.max(next.width) / 2.0 / (angle / 2.0).tan();
    (lu > filled && lw > filled).then(|| angle.to_degrees())
}

fn report(prev: &Segment, angle: f64) -> Diagnostic {
    let (x, y) = prev.b;
    Diagnostic {
        rule: META.id.to_string(),
        message: format!(
            "trace {} bends {angle:.0} degrees on {} at ({x:.2}, {y:.2}), an acid trap",
            prev.trace_id, prev.layer
        ),
        help: Some("reroute the bend at 90 degrees or more, e.g. two 45 degree turns".into()),
        severity: META.default_severity,
        elements: vec![prev.trace_id.clone()],
        location: Some(at(x, y, Some(&prev.layer))),
        ..Default::default()
    }
}

impl Rule for AcidTrapRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let geometry = read(ctx.circuit);
        let pairs = geometry.segments.windows(2);
        let acute = pairs.filter_map(|w| Some((&w[0], bend(&w[0], &w[1])?)));
        acute
            .filter(|(_, angle)| *angle < 90.0 - 1e-6)
            .map(|(prev, angle)| report(prev, angle))
            .collect()
    }
}
