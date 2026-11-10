//! Tests for the typed model, on elements copied from a real tscircuit build.

use std::path::Path;

use super::model::{Model, RoutePoint, UpstreamIssue};
use super::*;

fn model(source: &str) -> Model {
    parse(source, Path::new("circuit.json"))
        .expect("fixture parses")
        .model()
        .clone()
}

fn parts(source: &str) -> Vec<ComponentProps> {
    let circuit = parse(source, Path::new("circuit.json")).expect("fixture parses");
    circuit.components().map(|c| (*c.props).clone()).collect()
}

#[test]
fn reads_fieldcam_board_and_placement() {
    let m = model(include_str!("fixtures/fieldcam_pcb.json"));

    assert!(m.unreadable.is_empty(), "{:?}", m.unreadable);
    let board = &m.boards[0];
    assert_eq!((board.width, board.height), (Some(40.0), Some(154.0)));
    assert_eq!((board.thickness, board.num_layers), (Some(1.4), Some(4)));
    assert_eq!(board.min_board_edge_clearance, Some(0.2));
    let part = &m.components[0];
    assert_eq!(part.source_component_id, "source_component_0");
    assert_eq!(
        (part.center.x, part.center.y, part.layer.as_str()),
        (0.0, 22.0, "top")
    );
}

#[test]
fn reads_fieldcam_routes_drills_and_pours() {
    let m = model(include_str!("fixtures/fieldcam_pcb.json"));

    let route = &m.traces[0].route;
    assert!(matches!(&route[0], RoutePoint::Wire { width, .. } if *width == 0.15));
    assert!(matches!(&route[1], RoutePoint::Via { to_layer, .. } if to_layer == "top"));
    assert_eq!(
        (m.vias[0].hole_diameter, m.vias[0].outer_diameter),
        (0.2, 0.3)
    );
    assert_eq!(m.plated_holes[0].hole_diameter, Some(1.0));
    assert_eq!(m.plated_holes[0].rect_pad_width, Some(1.5));
    assert_eq!(
        (
            m.holes[0].hole_diameter,
            m.holes[0].pcb_component_id.as_deref()
        ),
        (Some(3.2), None)
    );
    let pour = &m.copper_pours[0];
    assert_eq!(pour.shape, "brep");
    assert_eq!(
        pour.brep_shape
            .as_ref()
            .map(|b| b.outer_ring.vertices.len()),
        Some(4)
    );
}

#[test]
fn reads_fieldcam_silkscreen_courtyards_and_parts() {
    let m = model(include_str!("fixtures/fieldcam_pcb.json"));

    assert_eq!(m.silkscreen_texts[0].text, "U1");
    assert_eq!(m.silkscreen_texts[0].font_size, 0.5);
    assert_eq!(m.silkscreen_paths[0].route.len(), 2);
    assert_eq!(
        (m.courtyard_rects[0].width, m.courtyard_rects[0].height),
        (2.96, 1.46)
    );
    assert_eq!(m.courtyard_outlines[0].outline.len(), 4);
    let joined = &m.internal_connections[0].source_port_ids;
    assert_eq!(joined, &["source_port_182", "source_port_183"]);
    let parts = parts(include_str!("fixtures/fieldcam_pcb.json"));
    let resistor = &parts[0];
    assert_eq!(resistor.resistance, Some(10_000.0));
    assert_eq!(
        resistor.manufacturer_part_number.as_deref(),
        Some("RC0603FR-0710KL")
    );
    assert_eq!(resistor.supplier_part_numbers["jlcpcb"].len(), 3);
    assert_eq!(parts[1].capacitance, Some(0.000001));
    assert_eq!(parts[1].display_capacitance.as_deref(), Some("1uF"));
}

#[test]
fn part_values_sent_as_strings_read_as_numbers() {
    let json = r#"[{"type": "source_component", "source_component_id": "c1", "ftype": "simple_capacitor",
        "name": "C1", "capacitance": "100nF", "max_voltage_rating": "16V", "display_value": "100n"}]"#;
    let part = &parts(json)[0];
    assert!((part.capacitance.expect("capacitance") - 1e-7).abs() < 1e-18);
    assert_eq!(part.max_voltage_rating, Some(16.0));
    assert_eq!(part.display_value.as_deref(), Some("100n"));
}

#[test]
fn malformed_typed_elements_are_listed_not_dropped() {
    let json = r#"[
        {"type": "pcb_via", "pcb_via_id": "pcb_via_9", "x": 1, "y": 2},
        {"type": "pcb_trace", "pcb_trace_id": "t1", "route": [{"route_type": "arc", "x": 0, "y": 0}]},
        {"type": "source_component", "source_component_id": "r1", "resistance": "lots"},
        {"type": "pcb_smtpad", "pcb_smtpad_id": "p1"}
    ]"#;
    let m = model(json);
    assert_eq!(m.unreadable.len(), 3, "{:?}", m.unreadable);
    assert!(m.unreadable[0].starts_with("pcb_via_9: pcb_via:"));
    assert!(m.unreadable[1].starts_with("t1: pcb_trace:"));
    assert!(m.unreadable[2].starts_with("r1: source_component:"));
    assert!(m.vias.is_empty() && m.traces.is_empty());
    assert!(parts(json)[0].unreadable.is_some());
}

#[test]
fn upstream_errors_and_warnings_become_issues() {
    let m = model(include_str!("fixtures/upstream_issues.json"));

    assert_eq!(m.issues.len(), 4);
    let trace = &m.issues[0];
    assert_eq!(
        trace,
        &UpstreamIssue {
            kind: "pcb_trace_error".into(),
            id: Some("pcb_trace_error_0".into()),
            message: Some("Trace too close to pad".into()),
            element_ids: [
                "pcb_component_1",
                "pcb_component_2",
                "pcb_port_7",
                "pcb_trace_3",
                "source_trace_3"
            ]
            .map(String::from)
            .to_vec(),
            is_error: true,
        }
    );
    assert!(!m.issues[1].is_error);
    assert_eq!(m.issues[1].element_ids, ["source_trace_0"]);
    assert_eq!(m.issues[2].element_ids, ["source_component_5"]);
    let autoroute = &m.issues[3];
    assert_eq!(autoroute.id.as_deref(), Some("pcb_autorouting_error_0"));
    assert!(autoroute.is_error && autoroute.message.is_none() && autoroute.element_ids.is_empty());
}
