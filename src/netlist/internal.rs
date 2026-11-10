//! Pins a part ties together inside its body. tscircuit lists them in
//! `source_component_internal_connection` (the TSX `internallyConnectedPins`);
//! they join the union-find exactly like a trace joins its ports.

use super::connectivity::{Connectivity, PortMap};
use crate::parser::model::InternalConnection;
use crate::parser::Circuit;

fn label(connection: &InternalConnection, index: usize) -> String {
    match connection
        .source_component_internal_connection_id
        .as_deref()
    {
        Some(id) => format!("source_component_internal_connection \"{id}\""),
        None => format!("source_component_internal_connection #{index}"),
    }
}

impl Connectivity {
    /// Unions the ports of every internal connection; a port no element
    /// defines is a dangling reference, and tying two declared nets is a short.
    pub(super) fn join_internal(&mut self, circuit: &Circuit, ports: &PortMap<'_>) {
        for (index, connection) in circuit.model().internal_connections.iter().enumerate() {
            let label = label(connection, index);
            let nodes = self.known_ports(&label, &connection.source_port_ids, ports);
            let Some(&anchor) = nodes.first() else {
                continue;
            };
            for &node in &nodes {
                self.link(anchor, node, &label);
            }
            self.internal_ports.extend(
                connection
                    .source_port_ids
                    .iter()
                    .filter(|id| ports.contains_key(id.as_str()))
                    .cloned(),
            );
        }
    }
}
