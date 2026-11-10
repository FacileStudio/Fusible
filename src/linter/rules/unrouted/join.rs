//! Joins of the copper graph: outlines that touch, and pins tied in a part.

use super::graph::{Graph, Piece};
use crate::linter::rules::dfm::Index;
use crate::linter::LintContext;

/// Outlines closer than this, in millimetres, are one conductor.
const TOUCH: f64 = 1e-6;

/// Whether two pieces are one conductor: a shared layer, touching outlines,
/// and no disagreement between their nets. A short is `shorted_nets`' finding,
/// so copper of two known nets never routes one through the other.
fn touch(a: &Piece, b: &Piece) -> bool {
    let layers = match (&a.layer, &b.layer) {
        (Some(x), Some(y)) => x == y,
        _ => true,
    };
    let nets = match (a.owner.1, b.owner.1) {
        (Some(x), Some(y)) => x == y,
        _ => true,
    };
    layers && nets && a.owner.0 != b.owner.0 && a.shape.gap(&b.shape) <= TOUCH
}

/// Unions the owners of every pair of touching pieces.
pub(super) fn touching(graph: &mut Graph<'_>) {
    let index = Index::new(graph.pieces.iter().map(|p| &p.shape));
    for (i, piece) in graph.pieces.iter().enumerate() {
        let near = index.near(&piece.shape, TOUCH);
        for other in near.into_iter().filter(|j| *j > i) {
            if touch(piece, &graph.pieces[other]) {
                graph.sets.union(piece.owner.0, graph.pieces[other].owner.0);
            }
        }
    }
}

/// Unions the ports of every `source_component_internal_connection`: the part
/// joins them, so no copper has to.
pub(super) fn internal(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for connection in &ctx.circuit.model().internal_connections {
        let ids = connection.source_port_ids.iter();
        let nodes: Vec<usize> = ids
            .filter_map(|id| graph.ports.get(id.as_str()).copied())
            .collect();
        for pair in nodes.windows(2) {
            graph.sets.union(pair[0], pair[1]);
        }
    }
}
