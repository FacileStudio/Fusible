//! Polygon pads in `route_circuit`.

use super::fixtures::{circuit, connect, pad};
use crate::router::engine::{route_circuit, RouteOptions};

#[test]
fn a_polygon_pad_routes_from_the_center_of_its_outline() {
    let polygon = |points: &str| {
        format!(
            r#"{{ "type": "source_port", "source_port_id": "sp_p", "name": "p" }},
            {{ "type": "pcb_port", "pcb_port_id": "pp_p", "source_port_id": "sp_p",
               "x": 0, "y": 0, "layers": ["top"] }},
            {{ "type": "pcb_smtpad", "pcb_smtpad_id": "pad_p", "pcb_port_id": "pp_p",
               "shape": "polygon", "layer": "top", "points": {points} }}"#
        )
    };
    let board = |points: &str| {
        let parts = [
            polygon(points),
            pad("q", 10.0, 0.0, "top"),
            connect("n", "p", "q"),
        ];
        circuit((40.0, 20.0), &parts)
    };
    let chamfered =
        r#"[{"x":-1,"y":-1},{"x":1,"y":-1},{"x":1,"y":1},{"x":0,"y":1},{"x":-1,"y":0}]"#;
    let res = route_circuit(&board(chamfered), &RouteOptions::default()).expect("routes");
    assert_eq!((res.total_nets, res.routed_nets), (1, 1));

    let hooked = r#"[{"x":-2,"y":-2},{"x":2,"y":-2},{"x":2,"y":-1},{"x":-1,"y":-1},{"x":-1,"y":2},{"x":-2,"y":2}]"#;
    let err = route_circuit(&board(hooked), &RouteOptions::default()).unwrap_err();
    assert!(
        format!("{err:?}").contains("outside its outline"),
        "{err:?}"
    );
}
