//! Pads and plated holes as copper of the port they realize.

use super::graph::{Graph, Owner, Piece};
use crate::linter::rules::pad_geometry::{hole_shape, pad_shape};
use crate::linter::LintContext;
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

fn add_holes(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for hole in &ctx.circuit.model().plated_holes {
        let port = hole.pcb_port_id.as_deref();
        let owner = port.and_then(|id| graph.pcb_ports.get(id).copied());
        let Some((owner, shape)) = owner.zip(hole_shape(hole)) else {
            continue;
        };
        graph.pieces.push(Piece {
            owner,
            layer: None,
            shape,
        });
    }
}

/// Adds the copper of every pad and plated hole that belongs to a port.
pub(super) fn add(graph: &mut Graph<'_>, ctx: &LintContext<'_>) {
    for pad in ctx.circuit.pads() {
        let Some((owner, shape)) = pad_owner(graph, ctx, pad).zip(pad_shape(pad)) else {
            continue;
        };
        let layer = pad.layer.as_deref().unwrap_or("top").to_ascii_lowercase();
        graph.pieces.push(Piece {
            owner,
            layer: Some(layer),
            shape,
        });
    }
    add_holes(graph, ctx);
}
