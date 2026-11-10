//! Net names of each source trace.

use std::collections::{BTreeSet, HashMap};

use crate::parser::{Circuit, Trace};

pub(super) struct NetIndex {
    by_trace: HashMap<String, Vec<String>>,
}

impl NetIndex {
    pub(super) fn new(circuit: &Circuit) -> Self {
        let names: HashMap<&str, &str> = circuit
            .nets()
            .filter_map(|n| Some((n.net_id.as_deref()?, n.name.as_deref()?)))
            .collect();
        let port_net: HashMap<&str, &str> = circuit
            .ports()
            .filter_map(|p| Some((p.port_id.as_deref()?, p.net_id.as_deref()?)))
            .collect();
        let by_trace = circuit
            .traces()
            .filter_map(|t| Some((t.trace_id.clone()?, trace_nets(t, &names, &port_net))))
            .collect();
        Self { by_trace }
    }

    pub(super) fn names(&self, trace: &str) -> Vec<String> {
        self.by_trace.get(trace).cloned().unwrap_or_default()
    }
}

fn trace_nets(
    trace: &Trace,
    names: &HashMap<&str, &str>,
    port_net: &HashMap<&str, &str>,
) -> Vec<String> {
    let direct = trace.connected_source_net_ids.iter().map(String::as_str);
    let ports = trace.connected_source_port_ids.iter();
    let via_ports = ports.filter_map(|port| port_net.get(port.as_str()).copied());
    let ids: BTreeSet<&str> = direct.chain(via_ports).collect();
    let resolved: BTreeSet<&str> = ids.iter().filter_map(|id| names.get(id).copied()).collect();
    resolved.into_iter().map(str::to_string).collect()
}
