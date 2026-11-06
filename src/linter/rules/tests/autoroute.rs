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

#[test]
fn pour_only_board_with_no_source_trace_passes() {
    let json = r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_board"},{"type":"pcb_copper_pour"}]"#;
    assert!(lint(AutorouteRule, json).is_empty());
}

#[test]
fn source_trace_without_pcb_trace_blows_even_when_others_routed() {
    let json = r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_board"},
        {"type":"source_trace","source_trace_id":"a"},{"type":"source_trace","source_trace_id":"b"},
        {"type":"pcb_trace","source_trace_id":"a"}]"#;
    let v = lint(AutorouteRule, json);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(v[0].message.ends_with(": b"));
}
