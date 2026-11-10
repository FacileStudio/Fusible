//! The net of each routed `pcb_trace`, resolved through the netlist.
//!
//! tscircuit ports carry no net, so a trace is followed to the ports its route
//! names, then to the ports and nets of its `source_trace`. A router may merge
//! several traces into one id (`source_trace_1__source_trace_2`).

use std::collections::HashMap;

use crate::linter::LintContext;
use crate::netlist::Netlist;
use crate::parser::geometry::{read, Geometry};
use crate::parser::model::{PcbTrace, RoutePoint};
use crate::parser::Trace;

/// Lookups from PCB traces to netlist net indices.
pub(super) struct TraceNets<'a> {
    netlist: &'a Netlist,
    pcb_ports: HashMap<&'a str, &'a str>,
    sources: HashMap<&'a str, &'a Trace>,
}

/// The `pcb_port` ids a route starts or ends on.
pub(super) fn named_ports(trace: &PcbTrace) -> impl Iterator<Item = &str> {
    let ends = trace.route.iter().flat_map(|point| match point {
        RoutePoint::Wire {
            start_pcb_port_id,
            end_pcb_port_id,
            ..
        } => [start_pcb_port_id.as_deref(), end_pcb_port_id.as_deref()],
        RoutePoint::Via { .. } => [None, None],
    });
    ends.flatten()
}

/// The board geometry with a net on every segment.
///
/// A trace the netlist resolves takes that net's name. One it cannot keeps the
/// names its `source_trace` gave it, or becomes a net of its own: unknown
/// copper is checked against everything, never skipped.
pub(super) fn geometry(ctx: &LintContext<'_>) -> Geometry {
    let nets = TraceNets::new(ctx);
    let traces = ctx.circuit.model().traces.iter();
    let resolved: HashMap<&str, &str> = traces
        .filter_map(|trace| {
            let node = ctx.netlist.nets.get(nets.of(trace)?)?;
            Some((trace.pcb_trace_id.as_str(), node.display_name()))
        })
        .collect();
    let mut geometry = read(ctx.circuit);
    for segment in &mut geometry.segments {
        match resolved.get(segment.trace_id.as_str()) {
            Some(net) => segment.net_names = vec![net.to_string()],
            None if segment.net_names.is_empty() => {
                segment.net_names = vec![format!("<unresolved trace {}>", segment.trace_id)];
            }
            None => {}
        }
    }
    geometry
}

impl<'a> TraceNets<'a> {
    /// Indexes the PCB ports and source traces of `ctx`.
    pub fn new(ctx: &LintContext<'a>) -> Self {
        let pcb_ports = ctx
            .circuit
            .pcb_ports()
            .filter_map(|p| Some((p.pcb_port_id.as_deref()?, p.source_port_id.as_deref()?)));
        let sources = ctx
            .circuit
            .traces()
            .filter_map(|t| Some((t.trace_id.as_deref()?, t)));
        Self {
            netlist: ctx.netlist,
            pcb_ports: pcb_ports.collect(),
            sources: sources.collect(),
        }
    }

    fn of_source(&self, id: &str) -> Option<usize> {
        let trace = self.sources.get(id)?;
        let ports = trace.connected_source_port_ids.iter();
        let nets = trace.connected_source_net_ids.iter();
        ports
            .filter_map(|port| self.netlist.port_to_net.get(port))
            .chain(nets.filter_map(|net| self.netlist.by_id.get(net)))
            .next()
            .copied()
    }

    /// The net index of `trace`, or `None` when nothing ties it to a net.
    pub fn of(&self, trace: &PcbTrace) -> Option<usize> {
        let routed = named_ports(trace)
            .filter_map(|pcb| self.pcb_ports.get(pcb))
            .find_map(|port| self.netlist.port_to_net.get(*port).copied());
        routed.or_else(|| {
            let ids = trace.source_trace_id.as_deref()?;
            ids.split("__").find_map(|id| self.of_source(id))
        })
    }
}
