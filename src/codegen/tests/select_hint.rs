//! Tests for the hint printed when several chips could be the MCU.

use crate::codegen::select::hint_message;
use crate::parser::{Circuit, Component};
use serde_json::json;

#[test]
fn hint_names_each_candidate_chip_by_part_number() {
    let part = |id: &str, mpn: &str| {
        json!({"type": "source_component", "source_component_id": id, "ftype": "chip", "name": id,
            "manufacturer_part_number": mpn})
    };
    let board = Circuit::from_values(vec![part("U1", "OV2640"), part("U2", "ESP32-S3R8")])
        .expect("fixture reads");
    let chips: Vec<&Component> = board.components().collect();
    let hint = hint_message(&board, &chips);
    assert!(hint.contains("U1 [OV2640]"), "got: {hint}");
    assert!(hint.contains("U2 [ESP32-S3R8]"), "got: {hint}");
}
