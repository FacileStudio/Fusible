//! Copper and drill geometry read from the raw Circuit JSON.
//!
//! The typed AST models connectivity only. Physical rules (trace current,
//! voltage clearance, fab capability) need widths, layers and drill sizes,
//! which this reader pulls from `Circuit::raw_elements` in one pass.
//! Anything it cannot read lands in [`Geometry::unreadable`]: a rule must
//! report those elements, never skip them.

use serde_json::Value;

use super::Circuit;

mod json;
mod nets;
mod trace;

use json::{num, text};
use nets::NetIndex;
use trace::read_trace;

/// One straight piece of a routed trace. Units are millimetres.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub trace_id: String,
    pub net_names: Vec<String>,
    pub layer: String,
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub width: f64,
}

/// A plated via.
#[derive(Debug, Clone, PartialEq)]
pub struct Via {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub outer: f64,
    pub hole: f64,
}

/// A drilled hole, plated or not. `outer` is the copper diameter when plated.
#[derive(Debug, Clone, PartialEq)]
pub struct Hole {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub hole: f64,
    pub outer: Option<f64>,
}

/// The board stack-up.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub layers: u8,
}

/// Everything physical the rules read.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Geometry {
    pub segments: Vec<Segment>,
    pub vias: Vec<Via>,
    pub holes: Vec<Hole>,
    pub board: Option<Board>,
    pub unreadable: Vec<String>,
}

fn read_via(el: &Value, geometry: &mut Geometry) {
    let id = text(el, &["pcb_via_id"]).unwrap_or("pcb_via").to_string();
    let fields = (
        num(el, "x"),
        num(el, "y"),
        num(el, "outer_diameter"),
        num(el, "hole_diameter"),
    );
    match fields {
        (Some(x), Some(y), Some(outer), Some(hole)) => {
            geometry.vias.push(Via {
                id,
                x,
                y,
                outer,
                hole,
            });
        }
        _ => geometry
            .unreadable
            .push(format!("{id}: via without position or diameters")),
    }
}

fn drill(el: &Value) -> Option<f64> {
    num(el, "hole_diameter").or_else(|| Some(num(el, "hole_width")?.min(num(el, "hole_height")?)))
}

fn copper(el: &Value) -> Option<f64> {
    let pill = num(el, "outer_width").zip(num(el, "outer_height"));
    num(el, "outer_diameter").or_else(|| pill.map(|(w, h)| w.min(h)))
}

fn read_hole(el: &Value, geometry: &mut Geometry) {
    let kind = text(el, &["type"]).unwrap_or("pcb_hole");
    let id = text(el, &["pcb_plated_hole_id", "pcb_hole_id"])
        .unwrap_or(kind)
        .to_string();
    match (num(el, "x"), num(el, "y"), drill(el)) {
        (Some(x), Some(y), Some(hole)) => {
            let outer = if kind == "pcb_plated_hole" {
                copper(el)
            } else {
                None
            };
            geometry.holes.push(Hole {
                id,
                x,
                y,
                hole,
                outer,
            });
        }
        _ => geometry
            .unreadable
            .push(format!("{id}: hole without position or diameter")),
    }
}

fn outline_size(el: &Value) -> Option<(f64, f64)> {
    let points = el.get("outline")?.as_array()?;
    let axis = |key: &str| {
        points
            .iter()
            .map(|p| num(p, key))
            .collect::<Option<Vec<f64>>>()
    };
    let span = |values: Vec<f64>| {
        let low = values.iter().copied().fold(f64::MAX, f64::min);
        let high = values.iter().copied().fold(f64::MIN, f64::max);
        (!values.is_empty()).then_some(high - low)
    };
    span(axis("x")?).zip(span(axis("y")?))
}

fn read_board(el: &Value, geometry: &mut Geometry) {
    let size = num(el, "width")
        .zip(num(el, "height"))
        .or_else(|| outline_size(el));
    let Some((width, height)) = size else {
        geometry
            .unreadable
            .push("pcb_board: no width and height or outline".into());
        return;
    };
    let layers = el.get("num_layers").and_then(Value::as_u64).unwrap_or(2);
    geometry.board = Some(Board {
        width,
        height,
        thickness: num(el, "thickness").unwrap_or(1.6),
        layers: u8::try_from(layers).unwrap_or(2),
    });
}

/// Reads every trace segment, via, hole and the board outline of `circuit`.
///
/// A trace's nets come from `source_trace.connected_source_net_ids`, then from the
/// nets of its `connected_source_port_ids`. Elements missing a required field are
/// listed in `unreadable` instead of being dropped.
pub fn read(circuit: &Circuit) -> Geometry {
    let nets = NetIndex::new(&circuit.raw_elements);
    let mut geometry = Geometry::default();
    for el in &circuit.raw_elements {
        match text(el, &["type"]) {
            Some("pcb_trace") => read_trace(el, &nets, &mut geometry),
            Some("pcb_via") => read_via(el, &mut geometry),
            Some("pcb_plated_hole" | "pcb_hole") => read_hole(el, &mut geometry),
            Some("pcb_board") => read_board(el, &mut geometry),
            _ => {}
        }
    }
    geometry
}
