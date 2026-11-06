//! Copper of drilled parts: plated holes and vias, present on every layer.

use serde_json::Value;

use super::copper::{Copper, Shape};
use crate::linter::LintContext;
use crate::parser::geometry::Geometry;

fn num(el: &Value, key: &str) -> Option<f64> {
    el.get(key)?.as_f64()
}

fn text<'a>(el: &'a Value, key: &str) -> Option<&'a str> {
    el.get(key)?.as_str()
}

fn layers(geometry: &Geometry) -> Vec<String> {
    let count = geometry.board.as_ref().map_or(2, |b| b.layers).max(2);
    let inner = (1..count - 1).map(|i| format!("inner{i}"));
    ["top".to_string(), "bottom".to_string()]
        .into_iter()
        .chain(inner)
        .collect()
}

fn plated_half(el: &Value) -> Option<(f64, f64)> {
    if let Some(d) = num(el, "outer_diameter") {
        return Some((d / 2.0, d / 2.0));
    }
    Some((
        num(el, "outer_width")? / 2.0,
        num(el, "outer_height")? / 2.0,
    ))
}

fn port_net(ctx: &LintContext<'_>, el: &Value) -> Option<String> {
    if let Some(id) = text(el, "net_id").or_else(|| text(el, "source_net_id")) {
        let idx = ctx.netlist.by_id.get(id)?;
        return Some(ctx.netlist.nets.get(*idx)?.display_name().to_string());
    }
    let pcb_port = text(el, "pcb_port_id")?;
    let source_port = ctx
        .circuit
        .raw_elements
        .iter()
        .filter(|e| text(e, "type") == Some("pcb_port"))
        .find(|e| text(e, "pcb_port_id") == Some(pcb_port))
        .and_then(|e| text(e, "source_port_id"))?;
    let idx = ctx.netlist.port_to_net.get(source_port)?;
    Some(ctx.netlist.nets.get(*idx)?.display_name().to_string())
}

fn via_net(ctx: &LintContext<'_>, geometry: &Geometry, el: &Value) -> Option<String> {
    if let Some(net) = port_net(ctx, el) {
        return Some(net);
    }
    let (x, y) = (num(el, "x")?, num(el, "y")?);
    let near = |p: (f64, f64)| (p.0 - x).abs() < 1e-6 && (p.1 - y).abs() < 1e-6;
    let touching = geometry.segments.iter().find(|s| near(s.a) || near(s.b))?;
    touching.net_names.first().cloned()
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
/// be resolved is left out, as for unnamed traces.
pub(super) fn drilled(ctx: &LintContext<'_>, geometry: &Geometry) -> Vec<Copper> {
    let mut out = Vec::new();
    for el in &ctx.circuit.raw_elements {
        let kind = text(el, "type");
        let (Some(x), Some(y)) = (num(el, "x"), num(el, "y")) else {
            continue;
        };
        let (half, net, id) = match kind {
            Some("pcb_plated_hole") => (
                plated_half(el),
                port_net(ctx, el),
                text(el, "pcb_plated_hole_id"),
            ),
            Some("pcb_via") => (
                num(el, "outer_diameter").map(|d| (d / 2.0, d / 2.0)),
                via_net(ctx, geometry, el),
                text(el, "pcb_via_id"),
            ),
            _ => continue,
        };
        let (Some((hw, hh)), Some(net)) = (half, net) else {
            continue;
        };
        let id = id.unwrap_or("drill").to_string();
        let shape = Shape::Rect { c: (x, y), hw, hh };
        out.extend(on_every_layer(geometry, id, net, shape));
    }
    out
}
