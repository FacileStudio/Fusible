//! Copper of drilled parts: plated holes and vias, present on every layer.

use super::copper::{Copper, Shape};
use super::pad_geometry::hole_shape;
use crate::linter::LintContext;
use crate::parser::geometry::Geometry;
use crate::parser::model::PcbVia;

fn layers(geometry: &Geometry) -> Vec<String> {
    let count = geometry.board.as_ref().map_or(2, |b| b.layers).max(2);
    let inner = (1..count - 1).map(|i| format!("inner{i}"));
    ["top".to_string(), "bottom".to_string()]
        .into_iter()
        .chain(inner)
        .collect()
}

fn net_name(ctx: &LintContext<'_>, net: Option<usize>) -> Option<String> {
    Some(ctx.netlist.nets.get(net?)?.display_name().to_string())
}

fn port_net(ctx: &LintContext<'_>, net: Option<&str>, pcb_port: Option<&str>) -> Option<String> {
    if let Some(id) = net {
        return net_name(ctx, ctx.netlist.by_id.get(id).copied());
    }
    let pcb_port = pcb_port?;
    let source_port = ctx
        .circuit
        .pcb_ports()
        .find(|p| p.pcb_port_id.as_deref() == Some(pcb_port))
        .and_then(|p| p.source_port_id.as_deref())?;
    net_name(ctx, ctx.netlist.port_to_net.get(source_port).copied())
}

/// The net of a via: its own `source_net_id`, else the net of the trace it
/// belongs to or that ends on it, else a net of its own.
fn via_net(ctx: &LintContext<'_>, geometry: &Geometry, el: &PcbVia) -> String {
    let near = |p: (f64, f64)| (p.0 - el.x).abs() < 1e-6 && (p.1 - el.y).abs() < 1e-6;
    let trace = el.pcb_trace_id.as_deref();
    let mut segments = geometry.segments.iter();
    let owner = segments.find(|s| trace == Some(s.trace_id.as_str()) || near(s.a) || near(s.b));
    port_net(ctx, el.source_net_id.as_deref(), None)
        .or_else(|| owner?.net_names.first().cloned())
        .unwrap_or_else(|| format!("<unresolved via {}>", el.pcb_via_id))
}

fn on_every_layer(
    geometry: &Geometry,
    id: String,
    net: String,
    shape: Shape,
) -> impl Iterator<Item = Copper> {
    layers(geometry).into_iter().map(move |layer| Copper {
        id: id.clone(),
        net: net.clone(),
        layer,
        shape: shape.clone(),
    })
}

/// Plated holes and vias as copper on every layer. A part whose net cannot
/// be resolved is a net of its own, so it is checked against everything.
pub(super) fn drilled(ctx: &LintContext<'_>, geometry: &Geometry) -> Vec<Copper> {
    let model = ctx.circuit.model();
    let holes = model.plated_holes.iter().filter_map(|el| {
        let net = port_net(ctx, el.source_net_id.as_deref(), el.pcb_port_id.as_deref());
        let shape = hole_shape(el)?;
        let id = el.pcb_plated_hole_id.clone();
        let net = net.unwrap_or_else(|| format!("<unresolved hole {id}>"));
        Some((id, net, shape))
    });
    let vias = model.vias.iter().map(|el| {
        let shape = Shape::disc((el.x, el.y), el.outer_diameter / 2.0);
        (el.pcb_via_id.clone(), via_net(ctx, geometry, el), shape)
    });
    holes
        .chain(vias)
        .flat_map(|(id, net, shape)| on_every_layer(geometry, id, net, shape))
        .collect()
}
