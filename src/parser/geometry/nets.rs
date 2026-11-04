//! Net names of each source trace.

use std::collections::{BTreeSet, HashMap};

use serde_json::Value;

use super::json::{strings, text};

pub(super) struct NetIndex {
    by_trace: HashMap<String, Vec<String>>,
}

impl NetIndex {
    pub(super) fn new(elements: &[Value]) -> Self {
        let kind = |el: &'_ Value| text(el, &["type"]).unwrap_or("").to_string();
        let mut names = HashMap::new();
        let mut port_net = HashMap::new();
        for el in elements {
            match kind(el).as_str() {
                "source_net" => {
                    insert_pair(&mut names, el, &["source_net_id", "net_id"], &["name"])
                }
                "source_port" => insert_pair(
                    &mut port_net,
                    el,
                    &["source_port_id", "port_id"],
                    &["source_net_id", "net_id"],
                ),
                _ => {}
            }
        }
        let by_trace = elements
            .iter()
            .filter(|el| kind(el) == "source_trace")
            .filter_map(|el| Some((text(el, &["source_trace_id", "trace_id"])?, el)))
            .map(|(id, el)| (id.to_string(), trace_nets(el, &names, &port_net)))
            .collect();
        Self { by_trace }
    }

    pub(super) fn names(&self, trace: &str) -> Vec<String> {
        self.by_trace.get(trace).cloned().unwrap_or_default()
    }
}

fn insert_pair(map: &mut HashMap<String, String>, el: &Value, keys: &[&str], values: &[&str]) {
    if let (Some(key), Some(value)) = (text(el, keys), text(el, values)) {
        map.insert(key.to_string(), value.to_string());
    }
}

fn trace_nets(
    el: &Value,
    names: &HashMap<String, String>,
    port_net: &HashMap<String, String>,
) -> Vec<String> {
    let direct = strings(el, &["connected_source_net_ids", "connected_net_ids"]);
    let ports = strings(el, &["connected_source_port_ids", "connected_port_ids"]);
    let via_ports = ports.iter().filter_map(|port| port_net.get(port).cloned());
    let ids: BTreeSet<String> = direct.into_iter().chain(via_ports).collect();
    let resolved: BTreeSet<&String> = ids.iter().filter_map(|id| names.get(id)).collect();
    resolved.into_iter().cloned().collect()
}
