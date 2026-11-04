use super::place::IDENTITY;
use super::probe::probes;
use super::rules::{missing_bosses, Measurement, HOLE_MISALIGNED};
use super::rules_support::{board_probes, hit, rules};
use crate::commands::body::build_kit;

fn boss() -> Vec<Measurement> {
    let mut hits = vec![hit("main:hole:0:top:near", 5.0)];
    hits.extend((0..4).map(|q| hit(&format!("main:hole:0:top:q{q}"), 1.0)));
    hits
}

fn fitted() -> Vec<(String, String)> {
    vec![("case".into(), "main".into())]
}

fn plated_board_probes() -> Vec<super::probe::Probe> {
    let json = r#"[
      {"type":"pcb_board","width":40,"height":30,"center":{"x":0,"y":0},"thickness":1.4},
      {"type":"pcb_plated_hole","x":-15,"y":10,"shape":"circle","hole_diameter":1.0,"outer_diameter":1.6}
    ]"#;
    probes(
        "main",
        &build_kit(json, Some("main".into())).unwrap(),
        &IDENTITY,
        1.0,
    )
}

#[test]
fn a_hole_with_no_material_near_it_has_no_boss() {
    let found = missing_bosses(&board_probes(), &[], &fitted());
    assert_eq!(rules(&found), [HOLE_MISALIGNED]);
    assert_eq!(found[0].parts, ["case", "main"]);
    assert!(
        found[0]
            .message
            .contains("no boss for the hole at (-15.00, 10.00) of board `main`"),
        "{}",
        found[0].message
    );
}

#[test]
fn a_boss_far_off_the_hole_leaves_it_without_a_boss() {
    let far = [hit("main:slab", 3.0)];
    let found = missing_bosses(&board_probes(), &far, &fitted());
    assert_eq!(rules(&found), [HOLE_MISALIGNED]);
}

#[test]
fn a_plated_hole_needs_no_boss() {
    assert!(missing_bosses(&plated_board_probes(), &[], &fitted()).is_empty());
}

#[test]
fn an_aligned_boss_is_not_missing() {
    assert!(missing_bosses(&board_probes(), &boss(), &fitted()).is_empty());
}

#[test]
fn a_body_not_fitted_to_the_board_is_not_asked_for_a_boss() {
    assert!(missing_bosses(&board_probes(), &[], &[]).is_empty());
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
