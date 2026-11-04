use super::kit_data::build_kit;

fn board(extra: &str) -> String {
    format!(
        r#"[
  {{"type":"pcb_board","width":40,"height":30,"center":{{"x":0,"y":0}},"thickness":1.6}},
  {extra}
]"#
    )
}

const EDGE_0402: &str = r#"
  {"type":"source_component","source_component_id":"sc1","name":"R1","ftype":"simple_resistor"},
  {"type":"pcb_component","pcb_component_id":"pc1","source_component_id":"sc1",
   "center":{"x":19.7,"y":0},"width":1.0,"height":0.5,"layer":"top"}"#;

const USB_C: &str = r#"
  {"type":"source_component","source_component_id":"sc2","name":"J1","ftype":"simple_chip",
   "supplier_part_numbers":{"lcsc":["C2765186"]}},
  {"type":"pcb_component","pcb_component_id":"pc2","source_component_id":"sc2",
   "center":{"x":17,"y":0},"width":7,"height":9,"layer":"top"},
  {"type":"cad_component","pcb_component_id":"pc2","position":{"x":17,"y":0,"z":0.8},"size":{"x":7,"y":9,"z":4.16}}"#;

#[test]
fn an_edge_resistor_gets_no_opening_and_a_connector_does() {
    let small = build_kit(&board(EDGE_0402), None).unwrap();
    assert!(small.components[0].touches_edge);
    assert!(small.openings.is_empty());

    let header = USB_C.replace(
        r#""ftype":"simple_chip""#,
        r#""ftype":"simple_usb_connector""#,
    );
    let kit = build_kit(&board(&header), None).unwrap();
    assert_eq!(kit.openings.len(), 1);
    assert_eq!(kit.openings[0].component, "J1");
}

#[test]
fn a_declared_cable_insertion_center_makes_an_opening() {
    let cable = USB_C.replace(
        r#""ftype":"simple_chip""#,
        r#""ftype":"simple_chip","cable_insertion_center":{"x":0,"y":0}"#,
    );
    let kit = build_kit(&board(&cable), None).unwrap();
    assert_eq!(kit.openings.len(), 1);
}

#[test]
fn the_verified_usb_c_height_replaces_the_model_height() {
    let kit = build_kit(&board(USB_C), None).unwrap();
    let keep = &kit.keep_out[0];
    assert!((keep.z1 - 3.96).abs() < 1e-9, "{}", keep.z1);
    assert!((keep.z0 + 1.28).abs() < 1e-9, "{}", keep.z0);
}

#[test]
fn an_unknown_part_keeps_the_model_height() {
    let other = USB_C.replace("C2765186", "C1");
    let kit = build_kit(&board(&other), None).unwrap();
    assert!(
        (kit.keep_out[0].z1 - 2.88).abs() < 1e-9,
        "{}",
        kit.keep_out[0].z1
    );
}
