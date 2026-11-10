//! Every drilled hole of a board as a capsule: vias, plated and non-plated holes.

use crate::linter::rules::copper::Shape;
use crate::parser::model::{Model, PcbHole, PcbPlatedHole};

/// What kind of hole a drill is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    Via,
    Plated,
    Unplated,
}

impl Kind {
    /// The word used in messages.
    pub fn word(self) -> &'static str {
        match self {
            Self::Via => "via",
            Self::Plated => "plated hole",
            Self::Unplated => "non-plated hole",
        }
    }
}

/// One drill: its id, kind, centre and the capsule the bit cuts.
#[derive(Debug, Clone)]
pub(super) struct Drill {
    pub id: String,
    pub kind: Kind,
    pub at: (f64, f64),
    /// The smallest drill width, in millimetres.
    pub width: f64,
    pub shape: Shape,
}

fn capsule(at: (f64, f64), w: f64, h: f64, degrees: f64) -> Shape {
    let half = w.min(h) / 2.0;
    let reach = (w.max(h) / 2.0 - half).max(0.0);
    let turn = degrees.to_radians()
        + if h > w {
            std::f64::consts::FRAC_PI_2
        } else {
            0.0
        };
    let (dx, dy) = (reach * turn.cos(), reach * turn.sin());
    Shape::Seg {
        a: (at.0 - dx, at.1 - dy),
        b: (at.0 + dx, at.1 + dy),
        half,
    }
}

fn drill(id: &str, kind: Kind, at: (f64, f64), size: (f64, f64), degrees: f64) -> Drill {
    Drill {
        id: id.to_string(),
        kind,
        at,
        width: size.0.min(size.1),
        shape: capsule(at, size.0, size.1, degrees),
    }
}

fn size(d: Option<f64>, w: Option<f64>, h: Option<f64>) -> Option<(f64, f64)> {
    d.map(|d| (d, d)).or_else(|| w.zip(h))
}

fn plated(el: &PcbPlatedHole) -> Option<Drill> {
    let hole = size(el.hole_diameter, el.hole_width, el.hole_height)?;
    let at = (
        el.x + el.hole_offset_x.unwrap_or(0.0),
        el.y + el.hole_offset_y.unwrap_or(0.0),
    );
    let degrees = el.ccw_rotation.unwrap_or(0.0);
    Some(drill(
        &el.pcb_plated_hole_id,
        Kind::Plated,
        at,
        hole,
        degrees,
    ))
}

fn unplated(el: &PcbHole) -> Option<Drill> {
    let hole = size(el.hole_diameter, el.hole_width, el.hole_height)?;
    Some(drill(
        &el.pcb_hole_id,
        Kind::Unplated,
        (el.x, el.y),
        hole,
        0.0,
    ))
}

/// Every drill the model can size. Holes it cannot size are listed by the
/// geometry reader as unreadable, which `fab_profile` reports.
pub(super) fn drills(model: &Model) -> Vec<Drill> {
    let vias = model.vias.iter().map(|v| {
        let d = v.hole_diameter;
        drill(&v.pcb_via_id, Kind::Via, (v.x, v.y), (d, d), 0.0)
    });
    let plated = model.plated_holes.iter().filter_map(plated);
    let holes = model.holes.iter().filter_map(unplated);
    vias.chain(plated).chain(holes).collect()
}
