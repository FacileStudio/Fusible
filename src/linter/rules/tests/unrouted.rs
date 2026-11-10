//! `unrouted` rule tests.

use super::lint;
use crate::linter::rules::UnroutedRule;

#[test]
fn upstream_errors_are_not_unrouted() {
    let v = lint(
        UnroutedRule,
        r#"[{"type":"pcb_placement_error","message":"x"}]"#,
    );
    assert!(v.is_empty(), "{v:?}");
}

#[test]
fn clean_routed_circuit_passes() {
    let json =
        r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_trace"},{"type":"pcb_board"}]"#;
    assert!(lint(UnroutedRule, json).is_empty());
}

#[test]
fn nets_without_traces_blow() {
    let json = r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_board"}]"#;
    let v = lint(UnroutedRule, json);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].rule, "unrouted");
    assert!(v[0].message.contains("no routed traces"));
}

#[test]
fn schematic_only_circuit_passes() {
    let json = r#"[{"type":"source_net","source_net_id":"n"}]"#;
    assert!(lint(UnroutedRule, json).is_empty());
}

#[test]
fn pour_only_board_with_no_source_trace_passes() {
    let json = r#"[{"type":"source_net","source_net_id":"n"},{"type":"pcb_board"},{"type":"pcb_copper_pour"}]"#;
    assert!(lint(UnroutedRule, json).is_empty());
}
