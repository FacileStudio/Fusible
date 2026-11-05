//! `decoupling_capacitor` rule tests.

use super::lint;
use crate::linter::rules::BypassCapRule;

/// A chip whose VDD pin reaches rail `V3_3` only through a `source_trace`,
/// next to a second component of the given name and `ftype` on that rail.
fn rail_with(name: &str, ftype: &str) -> String {
    format!(
        r#"[
        {{"type": "source_component", "source_component_id": "source_component_0",
          "name": "MCU", "ftype": "simple_chip"}},
        {{"type": "source_component", "source_component_id": "source_component_1",
          "name": "{name}", "ftype": "{ftype}"}},
        {{"type": "source_net", "source_net_id": "source_net_0", "name": "V3_3"}},
        {{"type": "source_port", "source_port_id": "source_port_0", "name": "VDD",
          "source_component_id": "source_component_0"}},
        {{"type": "source_port", "source_port_id": "source_port_1", "name": "pin1",
          "source_component_id": "source_component_1"}},
        {{"type": "source_trace", "source_trace_id": "source_trace_0",
          "connected_source_port_ids": ["source_port_0", "source_port_1"],
          "connected_source_net_ids": ["source_net_0"]}}
    ]"#
    )
}

#[test]
fn bypass_cap_flags_a_traced_rail_without_capacitor() {
    let v = lint(BypassCapRule, &rail_with("C9", "simple_resistor"));
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].rule, "decoupling_capacitor");
    assert!(v[0].message.contains("MCU"));
    assert!(v[0].message.contains("V3_3"));
}

#[test]
fn bypass_cap_accepts_a_capacitor_named_unlike_c() {
    assert!(lint(BypassCapRule, &rail_with("BYP", "simple_capacitor")).is_empty());
}

fn placed(cap_x: f64) -> String {
    let pad = |id: &str, port: &str, x: f64| {
        format!(
            r#"{{"type": "pcb_smtpad", "pcb_smtpad_id": "{id}", "source_port_id": "{port}", "layer": "top",
              "shape": "rect", "x": {x}, "y": 0, "width": 1, "height": 1}}"#
        )
    };
    let base = rail_with("C3", "simple_capacitor");
    let base = base.trim_end().trim_end_matches(']');
    format!(
        "{base}, {}, {}]",
        pad("pad_u", "source_port_0", 0.0),
        pad("pad_c", "source_port_1", cap_x)
    )
}

#[test]
fn bypass_cap_far_from_its_pin_warns_with_the_distance() {
    let v = lint(BypassCapRule, &placed(7.2));
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].severity, crate::linter::Severity::Warning);
    assert!(
        v[0].message
            .contains("C3 decouples MCU pin VDD from 6.2 mm away"),
        "{}",
        v[0].message
    );
    assert!(v[0].message.contains("place it within 3 mm"));
    assert_eq!(v[0].standard.as_deref(), Some("layout guideline"));
    assert!(v[0].help.as_deref().unwrap().contains("protoexpress.com"));
}

#[test]
fn bypass_cap_within_three_millimetres_or_unplaced_is_silent() {
    assert!(lint(BypassCapRule, &placed(3.9)).is_empty());
    assert!(lint(BypassCapRule, &rail_with("C3", "simple_capacitor")).is_empty());
}
