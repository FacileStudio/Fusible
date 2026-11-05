//! Unit tests for copper pour regions and their cutouts.

use super::super::*;
use super::board_with;

const NET_PREAMBLE: &str = r#",
    { "type": "source_net", "source_net_id": "n_gnd", "subcircuit_connectivity_map_key": "gnd" },
    { "type": "source_port", "source_port_id": "sp_gnd", "subcircuit_connectivity_map_key": "gnd" },
    { "type": "source_port", "source_port_id": "sp_sig", "subcircuit_connectivity_map_key": "sig" },
    { "type": "pcb_port", "pcb_port_id": "pp_gnd", "source_port_id": "sp_gnd" },
    { "type": "pcb_port", "pcb_port_id": "pp_sig", "source_port_id": "sp_sig" },
    { "type": "pcb_copper_pour", "shape": "rect", "layer": "bottom", "source_net_id": "n_gnd",
      "center": { "x": 0, "y": 0 }, "width": 20, "height": 20 }"#;

fn bottom_copper_with(pad_port: &str) -> String {
    let pad = format!(
        r#"{NET_PREAMBLE},
    {{ "type": "pcb_smtpad", "shape": "rect", "layer": "bottom", "x": 2, "y": 2,
      "width": 1, "height": 1, "pcb_port_id": "{pad_port}" }}"#
    );
    generate_gerber_files(&board_with(&pad), "T").unwrap()["B_Cu.gbr"].clone()
}

#[test]
fn rect_pour_emits_region_on_its_layer_only() {
    let json = board_with(NET_PREAMBLE);
    let files = generate_gerber_files(&json, "T").unwrap();
    assert!(files["B_Cu.gbr"].contains("G36*") && files["B_Cu.gbr"].contains("G37*"));
    assert!(!files["F_Cu.gbr"].contains("G36*"));
}

/// Returns the text of the first clear-polarity section of a Gerber file.
fn clear_section(gbr: &str) -> &str {
    let start = gbr.find("%LPC*%").expect("clear polarity");
    let rest = &gbr[start..];
    &rest[..rest.find("%LPD*%").expect("back to dark")]
}

#[test]
fn pad_of_another_net_gets_a_clear_cutout() {
    let gbr = bottom_copper_with("pp_sig");
    assert!(clear_section(&gbr).contains("X2000000Y2000000D03*"));
}

#[test]
fn pad_of_the_same_net_gets_no_cutout() {
    let gbr = bottom_copper_with("pp_gnd");
    assert!(!clear_section(&gbr).contains("X2000000Y2000000D03*"));
    assert!(gbr.contains("X2000000Y2000000D03*"));
}

#[test]
fn brep_pour_emits_outer_region_and_clear_inner_ring() {
    let pour = r#",{ "type": "pcb_copper_pour", "shape": "brep", "layer": "top",
      "brep_shape": { "outer_ring": { "vertices": [
          { "x": -5, "y": -5 }, { "x": 5, "y": -5 }, { "x": 5, "y": 5 }, { "x": -5, "y": 5 } ] },
        "inner_rings": [ { "vertices": [
          { "x": -1, "y": -1 }, { "x": 1, "y": -1 }, { "x": 1, "y": 1 } ] } ] } }"#;
    let gbr = generate_gerber_files(&board_with(pour), "T").unwrap()["F_Cu.gbr"].clone();
    assert!(gbr.contains("G36*"));
    assert!(clear_section(&gbr).contains("X-1000000Y-1000000D02*"));
}
