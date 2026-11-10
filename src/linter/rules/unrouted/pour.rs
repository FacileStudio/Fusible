//! Copper pours: a pour joins the copper of its own net that lies inside it.

use super::graph::{Graph, Piece};
use crate::linter::rules::copper::Shape;
use crate::linter::rules::dfm::Outline;
use crate::linter::rules::geom2d::Point;
use crate::linter::LintContext;
use crate::parser::model::{PcbCopperPour, Ring};

/// The filled area of a pour: inside `outer`, outside every cutout.
struct Region {
    outer: Outline,
    cutouts: Vec<Outline>,
}

fn ring(ring: &Ring) -> Outline {
    Outline::through(ring.vertices.iter().map(|p| (p.x, p.y)).collect())
}

fn rect(pour: &PcbCopperPour) -> Option<Outline> {
    let (c, hw, hh) = (pour.center?, pour.width? / 2.0, pour.height? / 2.0);
    let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)];
    let points = corners.iter().map(|(dx, dy)| (c.x + dx, c.y + dy));
    Some(Outline::through(points.collect()))
}

fn region(pour: &PcbCopperPour) -> Option<Region> {
    if let Some(brep) = &pour.brep_shape {
        return Some(Region {
            outer: ring(&brep.outer_ring),
            cutouts: brep.inner_rings.iter().map(ring).collect(),
        });
    }
    let drawn = pour.points.as_ref().filter(|p| p.len() >= 3);
    let outer = match drawn {
        Some(points) => Outline::through(points.iter().map(|p| (p.x, p.y)).collect()),
        None => rect(pour)?,
    };
    Some(Region {
        outer,
        cutouts: Vec::new(),
    })
}

impl Region {
    fn holds(&self, p: Point) -> bool {
        self.outer.contains(p) && !self.cutouts.iter().any(|cutout| cutout.contains(p))
    }

    fn reaches(&self, piece: &Piece) -> bool {
        match &piece.shape {
            Shape::Seg { a, b, .. } => self.holds(*a) || self.holds(*b),
            Shape::Rect { c, .. } => self.holds(*c),
        }
    }
}

/// Unions each pour with the pieces of its net anchored inside it on its
/// layer. A pour without a resolvable net joins nothing.
pub(super) fn join(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for pour in &ctx.circuit.model().copper_pours {
        let net = pour.source_net_id.as_deref();
        let Some(&net) = net.and_then(|id| ctx.netlist.by_id.get(id)) else {
            continue;
        };
        let Some(region) = region(pour) else {
            continue;
        };
        let layer = pour.layer.to_ascii_lowercase();
        let node = graph.sets.add();
        for piece in &graph.pieces {
            let on_layer = piece.layer.as_ref().is_none_or(|l| *l == layer);
            if on_layer && piece.owner.1 == Some(net) && region.reaches(piece) {
                graph.sets.union(node, piece.owner.0);
            }
        }
    }
}
