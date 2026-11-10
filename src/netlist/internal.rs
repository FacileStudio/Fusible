//! Pins a part ties together inside its body. tscircuit lists them in
//! `source_component_internal_connection` (the TSX `internallyConnectedPins`);
//! they join the union-find exactly like a trace joins its ports.

use serde_json::Value;

use super::connectivity::{Connectivity, PortMap};
use crate::parser::Circuit;

/// One `source_component_internal_connection`: an optional id and the ports
/// that form one conductor inside the part.
pub(super) struct InternalConnection {
    pub id: Option<String>,
    pub port_ids: Vec<String>,
}

impl InternalConnection {
    fn label(&self, index: usize) -> String {
        match self.id.as_deref() {
            Some(id) => format!("source_component_internal_connection \"{id}\""),
            None => format!("source_component_internal_connection #{index}"),
        }
    }
}

/// Reads every internal connection from the raw elements, the one place
/// to swap once the parser exposes a typed accessor.
pub(super) fn internal_connections(circuit: &Circuit) -> Vec<InternalConnection> {
    circuit
        .raw_elements
        .iter()
        .filter(|e| {
            e.get("type").and_then(Value::as_str) == Some("source_component_internal_connection")
        })
        .map(|e| InternalConnection {
            id: e
                .get("source_component_internal_connection_id")
                .and_then(Value::as_str)
                .map(str::to_string),
            port_ids: e
                .get("source_port_ids")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
        })
        .collect()
}

impl Connectivity {
    /// Unions the ports of every internal connection; a port no element
    /// defines is a dangling reference, and tying two declared nets is a short.
    pub(super) fn join_internal(&mut self, circuit: &Circuit, ports: &PortMap<'_>) {
        for (index, connection) in internal_connections(circuit).iter().enumerate() {
            let label = connection.label(index);
            let nodes = self.known_ports(&label, &connection.port_ids, ports);
            let Some(&anchor) = nodes.first() else {
                continue;
            };
            for &node in &nodes {
                self.link(anchor, node, &label);
            }
            self.internal_ports.extend(
                connection
                    .port_ids
                    .iter()
                    .filter(|id| ports.contains_key(id.as_str()))
                    .cloned(),
            );
        }
    }
}
