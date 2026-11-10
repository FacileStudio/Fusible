//! Copper shapes shared by the physical rules: capsules for trace segments,
//! rectangles for pads, edge-to-edge gaps and an R-tree to find neighbours.

use rstar::primitives::{GeomWithData, Rectangle};
use rstar::{RTree, AABB};

use super::pad_geometry::{pad_net, pad_shape};
use crate::linter::LintContext;
use crate::parser::geometry::Geometry;

use super::geom2d::{seg_rect, seg_seg, Point};

/// A copper outline: a trace segment with half its width, or a pad box.
/// Round copper is a segment of no length, see [`Shape::disc`].
#[derive(Debug, Clone)]
pub(super) enum Shape {
    Seg { a: Point, b: Point, half: f64 },
    Rect { c: Point, hw: f64, hh: f64 },
}

/// One piece of copper with the net it belongs to.
#[derive(Debug, Clone)]
pub(super) struct Copper {
    pub id: String,
    pub net: String,
    pub layer: String,
    pub shape: Shape,
}

impl Shape {
    /// A circle of radius `r` around `c`: a via, a round pad, a round ring.
    pub fn disc(c: Point, r: f64) -> Self {
        Self::Seg {
            a: c,
            b: c,
            half: r,
        }
    }

    /// Whether this is round copper and not a stretch of trace.
    pub fn is_disc(&self) -> bool {
        matches!(self, Self::Seg { a, b, .. } if a == b)
    }

    fn reach(&self) -> f64 {
        match self {
            Self::Seg { half, .. } => *half,
            Self::Rect { .. } => 0.0,
        }
    }

    fn centre_distance(&self, other: &Shape) -> f64 {
        match (self, other) {
            (Self::Seg { a, b, .. }, Self::Seg { a: c, b: d, .. }) => seg_seg(*a, *b, *c, *d),
            (Self::Seg { a, b, .. }, Self::Rect { c, hw, hh }) => seg_rect(*a, *b, *c, *hw, *hh),
            (Self::Rect { c, hw, hh }, Self::Seg { a, b, .. }) => seg_rect(*a, *b, *c, *hw, *hh),
            (Self::Rect { c, hw, hh }, Self::Rect { c: k, hw: w, hh: h }) => {
                let gap = |d: f64, r: f64| (d.abs() - r).max(0.0);
                gap(c.0 - k.0, hw + w).hypot(gap(c.1 - k.1, hh + h))
            }
        }
    }

    /// Edge-to-edge distance to `other` in millimetres, 0 when they touch.
    pub fn gap(&self, other: &Shape) -> f64 {
        (self.centre_distance(other) - self.reach() - other.reach()).max(0.0)
    }

    pub(super) fn envelope(&self, margin: f64) -> AABB<[f64; 2]> {
        let reach = self.reach() + margin;
        let (lo, hi) = match self {
            Self::Seg { a, b, .. } => ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1))),
            Self::Rect { c, hw, hh } => ((c.0 - hw, c.1 - hh), (c.0 + hw, c.1 + hh)),
        };
        AABB::from_corners([lo.0 - reach, lo.1 - reach], [hi.0 + reach, hi.1 + reach])
    }
}

/// Every trace segment, positioned pad, plated hole and via as copper.
///
/// `geometry` comes from `trace_net::geometry`, which puts a net on every
/// segment. A segment read without one is a net of its own, never left out.
pub(super) fn collect(ctx: &LintContext<'_>, geometry: &Geometry) -> Vec<Copper> {
    let segments = geometry.segments.iter().map(|s| {
        let named = s.net_names.first().cloned();
        Copper {
            id: s.trace_id.clone(),
            net: named.unwrap_or_else(|| format!("<unresolved trace {}>", s.trace_id)),
            layer: s.layer.clone(),
            shape: Shape::Seg {
                a: s.a,
                b: s.b,
                half: s.width / 2.0,
            },
        }
    });
    let pads = ctx.circuit.pads().enumerate().filter_map(|(index, pad)| {
        let shape = pad_shape(pad)?;
        let id = pad.pad_id.clone().unwrap_or_else(|| format!("#{index}"));
        Some(Copper {
            net: pad_net(ctx, pad, &id).1,
            layer: pad.layer.as_deref().unwrap_or("top").to_ascii_lowercase(),
            shape,
            id,
        })
    });
    segments
        .chain(pads)
        .chain(super::copper_drill::drilled(ctx, geometry))
        .collect()
}

/// Index pairs `(i, j)`, `i < j`, of copper on one layer, of distinct nets,
/// whose outlines come within `margin` of each other.
pub(super) fn neighbours(items: &[Copper], margin: f64) -> Vec<(usize, usize)> {
    let boxes = items.iter().enumerate().map(|(i, item)| {
        let envelope = item.shape.envelope(0.0);
        GeomWithData::new(Rectangle::from_aabb(envelope), i)
    });
    let tree = RTree::bulk_load(boxes.collect());
    let mut pairs = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let near = tree.locate_in_envelope_intersecting(item.shape.envelope(margin));
        for other in near.map(|found| found.data).filter(|j| *j > i) {
            if items[other].net != item.net && items[other].layer == item.layer {
                pairs.push((i, other));
            }
        }
    }
    pairs.sort_unstable();
    pairs
}
