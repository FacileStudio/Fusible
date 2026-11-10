//! Routed copper always has a net: port-to-port traces resolve through the
//! netlist, and copper nothing ties to a net is checked as a net of its own.

use serde_json::{json, Value};

use super::physical::lint_with;
use crate::linter::rules::{DrillToCopperRule, FabProfileRule};
use crate::linter::Diagnostic;

const FAB: &str = "fab: jlcpcb-2l\n";

fn route(id: &str, source: Option<&str>, y: f64) -> Value {
    let wire = |x: f64| json!({"route_type": "wire", "x": x, "y": y, "width": 0.2, "layer": "top"});
    json!({"type": "pcb_trace", "pcb_trace_id": id, "source_trace_id": source,
        "route": [wire(0.0), wire(10.0)]})
}

/// Two port-to-port source traces, as tscircuit writes them: no net ids.
fn links() -> Vec<Value> {
    let mut elements = Vec::new();
    for (trace, ports) in [("s0", ["a", "b"]), ("s1", ["c", "d"])] {
        elements.extend(ports.map(|p| json!({"type": "source_port", "source_port_id": p})));
        elements.push(json!({"type": "source_trace", "source_trace_id": trace,
            "display_name": format!("{} to {}", ports[0], ports[1]),
            "connected_source_port_ids": ports, "connected_source_net_ids": []}));
    }
    elements
}

fn spacing(extra: Vec<Value>) -> Vec<Diagnostic> {
    let elements: Vec<Value> = links().into_iter().chain(extra).collect();
    lint_with(FabProfileRule, &Value::Array(elements).to_string(), FAB)
}

#[test]
fn port_to_port_traces_are_checked_for_spacing() {
    let v = spacing(vec![
        route("t0", Some("s0"), 0.0),
        route("t1", Some("s1"), 0.25),
    ]);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(
        v[0].message.contains("between a to b and c to d"),
        "{}",
        v[0].message
    );

    let same = spacing(vec![
        route("t0", Some("s0"), 0.0),
        route("t1", Some("s0"), 0.25),
    ]);
    assert!(
        same.is_empty(),
        "one net keeps no spacing to itself: {same:?}"
    );
}

#[test]
fn a_trace_no_net_claims_is_a_net_of_its_own() {
    let v = spacing(vec![
        route("t0", Some("s0"), 0.0),
        route("stray", None, 0.25),
    ]);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(
        v[0].message.contains("<unresolved trace stray>"),
        "{}",
        v[0].message
    );
}

#[test]
fn a_via_no_net_claims_is_checked_against_every_trace() {
    let via = json!({"type": "pcb_via", "pcb_via_id": "v0", "x": 5, "y": 0.4,
        "outer_diameter": 0.6, "hole_diameter": 0.3, "layers": ["top", "bottom"]});
    let elements: Vec<Value> = links()
        .into_iter()
        .chain([route("t0", Some("s0"), 0.0), via])
        .collect();
    let v = lint_with(DrillToCopperRule, &Value::Array(elements).to_string(), FAB);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].elements, ["v0", "t0"]);
}
