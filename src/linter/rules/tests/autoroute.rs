//! `autoroute` rule tests.

use super::lint;
use crate::linter::rules::AutorouteRule;

#[test]
fn placement_error_blows() {
    let v = lint(
        AutorouteRule,
        r#"[{"type":"pcb_placement_error","message":"x"}]"#,
    );
    assert_eq!(v.len(), 1);
    assert!(v[0].message.contains("pcb_placement_error: x"));
}

#[test]
fn thermalpad_error_carries_the_pinlabel_hint() {
    let json = r#"[{"type":"source_trace_not_connected_error","message":"no thermalpad port"}]"#;
    let v = lint(AutorouteRule, json);
    assert!(v[0]
        .help
        .as_deref()
        .unwrap()
        .contains("pinLabel named thermalpad"));
}

#[test]
fn clean_routed_circuit_passes() {
    let json =
        r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_trace"},{"type":"pcb_board"}]"#;
    assert!(lint(AutorouteRule, json).is_empty());
}

#[test]
fn nets_without_traces_blow() {
    let json = r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_board"}]"#;
    let v = lint(AutorouteRule, json);
    assert_eq!(v.len(), 1);
    assert!(v[0].message.contains("no routed traces"));
}

#[test]
fn schematic_only_circuit_passes() {
    let json = r#"[{"type":"source_net","source_net_id":"n"}]"#;
    assert!(lint(AutorouteRule, json).is_empty());
}
