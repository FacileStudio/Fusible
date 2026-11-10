//! The verdict per net: which of its ports routed copper does not reach.

use std::collections::{BTreeMap, HashMap};

use super::graph::Graph;
use crate::linter::LintContext;

/// A net whose ports sit on more than one island of copper.
pub(super) struct Split<'a> {
    pub net: &'a str,
    /// Source port ids outside the largest island, in file order.
    pub stranded: Vec<&'a str>,
    pub total: usize,
}

/// The island holding the most ports; the first one seen wins a tie.
fn largest(ports: &[(&str, usize)]) -> Option<usize> {
    let mut sizes: Vec<(usize, usize)> = Vec::new();
    for (_, island) in ports {
        match sizes.iter_mut().find(|(seen, _)| seen == island) {
            Some((_, size)) => *size += 1,
            None => sizes.push((*island, 1)),
        }
    }
    let best = sizes.iter().map(|(_, size)| *size).max()?;
    sizes.iter().find(|(_, size)| *size == best).map(|s| s.0)
}

/// Every net with a port on the board that the net's copper does not reach.
pub(super) fn splits<'a>(ctx: &LintContext<'a>, graph: &mut Graph<'a>) -> Vec<Split<'a>> {
    let mut per_net: BTreeMap<usize, Vec<(&'a str, usize)>> = BTreeMap::new();
    for id in ctx.circuit.ports().filter_map(|p| p.port_id.as_deref()) {
        let net = ctx.netlist.port_to_net.get(id);
        if let Some((&net, &node)) = net.zip(graph.ports.get(id)) {
            let island = graph.sets.find(node);
            per_net.entry(net).or_default().push((id, island));
        }
    }
    let found = per_net.into_iter().filter_map(|(net, ports)| {
        let main = largest(&ports)?;
        let stranded: Vec<&str> = ports.iter().filter(|p| p.1 != main).map(|p| p.0).collect();
        (!stranded.is_empty()).then(|| Split {
            net: ctx.netlist.nets[net].display_name(),
            stranded,
            total: ports.len(),
        })
    });
    found.collect()
}

/// `U1.GND` style names of the source ports, by id.
pub(super) fn labels<'a>(ctx: &LintContext<'a>) -> HashMap<&'a str, String> {
    let parts: HashMap<&str, &str> = ctx
        .circuit
        .components()
        .filter_map(|c| Some((c.component_id.as_deref()?, c.name.as_deref()?)))
        .collect();
    let named = ctx.circuit.ports().filter_map(|port| {
        let id = port.port_id.as_deref()?;
        let part = port.source_component_id.as_deref();
        let part = part.and_then(|p| parts.get(p)).copied();
        let label = match (part, port.name.as_deref()) {
            (Some(part), Some(pin)) => format!("{part}.{pin}"),
            _ => id.to_string(),
        };
        Some((id, label))
    });
    named.collect()
}
