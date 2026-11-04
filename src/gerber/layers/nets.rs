//! Net connectivity keys resolved from Circuit JSON source elements.

use std::collections::HashMap;

use serde_json::Value;

use super::elements::PcbTraceElm;

/// Maps PCB and source element ids to their connectivity key; equal keys mean the same net.
#[derive(Debug, Default)]
pub struct NetKeys {
    pcb_to_source_port: HashMap<String, String>,
    source_port_key: HashMap<String, String>,
    source_net_key: HashMap<String, String>,
    source_trace_key: HashMap<String, String>,
}

impl NetKeys {
    /// Records the connectivity data carried by a `pcb_port`, `source_port`, `source_net` or `source_trace`.
    pub(super) fn record(&mut self, kind: &str, val: &Value) {
        let text = |key: &str| val.get(key).and_then(Value::as_str).map(str::to_owned);
        let conn = text("subcircuit_connectivity_map_key");
        let (map, id, value) = match kind {
            "pcb_port" => (
                &mut self.pcb_to_source_port,
                text("pcb_port_id"),
                text("source_port_id"),
            ),
            "source_port" => (&mut self.source_port_key, text("source_port_id"), conn),
            "source_net" => (&mut self.source_net_key, text("source_net_id"), conn),
            _ => (&mut self.source_trace_key, text("source_trace_id"), conn),
        };
        if let (Some(id), Some(value)) = (id, value) {
            map.insert(id, value);
        }
    }

    /// Returns the net key of a `pcb_port` id.
    pub(super) fn port_net(&self, pcb_port_id: Option<&str>) -> Option<String> {
        let source = self.pcb_to_source_port.get(pcb_port_id?)?;
        self.source_port_key.get(source).cloned()
    }

    /// Returns the net key of a `source_net` id.
    pub(super) fn source_net(&self, source_net_id: Option<&str>) -> Option<String> {
        self.source_net_key.get(source_net_id?).cloned()
    }

    /// Returns the net key of a routed trace, from its source trace or the first port it connects.
    pub(super) fn trace_net(&self, trace: &PcbTraceElm) -> Option<String> {
        let from_source = trace
            .source_trace_id
            .as_ref()
            .and_then(|id| self.source_trace_key.get(id));
        from_source.cloned().or_else(|| {
            trace
                .connects_to
                .iter()
                .find_map(|p| self.port_net(Some(p)))
        })
    }
}

/// Returns true when both nets are known and equal.
pub(super) fn same_net(a: &Option<String>, b: &Option<String>) -> bool {
    a.is_some() && a == b
}
