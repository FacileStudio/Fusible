//! Tests for MCU component selection.

use crate::codegen::select::select_mcu_component;
use crate::parser::Circuit;
use serde_json::{json, Value};

fn component(component_id: &str, kind: &str, mpn: Option<&str>) -> Value {
    json!({
        "type": "source_component",
        "source_component_id": component_id,
        "ftype": kind,
        "name": component_id,
        "manufacturer_part_number": mpn,
    })
}

fn chip(component_id: &str, mpn: &str) -> Value {
    component(component_id, "chip", Some(mpn))
}

fn circuit(values: Vec<Value>) -> Circuit {
    Circuit::from_values(values).expect("fixture reads")
}

fn selected_id(circuit: &Circuit, target: Option<&str>) -> Option<String> {
    select_mcu_component(circuit, target).and_then(|c| c.component_id.clone())
}

#[test]
fn passive_named_mcu_is_never_selected() {
    let board = circuit(vec![
        component("C_MCU", "simple_capacitor", None),
        chip("U10", "AP2112K-3.3"),
        chip("U1", "ESP32-S3R8"),
    ]);
    assert_eq!(selected_id(&board, None).as_deref(), Some("U1"));
}

#[test]
fn target_matches_the_part_number_of_a_chip() {
    let board = circuit(vec![chip("U1", "OV2640"), chip("U2", "ESP32-S3R8")]);
    assert_eq!(selected_id(&board, Some("esp32-s3")).as_deref(), Some("U2"));
}

#[test]
fn two_unknown_chips_without_target_select_nothing() {
    let board = circuit(vec![
        component("U1", "chip", None),
        component("U2", "chip", None),
    ]);
    assert!(select_mcu_component(&board, None).is_none());
}

#[test]
fn single_chip_is_selected_even_when_unknown() {
    let board = circuit(vec![
        component("C1", "simple_capacitor", None),
        component("U7", "chip", None),
    ]);
    assert_eq!(selected_id(&board, None).as_deref(), Some("U7"));
}
