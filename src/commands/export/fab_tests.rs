//! Tests for the fabrication bundle files.

use super::fab_files::bom_csv;
use super::pnp::pnp_csv;

const PNP_HEADER: &str = "designator,value,package,x,y,rotation,side\n";

const CIRCUIT: &str = r#"[
  {"type":"source_component","source_component_id":"sc0","name":"R1","ftype":"simple_resistor","resistance":"10k"},
  {"type":"pcb_component","pcb_component_id":"pc0","source_component_id":"sc0",
   "center":{"x":1.5,"y":-2},"rotation":90,"layer":"bottom"},
  {"type":"cad_component","pcb_component_id":"pc0","footprinter_string":"0402"}
]"#;

#[test]
fn pnp_lists_designator_package_position_and_side() {
    let csv = pnp_csv(CIRCUIT).unwrap();
    let row = csv.lines().nth(1).unwrap();
    assert_eq!(csv.lines().next().unwrap(), PNP_HEADER.trim_end());
    assert!(row.starts_with("R1,"), "{row}");
    assert!(row.ends_with(",0402,1.5000,-2.0000,90.00,bottom"), "{row}");
}

#[test]
fn bom_lists_the_mounted_part() {
    assert!(bom_csv(CIRCUIT).unwrap().contains("R1"));
}
