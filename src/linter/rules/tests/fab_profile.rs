use super::physical::{board, lint_with};
use crate::linter::rules::{FabProfileRule, TraceClearanceRule};

const FAB: &str = "fab: jlcpcb-2l\n";

fn rules(json: &str) -> Vec<String> {
    lint_with(FabProfileRule, json, FAB)
        .into_iter()
        .map(|d| d.message)
        .collect()
}

#[test]
fn a_thin_trace_names_the_profile_and_its_minimum() {
    let json = board("", &["A"], &[("A", "top", 0.1, (0.0, 0.0), (5.0, 0.0))]);
    let v = lint_with(FabProfileRule, &json, FAB);
    assert_eq!(v.len(), 1);
    assert_eq!(
        v[0].message,
        "trace 0.10 mm on top is below jlcpcb-2l minimum 0.127 mm"
    );
    assert_eq!(v[0].standard.as_deref(), Some("jlcpcb-2l"));
    assert_eq!(
        v[0].source_url.as_deref(),
        Some("https://jlcpcb.com/capabilities/pcb-capabilities")
    );
    let ok = board("", &["A"], &[("A", "top", 0.127, (0.0, 0.0), (5.0, 0.0))]);
    assert!(rules(&ok).is_empty());
}

#[test]
fn tight_spacing_between_nets_is_reported_once() {
    let json = board(
        "",
        &["A", "B"],
        &[
            ("A", "top", 0.2, (0.0, 0.0), (5.0, 0.0)),
            ("B", "top", 0.2, (0.0, 0.3), (5.0, 0.3)),
        ],
    );
    let v = rules(&json);
    assert_eq!(
        v,
        vec!["spacing 0.10 mm on top between A and B is below jlcpcb-2l minimum 0.127 mm"]
    );
}

#[test]
fn a_set_fab_replaces_the_fixed_pad_clearance() {
    let json = r#"[
        {"type":"source_net","source_net_id":"a","name":"A"},
        {"type":"source_net","source_net_id":"b","name":"B"},
        {"type":"source_pad","source_pad_id":"p1","net_id":"a","x":0,"y":0,"width":1,"height":1},
        {"type":"source_pad","source_pad_id":"p2","net_id":"b","x":1.14,"y":0,"width":1,"height":1}]"#;
    assert_eq!(lint_with(TraceClearanceRule, json, "").len(), 1);
    assert!(lint_with(TraceClearanceRule, json, FAB).is_empty());
}

#[test]
fn drills_pads_rings_and_via_spacing_are_checked() {
    let vias = r#"{"type":"pcb_via","pcb_via_id":"v1","x":0,"y":0,"outer_diameter":0.5,"hole_diameter":0.25},
                  {"type":"pcb_via","pcb_via_id":"v2","x":0.6,"y":0,"outer_diameter":0.6,"hole_diameter":0.3}"#;
    let json = format!("[{vias}]");
    let msgs = rules(&json);
    assert!(
        msgs.iter()
            .any(|m| m == "via drill 0.25 mm is below jlcpcb-2l minimum 0.30 mm"),
        "{msgs:?}"
    );
    assert!(
        msgs.iter()
            .any(|m| m.starts_with("via pad 0.50 mm is below")),
        "{msgs:?}"
    );
    assert!(
        msgs.iter().any(|m| m.starts_with("via to via 0.05 mm")),
        "{msgs:?}"
    );
    let off = r#"[{"type":"pcb_hole","pcb_hole_id":"h","x":0,"y":0,"hole_diameter":3.22}]"#;
    assert!(rules(off)[0].contains("not a multiple of the jlcpcb-2l step 0.05 mm"));
    let fine = r#"[{"type":"pcb_hole","pcb_hole_id":"h","x":0,"y":0,"hole_diameter":3.2}]"#;
    assert!(rules(fine).is_empty());
}

#[test]
fn board_size_outside_the_limits_is_reported_and_no_fab_is_silent() {
    let small = r#"[{"type":"pcb_board","width":4,"height":30}]"#;
    assert!(rules(small)[0].starts_with("board 4.00 x 30.00 mm is outside jlcpcb-2l limits"));
    let rotated = r#"[{"type":"pcb_board","width":450,"height":300}]"#;
    assert!(rules(rotated).is_empty());
    assert!(lint_with(FabProfileRule, small, "name: x\n").is_empty());
}
