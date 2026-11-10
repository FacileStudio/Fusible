//! Tests for the centroid, IPC-D-356A netlist and README writers.

use std::path::Path;

use super::{centroid_csv, ipc356, readme, CENTROID_HEADER};
use crate::config::Date;
use crate::netlist::Netlist;
use crate::parser::parse;

const PLACED: &str = r#"[
  {"type":"source_component","source_component_id":"sc0","name":"R1","ftype":"simple_resistor","resistance":"10k"},
  {"type":"pcb_component","pcb_component_id":"pc0","source_component_id":"sc0",
   "center":{"x":1.5,"y":-2},"rotation":90,"layer":"bottom"},
  {"type":"cad_component","pcb_component_id":"pc0","footprinter_string":"0402"}
]"#;

const BOARD: &str = r#"[
  {"type":"pcb_board","width":20,"height":20,"num_layers":2},
  {"type":"source_component","source_component_id":"sc0","name":"R1","ftype":"simple_resistor"},
  {"type":"source_net","source_net_id":"n_vbus","name":"VERY_LONG_POWER_RAIL"},
  {"type":"source_port","source_port_id":"sp0","name":"pin1","pin_number":1,
   "source_component_id":"sc0","source_net_id":"n_vbus"},
  {"type":"pcb_port","pcb_port_id":"pp0","source_port_id":"sp0"},
  {"type":"pcb_smtpad","pcb_smtpad_id":"pad0","pcb_port_id":"pp0","shape":"rect",
   "x":-1.5,"y":2.25,"width":0.6,"height":0.5,"layer":"bottom"},
  {"type":"pcb_hole","pcb_hole_id":"h0","hole_shape":"circle","hole_diameter":3.2,"x":5,"y":5},
  {"type":"pcb_via","pcb_via_id":"v0","x":1,"y":1,"outer_diameter":0.6,"hole_diameter":0.3,"layers":["top","bottom"]}
]"#;

fn netlist_of(json: &str) -> String {
    let circuit = parse(json, Path::new("circuit.json")).unwrap();
    let netlist = Netlist::from_circuit(&circuit);
    ipc356(&circuit, &netlist, "demo")
}

#[test]
fn centroid_names_units_and_rotation_in_the_header() {
    let csv = centroid_csv(PLACED).unwrap();
    let mut lines = csv.lines();
    assert_eq!(lines.next().unwrap(), CENTROID_HEADER.trim_end());
    assert!(CENTROID_HEADER.contains("(mm)") && CENTROID_HEADER.contains("deg CCW"));
    let row = lines.next().unwrap();
    assert!(row.starts_with("R1,"), "{row}");
    assert!(row.ends_with(",0402,1.5000,-2.0000,90.00,bottom"), "{row}");
}

#[test]
fn ipc356_writes_fixed_columns_for_pads_holes_and_vias() {
    let text = netlist_of(BOARD);
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines.contains(&"P  UNITS CUST 1"), "{text}");
    assert!(
        lines.contains(&"P  NNAME1   VERY_LONG_POWER_RAIL"),
        "{text}"
    );
    let pad = lines.iter().find(|l| l.starts_with("327")).unwrap();
    assert_eq!(&pad[3..17], "NNAME1        ");
    assert_eq!(&pad[20..31], "R1    -1   ");
    assert_eq!(&pad[38..41], "A02");
    assert_eq!(&pad[41..57], "X-001500Y+002250");
    assert_eq!(&pad[57..67], "X0600Y0500");
    let hole = lines.iter().find(|l| l.starts_with("367")).unwrap();
    assert_eq!(&hole[32..38], "D3200U");
    let via = lines.iter().find(|l| l.contains("VIA")).unwrap();
    assert!(via.starts_with("317"), "{via}");
    assert_eq!(&via[32..41], "D0300PA00");
    assert_eq!(lines.last(), Some(&"999"));
}

#[test]
fn readme_maps_every_layer_and_names_board_version_and_date() {
    let date = Date::parse("2026-11-10").unwrap();
    let text = readme(
        "main",
        date,
        ["F_Cu.gbr", "In1_Cu.gbr", "drill_npth.drl"].into_iter(),
    );
    assert!(text.starts_with("Board: main\n"), "{text}");
    assert!(text.contains(env!("CARGO_PKG_VERSION")));
    assert!(text.contains("Date: 2026-11-10"));
    assert!(text.contains("F_Cu.gbr         top copper"), "{text}");
    assert!(text.contains("In1_Cu.gbr       inner copper 1"), "{text}");
    assert!(
        text.contains("drill_npth.drl   non-plated drills"),
        "{text}"
    );
}
