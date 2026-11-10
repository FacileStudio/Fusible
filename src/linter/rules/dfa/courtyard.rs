//! Part courtyards as polygons, turned with the part.
//!
//! A `pcb_courtyard_rect` is turned by its own `ccw_rotation`, or by the
//! part's `rotation` when tscircuit left it out: older tscircuit wrote the
//! unturned size for a rotated part. Outlines are already in board space.

use std::collections::HashMap;

use crate::linter::rules::geom2d::{crosses, point_seg, Point};
use crate::parser::model::{CourtyardRect, Model};

const EPSILON: f64 = 1e-6;

/// One courtyard polygon of a part on one side.
#[derive(Debug, Clone)]
pub(super) struct Courtyard {
    pub part: String,
    pub layer: String,
    pub points: Vec<Point>,
}

fn turned_rect(rect: &CourtyardRect, degrees: f64) -> Vec<Point> {
    let (sin, cos) = degrees.to_radians().sin_cos();
    let (hw, hh) = (rect.width / 2.0, rect.height / 2.0);
    let c = (rect.center.x, rect.center.y);
    let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)];
    let turn = |(x, y): Point| (c.0 + x * cos - y * sin, c.1 + x * sin + y * cos);
    corners.into_iter().map(turn).collect()
}

/// Every courtyard of `model`, keyed to its `pcb_component_id`.
pub(super) fn courtyards(model: &Model) -> Vec<Courtyard> {
    let rotation: HashMap<&str, f64> = model
        .components
        .iter()
        .map(|c| (c.pcb_component_id.as_str(), c.rotation))
        .collect();
    let rects = model.courtyard_rects.iter().map(|r| {
        let part = rotation
            .get(r.pcb_component_id.as_str())
            .copied()
            .unwrap_or(0.0);
        Courtyard {
            part: r.pcb_component_id.clone(),
            layer: r.layer.clone(),
            points: turned_rect(r, r.ccw_rotation.unwrap_or(part)),
        }
    });
    let outlines = model
        .courtyard_outlines
        .iter()
        .filter(|o| o.outline.len() >= 3)
        .map(|o| Courtyard {
            part: o.pcb_component_id.clone(),
            layer: o.layer.clone(),
            points: o.outline.iter().map(|p| (p.x, p.y)).collect(),
        });
    rects.chain(outlines).collect()
}

impl Courtyard {
    fn edges(&self) -> impl Iterator<Item = (Point, Point)> + '_ {
        let n = self.points.len();
        (0..n).map(move |i| (self.points[i], self.points[(i + 1) % n]))
    }

    /// The mean of the vertices.
    pub fn centre(&self) -> Point {
        let n = self.points.len() as f64;
        let sum = self
            .points
            .iter()
            .fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
        (sum.0 / n, sum.1 / n)
    }

    /// Whether `p` lies inside the polygon and off its boundary.
    fn holds(&self, p: Point) -> bool {
        let on_edge = |(a, b): (Point, Point)| point_seg(p, a, b) <= EPSILON;
        let hit = |(a, b): &(Point, Point)| {
            (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0)
        };
        !self.edges().any(on_edge) && self.edges().filter(hit).count() % 2 == 1
    }

    /// Whether the two courtyards share area: two edges cross, or a vertex
    /// or the centre of one lies inside the other. Touching edges do not
    /// count, and a concave outline keeps its notches.
    pub fn overlaps(&self, other: &Courtyard) -> bool {
        let crossing = self
            .edges()
            .any(|(a, b)| other.edges().any(|(c, d)| crosses(a, b, c, d)));
        let within = |inner: &Courtyard, outer: &Courtyard| {
            let centre = Some(inner.centre()).filter(|c| inner.holds(*c));
            let mut probes = inner.points.iter().copied().chain(centre);
            probes.any(|p| outer.holds(p))
        };
        crossing || within(self, other) || within(other, self)
    }

    /// The polygon's edges, for distance checks.
    pub fn sides(&self) -> Vec<(Point, Point)> {
        self.edges().collect()
    }
}

#[cfg(test)]
#[path = "courtyard_tests.rs"]
mod tests;
