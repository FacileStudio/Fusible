//! `unrouted` per-net connectivity: three ports of one net at (0,0), (5,5)
//! and (10,0), each behind its own `source_trace`.

use serde_json::{json, Value};

use super::lint;
use crate::linter::rules::UnroutedRule;
use crate::linter::Diagnostic;

fn wire(x: f64, y: f64) -> Value {
    json!({"route_type": "wire", "x": x, "y": y, "width": 0.2, "layer": "top"})
}

fn trace(id: &str, source: &str, route: Value) -> Value {
    json!({"type": "pcb_trace", "pcb_trace_id": id, "source_trace_id": source, "route": route})
}

/// A route between two named ports, drawn away from every pad.
fn named(id: &str, source: &str, from: &str, to: &str) -> Value {
    let mut start = wire(20.0, 20.0);
    let mut end = wire(21.0, 20.0);
    start["start_pcb_port_id"] = json!(from);
    end["end_pcb_port_id"] = json!(to);
    trace(id, source, json!([start, end]))
}

fn check(extra: Vec<Value>) -> Vec<Diagnostic> {
    let mut elements = vec![
        json!({"type": "pcb_board", "width": 30, "height": 30, "center": {"x": 5, "y": 5}}),
        json!({"type": "source_net", "source_net_id": "n1", "name": "GND"}),
        json!({"type": "source_net", "source_net_id": "n2", "name": "VCC"}),
        json!({"type": "source_component", "source_component_id": "c1", "name": "U1"}),
    ];
    for (i, (x, y)) in [(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)]
        .into_iter()
        .enumerate()
    {
        let (port, pcb) = (format!("p{i}"), format!("pp{i}"));
        elements.extend([
            json!({"type": "source_port", "source_port_id": port, "name": format!("pin{i}"),
                "source_component_id": "c1"}),
            json!({"type": "source_trace", "source_trace_id": format!("s{i}"),
                "connected_source_port_ids": [port], "connected_source_net_ids": ["n1"]}),
            json!({"type": "pcb_port", "pcb_port_id": pcb, "source_port_id": port}),
            json!({"type": "pcb_smtpad", "pcb_smtpad_id": format!("pad{i}"), "pcb_port_id": pcb,
                "layer": "top", "shape": "rect", "x": x, "y": y, "width": 1, "height": 1}),
        ]);
    }
    elements.extend(extra);
    lint(UnroutedRule, &Value::Array(elements).to_string())
}

#[test]
fn a_net_routed_as_one_tree_passes_whatever_the_trace_ids() {
    let v = check(vec![
        named("t0", "s0__s1", "pp0", "pp1"),
        named("t1", "s0__s1", "pp1", "pp2"),
    ]);
    assert!(v.is_empty(), "{v:?}");
}

#[test]
fn a_port_neither_copper_nor_the_part_reaches_blows() {
    let v = check(vec![named("t0", "s0", "pp0", "pp1")]);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].rule, "unrouted");
    assert!(v[0].message.contains("net \"GND\""), "{}", v[0].message);
    assert!(v[0].message.contains("U1.pin2 (1 of 3 ports)"));
    assert_eq!(v[0].elements, ["p2"]);

    let tie = json!({"type": "source_component_internal_connection",
        "source_component_id": "c1", "source_port_ids": ["p1", "p2"]});
    let v = check(vec![named("t0", "s0", "pp0", "pp1"), tie]);
    assert!(
        v.is_empty(),
        "a pin tied inside the part needs no copper: {v:?}"
    );
}

#[test]
fn copper_that_touches_joins_without_port_ids() {
    let across = trace("t0", "s0", json!([wire(0.0, 0.0), wire(10.0, 0.0)]));
    let tee = trace("t1", "s1", json!([wire(5.0, 5.0), wire(5.0, 0.0)]));
    let v = check(vec![across.clone(), tee]);
    assert!(v.is_empty(), "{v:?}");

    let short = trace("t1", "s1", json!([wire(5.0, 5.0), wire(5.0, 2.0)]));
    let v = check(vec![across, short]);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].elements, ["p1"]);
}

#[test]
fn only_a_pour_of_the_same_net_joins_its_ports() {
    let pour = |net: &str| {
        json!({"type": "pcb_copper_pour", "pcb_copper_pour_id": "pour", "layer": "top",
            "shape": "rect", "source_net_id": net, "center": {"x": 5, "y": 5},
            "width": 30, "height": 30})
    };
    let routed = named("t0", "s0", "pp0", "pp1");
    assert!(check(vec![routed.clone(), pour("n1")]).is_empty());
    assert_eq!(check(vec![routed, pour("n2")]).len(), 1);
}
