use super::place::IDENTITY;
use super::probe::probes;
use super::rules::{missing_bosses, HOLE_MISALIGNED};
use super::rules_support::rules;
use crate::commands::body::build_kit;

fn fitted() -> Vec<(String, String)> {
    vec![("case".into(), "main".into())]
}

fn probes_of(holes: &str, min_hole: f64) -> Vec<super::probe::Probe> {
    let json = format!(
        r#"[
      {{"type":"pcb_board","width":40,"height":30,"center":{{"x":0,"y":0}},"thickness":1.4}},
      {holes},
      {{"type":"pcb_component","pcb_component_id":"pc1","center":{{"x":10,"y":0}},"width":6,"height":6}}
    ]"#
    );
    let kit = build_kit(&json, Some("main".into())).unwrap();
    probes("main", &kit, &IDENTITY, min_hole)
}

#[test]
fn a_hole_under_a_component_needs_no_boss() {
    let hole = r#"{"type":"pcb_hole","x":10,"y":1,"hole_diameter":3.2}"#;
    assert!(missing_bosses(&probes_of(hole, 1.0), &[], &fitted()).is_empty());
}

#[test]
fn a_bare_hole_still_needs_a_boss() {
    let hole = r#"{"type":"pcb_hole","x":-10,"y":1,"hole_diameter":3.2}"#;
    let found = missing_bosses(&probes_of(hole, 1.0), &[], &fitted());
    assert_eq!(rules(&found), [HOLE_MISALIGNED]);
}

#[test]
fn a_hole_under_the_minimum_diameter_needs_no_boss() {
    let hole = r#"{"type":"pcb_hole","x":-10,"y":1,"hole_diameter":0.8}"#;
    assert!(missing_bosses(&probes_of(hole, 1.0), &[], &fitted()).is_empty());
}

#[test]
fn the_minimum_diameter_is_configurable() {
    let hole = r#"{"type":"pcb_hole","x":-10,"y":1,"hole_diameter":3.2}"#;
    assert!(missing_bosses(&probes_of(hole, 4.0), &[], &fitted()).is_empty());
    assert!(!missing_bosses(&probes_of(hole, 3.0), &[], &fitted()).is_empty());
}
