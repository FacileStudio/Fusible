//! Tests for pins joined inside a part by `source_component_internal_connection`.

use super::tests::{net, netlist};

#[test]
fn pins_joined_internally_form_one_net() {
    let n = netlist(
        r#"[
        {"type": "source_net", "source_net_id": "n_gnd", "name": "GND"},
        {"type": "source_port", "source_port_id": "a", "net_id": "n_gnd"},
        {"type": "source_port", "source_port_id": "b"},
        {"type": "source_component_internal_connection",
         "source_component_internal_connection_id": "ic0", "source_port_ids": ["a", "b"]}
    ]"#,
    );
    assert_eq!(n.nets.len(), 1);
    assert_eq!(n.port_to_net["b"], n.by_id["n_gnd"]);
    assert_eq!(net(&n, "n_gnd").endpoints.len(), 2);
    assert!(n.shorts().is_empty());
}

#[test]
fn floating_pin_joined_internally_to_a_connected_pin_is_not_floating() {
    let n = netlist(
        r#"[
        {"type": "source_net", "source_net_id": "n_gnd", "name": "GND"},
        {"type": "source_port", "source_port_id": "a"},
        {"type": "source_port", "source_port_id": "b"},
        {"type": "source_port", "source_port_id": "c", "net_id": "n_gnd"},
        {"type": "source_trace", "source_trace_id": "t0", "connected_source_port_ids": ["a", "b"]}
    ]"#,
    );
    assert_eq!(n.floating_nets().count(), 1);
    let joined = netlist(
        r#"[
        {"type": "source_net", "source_net_id": "n_gnd", "name": "GND"},
        {"type": "source_port", "source_port_id": "a"},
        {"type": "source_port", "source_port_id": "b"},
        {"type": "source_port", "source_port_id": "c", "net_id": "n_gnd"},
        {"type": "source_trace", "source_trace_id": "t0", "connected_source_port_ids": ["a", "b"]},
        {"type": "source_component_internal_connection",
         "source_component_internal_connection_id": "ic0", "source_port_ids": ["c", "a"]}
    ]"#,
    );
    assert_eq!(joined.floating_nets().count(), 0);
    assert_eq!(joined.nets.len(), 1);
    assert_eq!(net(&joined, "n_gnd").endpoints.len(), 3);
}

#[test]
fn internal_connection_between_declared_nets_is_a_short() {
    let n = netlist(
        r#"[
        {"type": "source_net", "source_net_id": "n_gnd", "name": "GND"},
        {"type": "source_net", "source_net_id": "n_3v3", "name": "3V3"},
        {"type": "source_port", "source_port_id": "a", "net_id": "n_gnd"},
        {"type": "source_port", "source_port_id": "b", "net_id": "n_3v3"},
        {"type": "source_component_internal_connection",
         "source_component_internal_connection_id": "ic0", "source_port_ids": ["a", "b"]}
    ]"#,
    );
    assert_eq!(n.shorts().len(), 1);
    assert_eq!(n.shorts()[0].nets, ["3V3", "GND"]);
    assert_eq!(
        n.shorts()[0].trace,
        r#"source_component_internal_connection "ic0""#
    );
}

#[test]
fn internal_connection_to_a_missing_port_is_a_dangling_ref() {
    let n = netlist(
        r#"[
        {"type": "source_port", "source_port_id": "a"},
        {"type": "source_component_internal_connection",
         "source_component_internal_connection_id": "ic0", "source_port_ids": ["a", "ghost"]}
    ]"#,
    );
    assert_eq!(n.dangling_refs().len(), 1);
    assert_eq!(
        n.dangling_refs()[0].owner,
        r#"source_component_internal_connection "ic0""#
    );
    assert_eq!(n.dangling_refs()[0].target, r#"source_port "ghost""#);
    assert!(n.nets.is_empty());
}
