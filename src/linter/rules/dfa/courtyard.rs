//! Part courtyards as polygons, turned with the part.
//!
//! A `pcb_courtyard_rect` is turned by its own `ccw_rotation`, or by the
//! part's `rotation` when tscircuit left it out: older tscircuit wrote the
//! unturned size for a rotated part. Outlines are already in board space.

use std::collections::HashMap;

use crate::linter::rules::geom2d::Point;
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

    fn span(&self, axis: Point) -> (f64, f64) {
        let dots = self.points.iter().map(|p| p.0 * axis.0 + p.1 * axis.1);
        dots.fold((f64::MAX, f64::MIN), |(lo, hi), d| (lo.min(d), hi.max(d)))
    }

    fn separates(&self, other: &Courtyard, axis: Point) -> bool {
        let ((a0, a1), (b0, b1)) = (self.span(axis), other.span(axis));
        a1.min(b1) - a0.max(b0) <= EPSILON
    }

    /// Whether the two courtyards share area, by the separating axis test
    /// over both polygons' edge normals. Touching edges do not count. A
    /// concave outline is tested as its convex hull.
    pub fn overlaps(&self, other: &Courtyard) -> bool {
        let mut normals = self.edges().chain(other.edges()).filter_map(|(a, b)| {
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let len = dx.hypot(dy);
            (len > EPSILON).then(|| (-dy / len, dx / len))
        });
        !normals.any(|axis| self.separates(other, axis))
    }

    /// The polygon's edges, for distance checks.
    pub fn sides(&self) -> Vec<(Point, Point)> {
        self.edges().collect()
    }
}
