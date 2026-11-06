//! Pour clipping against board outlines and clearance around holes.

use super::super::*;
use super::board_with;

/// Returns the text of the first clear-polarity section of a Gerber file.
fn clear_section(gbr: &str) -> &str {
    let start = gbr.find("%LPC*%").expect("clear polarity");
    let rest = &gbr[start..];
    &rest[..rest.find("%LPD*%").expect("back to dark")]
}

#[test]
fn oversized_pour_is_clipped_beyond_one_millimetre_past_the_outline() {
    let pour = r#",{ "type": "pcb_copper_pour", "shape": "rect", "layer": "top",
      "center": { "x": 0, "y": 0 }, "width": 100, "height": 100 }"#;
    let gbr = generate_gerber_files(&board_with(pour), "T").unwrap()["F_Cu.gbr"].clone();
    assert!(clear_section(&gbr).contains("X51000000"), "{gbr}");
}

#[test]
fn pour_is_clipped_against_every_board_outline() {
    let extra = r#",{ "type": "pcb_board", "pcb_board_id": "b1", "width": 10, "height": 10,
      "center": { "x": 100, "y": 0 } },
      { "type": "pcb_copper_pour", "shape": "rect", "layer": "top",
      "center": { "x": 50, "y": 0 }, "width": 200, "height": 20 }"#;
    let gbr = generate_gerber_files(&board_with(extra), "T").unwrap()["F_Cu.gbr"].clone();
    assert!(clear_section(&gbr).contains("X95000000"), "{gbr}");
}

#[test]
fn non_plated_hole_gets_a_clear_ring() {
    let extra = r#",{ "type": "pcb_copper_pour", "shape": "rect", "layer": "top",
      "center": { "x": 0, "y": 0 }, "width": 20, "height": 20 },
      { "type": "pcb_hole", "x": 3, "y": 3, "hole_shape": "circle", "hole_diameter": 2 }"#;
    let gbr = generate_gerber_files(&board_with(extra), "T").unwrap()["F_Cu.gbr"].clone();
    assert!(
        clear_section(&gbr).contains("X3000000Y3000000D03*"),
        "{gbr}"
    );
}
