//! `unrouted` and standalone `<via>` elements: a via joins its own ports even
//! when the routes that end on it carry no port ids, as in a replayed build.

use serde_json::{json, Value};

use super::unrouted_nets::{check, trace, wire};

fn via_port(id: &str, layer: &str) -> [Value; 3] {
    [
        json!({"type": "source_port", "source_port_id": id, "name": layer,
            "source_component_id": "via1"}),
        json!({"type": "source_trace", "source_trace_id": format!("s_{id}"),
            "connected_source_port_ids": [id], "connected_source_net_ids": ["n1"]}),
        json!({"type": "pcb_port", "pcb_port_id": format!("pp_{id}"), "source_port_id": id,
            "x": 20, "y": 0, "layers": [layer]}),
    ]
}

fn board(via_ports: Value) -> Vec<Value> {
    let mut parts: Vec<Value> = via_port("vt", "top").into();
    parts.extend(via_port("vb", "bottom"));
    parts.extend([
        trace("t0", "s0", json!([wire(0.0, 0.0), wire(10.0, 0.0)])),
        trace("t1", "s1", json!([wire(5.0, 5.0), wire(5.0, 0.0)])),
        trace("t2", "s0", json!([wire(10.0, 0.0), wire(20.0, 0.0)])),
        json!({"type": "pcb_via", "pcb_via_id": "v1", "x": 20, "y": 0, "outer_diameter": 0.6,
            "hole_diameter": 0.3, "pcb_port_ids": via_ports}),
    ]);
    parts
}

#[test]
fn a_standalone_via_joins_its_own_ports_without_route_port_ids() {
    let v = check(board(json!(["pp_vt", "pp_vb"])));
    assert!(v.is_empty(), "{v:?}");

    let v = check(board(json!([])));
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(v[0].message.contains("2 of 5 ports"), "{}", v[0].message);
}
