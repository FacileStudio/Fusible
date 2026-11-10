//! Pads and plated holes as copper of the port they realize.

use super::graph::{Graph, Owner, Piece};
use crate::linter::rules::copper::Shape;
use crate::linter::rules::pad_geometry::pad_box;
use crate::linter::LintContext;
use crate::parser::model::PcbPlatedHole;
use crate::parser::Pad;

fn pad_owner(graph: &Graph<'_>, ctx: &LintContext<'_>, pad: &Pad) -> Option<Owner> {
    let pcb = pad.pcb_port_id.as_deref();
    if let Some(owner) = pcb.and_then(|id| graph.pcb_ports.get(id)) {
        return Some(*owner);
    }
    let port = pad.port_id.as_deref()?;
    let node = *graph.ports.get(port)?;
    Some((node, ctx.netlist.port_to_net.get(port).copied()))
}

/// Half extents of the copper around a plated hole: its ring, its rectangular
/// pad, or the barrel alone when no pad size is written.
fn hole_half(hole: &PcbPlatedHole) -> Option<(f64, f64)> {
    let pair = |w: Option<f64>, h: Option<f64>| Some((w? / 2.0, h? / 2.0));
    pair(hole.outer_diameter, hole.outer_diameter)
        .or_else(|| pair(hole.outer_width, hole.outer_height))
        .or_else(|| pair(hole.rect_pad_width, hole.rect_pad_height))
        .or_else(|| pair(hole.hole_diameter, hole.hole_diameter))
        .or_else(|| pair(hole.hole_width, hole.hole_height))
}

fn add_holes(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for hole in &ctx.circuit.model().plated_holes {
        let port = hole.pcb_port_id.as_deref();
        let owner = port.and_then(|id| graph.pcb_ports.get(id).copied());
        let Some((owner, (hw, hh))) = owner.zip(hole_half(hole)) else {
            continue;
        };
        let c = (hole.x, hole.y);
        graph.pieces.push(Piece {
            owner,
            layer: None,
            shape: Shape::Rect { c, hw, hh },
        });
    }
}

/// Adds the copper of every pad and plated hole that belongs to a port.
pub(super) fn add(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for pad in ctx.circuit.pads() {
        let Some(owner) = pad_owner(graph, ctx, pad) else {
            continue;
        };
        let Some((x, y, hw, hh)) = pad_box(pad) else {
            continue;
        };
        let layer = pad.layer.as_deref().unwrap_or("top").to_ascii_lowercase();
        graph.pieces.push(Piece {
            owner,
            layer: Some(layer),
            shape: Shape::Rect { c: (x, y), hw, hh },
        });
    }
    add_holes(graph, ctx);
}
