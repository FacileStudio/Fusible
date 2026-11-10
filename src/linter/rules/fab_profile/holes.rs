//! Drill, via and board-size limits of a fab profile.

use super::layout;
use super::{violation, EPSILON};
use crate::config::FabProfile;
use crate::linter::rules::physics::mm;
use crate::linter::Diagnostic;
use crate::parser::geometry::Geometry;

struct Drill<'a> {
    id: &'a str,
    kind: &'static str,
    hole: f64,
    outer: Option<f64>,
}

fn drills(geometry: &Geometry) -> Vec<Drill<'_>> {
    let vias = geometry.vias.iter().map(|v| Drill {
        id: &v.id,
        kind: "via",
        hole: v.hole,
        outer: Some(v.outer),
    });
    let holes = geometry.holes.iter().map(|h| Drill {
        id: &h.id,
        kind: "hole",
        hole: h.hole,
        outer: h.outer,
    });
    vias.chain(holes).collect()
}

fn off_step(hole: f64, step: f64) -> bool {
    let steps = hole / step;
    (steps - steps.round()).abs() * step > 1e-6
}

fn check_drill(profile: &FabProfile, d: &Drill<'_>) -> Vec<Diagnostic> {
    let name = &profile.name;
    let mut messages = Vec::new();
    if d.hole < profile.min_drill - EPSILON {
        let (hole, min) = (mm(d.hole), mm(profile.min_drill));
        messages.push(format!(
            "{} drill {hole} mm is below {name} minimum {min} mm",
            d.kind
        ));
    }
    if off_step(d.hole, profile.drill_step) {
        let (hole, step) = (mm(d.hole), mm(profile.drill_step));
        messages.push(format!(
            "{} drill {hole} mm is not a multiple of the {name} step {step} mm",
            d.kind
        ));
    }
    if let Some(outer) = d
        .outer
        .filter(|o| d.kind == "via" && *o < profile.min_via_pad - EPSILON)
    {
        let (pad, min) = (mm(outer), mm(profile.min_via_pad));
        messages.push(format!("via pad {pad} mm is below {name} minimum {min} mm"));
    }
    messages
        .into_iter()
        .map(|m| violation(profile, m, vec![d.id.to_string()]))
        .collect()
}

/// Drill, via pad, via-to-via and board-size violations. Rings belong to
/// `annular_ring`.
pub(super) fn check(profile: &FabProfile, geometry: &Geometry) -> Vec<Diagnostic> {
    let per_drill = drills(geometry)
        .into_iter()
        .flat_map(|d| check_drill(profile, &d));
    per_drill
        .chain(layout::close_vias(profile, geometry))
        .chain(layout::board_size(profile, geometry))
        .collect()
}
