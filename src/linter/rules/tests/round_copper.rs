//! Round copper is measured as a circle: a diagonal gap is the distance
//! between two rims, not between the corners of two bounding boxes.

use serde_json::{json, Value};

use super::physical::lint_with;
use super::{lint, Diagnostic};
use crate::linter::rules::{TraceClearanceRule, VoltageClearanceRule};

const MAINS: &str = "nets:\n  MAINS_L: { voltage: 230 }\n";

fn nets() -> Vec<Value> {
    vec![
        json!({"type": "source_net", "source_net_id": "n_l", "name": "MAINS_L"}),
        json!({"type": "source_net", "source_net_id": "n_g", "name": "GND"}),
        json!({"type": "source_port", "source_port_id": "sp", "net_id": "n_l"}),
        json!({"type": "pcb_port", "pcb_port_id": "pp", "source_port_id": "sp"}),
    ]
}

fn via(id: &str, net: &str, at: f64) -> Value {
    json!({"type": "pcb_via", "pcb_via_id": id, "source_net_id": net, "x": at, "y": at,
        "outer_diameter": 0.6, "hole_diameter": 0.3, "layers": ["top", "bottom"]})
}

fn mains(extra: Vec<Value>) -> Vec<Diagnostic> {
    let elements: Vec<Value> = nets().into_iter().chain(extra).collect();
    lint_with(
        VoltageClearanceRule,
        &Value::Array(elements).to_string(),
        MAINS,
    )
}

#[test]
fn diagonal_vias_are_measured_rim_to_rim() {
    let v = mains(vec![via("v0", "n_l", 0.0), via("v1", "n_g", 1.2)]);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(
        v[0].message.contains("are 1.10 mm apart"),
        "{}",
        v[0].message
    );

    let clear = mains(vec![via("v0", "n_l", 0.0), via("v1", "n_g", 1.4)]);
    assert!(
        clear.is_empty(),
        "1.38 mm rim to rim clears 1.25 mm: {clear:?}"
    );
}

#[test]
fn a_round_plated_hole_is_a_circle_and_a_rect_pad_one_is_copper() {
    let hole = |shape: Value| {
        let mut hole = json!({"type": "pcb_plated_hole", "pcb_plated_hole_id": "h0",
            "pcb_port_id": "pp", "x": 0, "y": 0, "hole_diameter": 0.3});
        hole.as_object_mut()
            .unwrap()
            .extend(shape.as_object().unwrap().clone());
        hole
    };
    let round = hole(json!({"shape": "circle", "outer_diameter": 0.6}));
    assert!(mains(vec![round, via("v1", "n_g", 1.35)]).is_empty());

    let square = hole(json!({"shape": "circular_hole_with_rect_pad",
        "rect_pad_width": 0.6, "rect_pad_height": 0.6}));
    let v = mains(vec![square, via("v1", "n_g", 1.35)]);
    assert_eq!(v.len(), 1, "a square pad reaches into the corner: {v:?}");
}

#[test]
fn diagonal_round_pads_are_measured_rim_to_rim() {
    let pad = |id: &str, port: Option<&str>, at: f64| {
        json!({"type": "pcb_smtpad", "pcb_smtpad_id": id, "pcb_port_id": port, "layer": "top",
            "shape": "circle", "x": at, "y": at, "radius": 0.5})
    };
    let pads = |at: f64| {
        let elements: Vec<Value> = nets()
            .into_iter()
            .chain([pad("a", Some("pp"), 0.0), pad("b", None, at)])
            .collect();
        lint(TraceClearanceRule, &Value::Array(elements).to_string())
    };
    assert!(pads(0.82).is_empty(), "0.16 mm rim to rim clears 0.15 mm");
    assert_eq!(pads(0.80).len(), 1, "0.13 mm rim to rim does not");
}
