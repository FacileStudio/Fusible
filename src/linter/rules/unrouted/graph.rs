//! Routed copper as a graph: ports, traces and pours are nodes, and two nodes
//! join when their copper touches on one layer.

use std::collections::HashMap;

use crate::linter::rules::copper::Shape;
use crate::linter::rules::trace_net::{named_ports, TraceNets};
use crate::linter::LintContext;
use crate::netlist::union_find::UnionFind;
use crate::parser::geometry::read;

/// A node and the net the netlist gives it, when it has one.
pub(super) type Owner = (usize, Option<usize>);

/// One copper outline owned by a node. `layer` is `None` for drilled copper,
/// which sits on every layer.
pub(super) struct Piece {
    pub owner: Owner,
    pub layer: Option<String>,
    pub shape: Shape,
}

/// The copper connectivity of one board.
#[derive(Default)]
pub(super) struct Graph<'a> {
    pub sets: UnionFind,
    /// Node of each `source_port` that has a `pcb_port`.
    pub ports: HashMap<&'a str, usize>,
    pub pcb_ports: HashMap<&'a str, Owner>,
    pub pieces: Vec<Piece>,
    traces: HashMap<&'a str, Owner>,
}

impl<'a> Graph<'a> {
    /// Builds the graph of `ctx`: named route ends, touching copper, pours of
    /// the same net and pins tied inside a part.
    pub fn new(ctx: &LintContext<'a>) -> Self {
        let mut graph = Self::default();
        graph.add_ports(ctx);
        graph.add_traces(ctx);
        graph.add_vias(ctx);
        super::pads::add(&mut graph, ctx);
        super::join::touching(&mut graph);
        super::pour::join(&mut graph, ctx);
        super::join::internal(&mut graph, ctx);
        graph
    }

    fn add_ports(&mut self, ctx: &LintContext<'a>) {
        for port in ctx.circuit.pcb_ports() {
            let pcb = port.pcb_port_id.as_deref();
            let Some((pcb, source)) = pcb.zip(port.source_port_id.as_deref()) else {
                continue;
            };
            let sets = &mut self.sets;
            let node = *self.ports.entry(source).or_insert_with(|| sets.add());
            let net = ctx.netlist.port_to_net.get(source).copied();
            self.pcb_ports.insert(pcb, (node, net));
        }
    }

    fn add_traces(&mut self, ctx: &LintContext<'a>) {
        let nets = TraceNets::new(ctx);
        for trace in &ctx.circuit.model().traces {
            let owner = (self.sets.add(), nets.of(trace));
            self.traces.insert(trace.pcb_trace_id.as_str(), owner);
            for end in named_ports(trace) {
                if let Some(&(port, _)) = self.pcb_ports.get(end) {
                    self.sets.union(owner.0, port);
                }
            }
        }
        for segment in read(ctx.circuit).segments {
            let Some(&owner) = self.traces.get(segment.trace_id.as_str()) else {
                continue;
            };
            let (a, b, half) = (segment.a, segment.b, segment.width / 2.0);
            self.pieces.push(Piece {
                owner,
                layer: Some(segment.layer),
                shape: Shape::Seg { a, b, half },
            });
        }
    }

    fn add_vias(&mut self, ctx: &LintContext<'a>) {
        for via in &ctx.circuit.model().vias {
            let routed = via.pcb_trace_id.as_deref();
            let owner = match routed.and_then(|id| self.traces.get(id)) {
                Some(owner) => *owner,
                None => {
                    let net = via.source_net_id.as_deref();
                    let net = net.and_then(|id| ctx.netlist.by_id.get(id).copied());
                    (self.sets.add(), net)
                }
            };
            for port in &via.pcb_port_ids {
                if let Some(&(node, _)) = self.pcb_ports.get(port.as_str()) {
                    self.sets.union(owner.0, node);
                }
            }
            self.pieces.push(Piece {
                owner,
                layer: None,
                shape: Shape::disc((via.x, via.y), via.outer_diameter / 2.0),
            });
        }
    }
}
