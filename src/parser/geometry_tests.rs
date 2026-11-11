use std::path::Path;

use super::geometry::{read, Board, Segment};
use super::parse;

const FIXTURE: &str = r#"[
  {"type":"source_net","source_net_id":"source_net_1","name":"VBUS"},
  {"type":"source_trace","source_trace_id":"source_trace_0",
   "connected_source_net_ids":["source_net_1"]},
  {"type":"pcb_board","width":40,"height":30,"thickness":1.6,"num_layers":2},
  {"type":"pcb_trace","pcb_trace_id":"pcb_trace_0","source_trace_id":"source_trace_0","route":[
    {"route_type":"wire","x":1,"y":2,"width":0.25,"layer":"top"},
    {"route_type":"wire","x":5,"y":2,"width":0.25,"layer":"top"},
    {"route_type":"via","x":6,"y":2,"from_layer":"top","to_layer":"bottom"},
    {"route_type":"wire","x":6,"y":8,"width":0.4,"layer":"bottom"}]},
  {"type":"pcb_via","pcb_via_id":"v1","x":3,"y":2,"outer_diameter":0.6,"hole_diameter":0.3},
  {"type":"pcb_plated_hole","pcb_plated_hole_id":"h1","shape":"circle","x":9,"y":9,
   "hole_diameter":1.0,"outer_diameter":1.6},
  {"type":"pcb_plated_hole","pcb_plated_hole_id":"h2","shape":"pill","x":9,"y":3,
   "hole_width":1.0,"hole_height":2.0,"outer_width":1.8,"outer_height":2.8},
  {"type":"pcb_hole","pcb_hole_id":"h3","hole_diameter":3.2,"x":1,"y":1}
]"#;

fn geometry(json: &str) -> super::geometry::Geometry {
    read(&parse(json, Path::new("c.json")).expect("fixture parses"))
}

#[test]
fn reads_segments_with_net_layer_and_start_width() {
    let g = geometry(FIXTURE);
    assert_eq!(g.segments.len(), 3);
    let first = Segment {
        trace_id: "pcb_trace_0".into(),
        net_names: vec!["VBUS".into()],
        layer: "top".into(),
        a: (1.0, 2.0),
        b: (5.0, 2.0),
        width: 0.25,
    };
    assert_eq!(g.segments[0], first);
    assert_eq!(
        (g.segments[1].layer.as_str(), g.segments[1].width),
        ("top", 0.25)
    );
    assert_eq!(
        (g.segments[2].layer.as_str(), g.segments[2].width),
        ("bottom", 0.4)
    );
}

#[test]
fn reads_vias_holes_and_board() {
    let g = geometry(FIXTURE);
    assert_eq!((g.vias[0].outer, g.vias[0].hole), (0.6, 0.3));
    assert_eq!((g.holes[0].hole, g.holes[0].outer), (1.0, Some(1.6)));
    assert_eq!((g.holes[1].hole, g.holes[1].outer), (1.0, Some(1.8)));
    assert_eq!((g.holes[2].hole, g.holes[2].outer), (3.2, None));
    let board = Board {
        width: 40.0,
        height: 30.0,
        thickness: 1.6,
        layers: 2,
    };
    assert_eq!(g.board, Some(board));
    assert!(g.unreadable.is_empty());
}

#[test]
fn resolves_nets_through_ports_when_the_trace_names_none() {
    let json = r#"[
      {"type":"source_net","net_id":"n1","name":"GND"},
      {"type":"source_port","port_id":"p1","net_id":"n1"},
      {"type":"source_trace","trace_id":"t1","connected_port_ids":["p1"]},
      {"type":"pcb_trace","pcb_trace_id":"pt","source_trace_id":"t1","route":[
        {"route_type":"wire","x":0,"y":0,"width":0.2,"layer":"top"},
        {"route_type":"wire","x":1,"y":0,"width":0.2,"layer":"top"}]}
    ]"#;
    assert_eq!(
        geometry(json).segments[0].net_names,
        vec!["GND".to_string()]
    );
}

#[test]
fn lists_elements_it_cannot_read() {
    let json = r#"[
      {"type":"pcb_trace","pcb_trace_id":"pt","route":[
        {"route_type":"wire","x":0,"y":0,"layer":"top"},{"route_type":"wire","x":1,"y":0,"width":0.2,"layer":"top"}]},
      {"type":"pcb_via","pcb_via_id":"v","x":0,"y":0},
      {"type":"pcb_board"}
    ]"#;
    let g = geometry(json);
    assert_eq!(g.unreadable.len(), 3);
    assert!(g.segments.is_empty() && g.vias.is_empty() && g.board.is_none());
}

#[test]
fn a_segment_leaving_a_via_lies_on_the_layer_of_its_wire() {
    let g = geometry(
        r#"[
  {"type":"pcb_trace","pcb_trace_id":"t","route":[
    {"route_type":"wire","x":0,"y":0,"width":0.15,"layer":"top"},
    {"route_type":"via","x":0,"y":0,"from_layer":"inner1","to_layer":"top"},
    {"route_type":"wire","x":0,"y":0,"width":1.0,"layer":"inner1"},
    {"route_type":"wire","x":3,"y":0,"width":0.15,"layer":"inner1"}]}
]"#,
    );
    let read: Vec<(&str, f64)> = g
        .segments
        .iter()
        .map(|s| (s.layer.as_str(), s.width))
        .collect();
    assert_eq!(read, [("top", 0.15), ("inner1", 1.0), ("inner1", 1.0)]);
}
