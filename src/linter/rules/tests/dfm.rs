//! Edge cases of the DFM and DFA rules the example fixtures do not cover.

use super::physical::lint_with;
use crate::linter::rules::{
    AcidTrapRule, AnnularRingRule, CourtyardClearanceRule, EdgeClearanceRule,
};

const FAB: &str = "fab: jlcpcb-2l\n";

fn courtyards(rotation: f64, own: &str) -> String {
    format!(
        r#"[{{"type":"pcb_component","pcb_component_id":"pc0","source_component_id":"a","center":{{"x":0,"y":0}},
            "width":1,"height":1,"layer":"top","rotation":{rotation}}},
           {{"type":"pcb_courtyard_rect","pcb_courtyard_rect_id":"c0","pcb_component_id":"pc0","layer":"top",
            "center":{{"x":0,"y":0}},"width":4,"height":1{own}}},
           {{"type":"pcb_courtyard_rect","pcb_courtyard_rect_id":"c1","pcb_component_id":"pc1","layer":"top",
            "center":{{"x":0,"y":1.5}},"width":1,"height":1}}]"#
    )
}

#[test]
fn courtyards_touching_or_apart_pass_and_own_rotation_wins() {
    assert!(lint_with(CourtyardClearanceRule, &courtyards(0.0, ""), "").is_empty());
    assert_eq!(
        lint_with(CourtyardClearanceRule, &courtyards(90.0, ""), "").len(),
        1
    );
    let own = r#","ccw_rotation":0"#;
    assert!(lint_with(CourtyardClearanceRule, &courtyards(90.0, own), "").is_empty());
}

#[test]
fn via_rings_use_the_via_limit_not_the_plated_hole_one() {
    let via = r#"[{"type":"pcb_via","pcb_via_id":"v","x":0,"y":0,"outer_diameter":0.3,"hole_diameter":0.2}]"#;
    assert!(lint_with(AnnularRingRule, via, FAB).is_empty());
    let thin = r#"[{"type":"pcb_via","pcb_via_id":"v","x":0,"y":0,"outer_diameter":0.25,"hole_diameter":0.2}]"#;
    let v = lint_with(AnnularRingRule, thin, FAB);
    assert_eq!(
        v[0].message,
        "via v annular ring 0.025 mm is below jlcpcb-2l minimum 0.05 mm"
    );
    assert_eq!(v[0].location.as_ref().map(|l| (l.x, l.y)), Some((0.0, 0.0)));
    assert!(lint_with(AnnularRingRule, thin, "").is_empty());
}

#[test]
fn a_board_without_a_centre_has_no_edge_to_measure() {
    let json = r#"[{"type":"pcb_board","width":10,"height":10},
        {"type":"source_net","source_net_id":"n","name":"A"},
        {"type":"source_trace","source_trace_id":"t","connected_source_net_ids":["n"]},
        {"type":"pcb_trace","pcb_trace_id":"p","source_trace_id":"t","route":[
          {"route_type":"wire","x":0,"y":0,"width":0.2,"layer":"top"},
          {"route_type":"wire","x":5,"y":0,"width":0.2,"layer":"top"}]}]"#;
    assert!(lint_with(EdgeClearanceRule, json, FAB).is_empty());
}

#[test]
fn a_short_stub_fills_its_own_wedge() {
    let route = |x: f64| {
        format!(
            r#"[{{"type":"pcb_trace","pcb_trace_id":"p","route":[
              {{"route_type":"wire","x":0,"y":0,"width":0.2,"layer":"top"}},
              {{"route_type":"wire","x":{x},"y":0,"width":0.2,"layer":"top"}},
              {{"route_type":"wire","x":-5,"y":5,"width":0.2,"layer":"top"}}]}}]"#
        )
    };
    assert_eq!(lint_with(AcidTrapRule, &route(5.0), "").len(), 1);
    assert!(lint_with(AcidTrapRule, &route(0.05), "").is_empty());
}
