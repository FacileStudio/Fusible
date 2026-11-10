//! Vias and non-plated holes as test terminals.

use super::terminals::net_of;
use super::Terminal;
use crate::netlist::Netlist;
use crate::parser::model::{PcbHole, PcbVia};
use crate::parser::Circuit;

/// The net of a via: its own `source_net_id`, else the net of the first
/// port or net its trace joins.
fn via_net(v: &PcbVia, circuit: &Circuit, netlist: &Netlist) -> String {
    if let Some(id) = v.source_net_id.as_deref() {
        return net_of(netlist, netlist.by_id.get(id));
    }
    let model = circuit.model();
    let pcb = model
        .traces
        .iter()
        .find(|t| Some(t.pcb_trace_id.as_str()) == v.pcb_trace_id.as_deref());
    let source = pcb.and_then(|t| t.source_trace_id.as_deref());
    let trace = circuit
        .traces()
        .find(|t| t.trace_id.is_some() && t.trace_id.as_deref() == source);
    let Some(trace) = trace else {
        return net_of(netlist, None);
    };
    let ports = trace
        .connected_source_port_ids
        .iter()
        .filter_map(|p| netlist.port_to_net.get(p));
    let nets = trace
        .connected_source_net_ids
        .iter()
        .filter_map(|n| netlist.by_id.get(n));
    net_of(netlist, ports.chain(nets).next())
}

fn hole(h: &PcbHole) -> Option<Terminal> {
    let d = h
        .hole_diameter
        .or_else(|| Some(h.hole_width?.min(h.hole_height?)))?;
    Some(Terminal {
        net: "N/C".into(),
        refdes: "HOLE".into(),
        pin: String::new(),
        drill: Some((d, false)),
        access: 0,
        at: (h.x, h.y),
        size: (d, d),
    })
}

/// Every via, then every non-plated hole.
pub(super) fn drilled(circuit: &Circuit, netlist: &Netlist) -> Vec<Terminal> {
    let model = circuit.model();
    let vias = model.vias.iter().map(|v| Terminal {
        net: via_net(v, circuit, netlist),
        refdes: "VIA".into(),
        pin: String::new(),
        drill: Some((v.hole_diameter, true)),
        access: 0,
        at: (v.x, v.y),
        size: (v.outer_diameter, v.outer_diameter),
    });
    vias.chain(model.holes.iter().filter_map(hole)).collect()
}
