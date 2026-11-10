//! Copper and drill geometry: a view over the typed [`Model`].
//!
//! The typed AST models connectivity only. Physical rules (trace current,
//! voltage clearance, fab capability) need widths, layers and drill sizes,
//! which this view flattens out of [`Circuit::model`]: routes become straight
//! segments tagged with their nets, holes and vias get one drill diameter.
//! Anything it cannot read lands in [`Geometry::unreadable`]: a rule must
//! report those elements, never skip them.

use super::model::{Model, PcbBoard, PcbHole, PcbPlatedHole, PcbVia};
use super::Circuit;

mod nets;
mod trace;

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

fn via(el: &PcbVia) -> Via {
    Via {
        id: el.pcb_via_id.clone(),
        x: el.x,
        y: el.y,
        outer: el.outer_diameter,
        hole: el.hole_diameter,
    }
}

fn smaller(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    Some(a?.min(b?))
}

fn plated(el: &PcbPlatedHole) -> Result<Hole, String> {
    let id = el.pcb_plated_hole_id.clone();
    let hole = el
        .hole_diameter
        .or_else(|| smaller(el.hole_width, el.hole_height));
    let outer = el
        .outer_diameter
        .or_else(|| smaller(el.outer_width, el.outer_height));
    match hole {
        Some(hole) => Ok(Hole {
            id,
            x: el.x,
            y: el.y,
            hole,
            outer,
        }),
        None => Err(format!("{id}: hole without position or diameter")),
    }
}

fn unplated(el: &PcbHole) -> Result<Hole, String> {
    let id = el.pcb_hole_id.clone();
    let hole = el
        .hole_diameter
        .or_else(|| smaller(el.hole_width, el.hole_height));
    match hole {
        Some(hole) => Ok(Hole {
            id,
            x: el.x,
            y: el.y,
            hole,
            outer: None,
        }),
        None => Err(format!("{id}: hole without position or diameter")),
    }
}

fn outline_size(board: &PcbBoard) -> Option<(f64, f64)> {
    let points = board.outline.as_deref().filter(|p| !p.is_empty())?;
    let span = |axis: fn(&super::PadPoint) -> f64| {
        let low = points.iter().map(axis).fold(f64::MAX, f64::min);
        let high = points.iter().map(axis).fold(f64::MIN, f64::max);
        high - low
    };
    Some((span(|p| p.x), span(|p| p.y)))
}

fn board(el: &PcbBoard) -> Result<Board, String> {
    let size = el.width.zip(el.height).or_else(|| outline_size(el));
    let (width, height) = size.ok_or("pcb_board: no width and height or outline")?;
    Ok(Board {
        width,
        height,
        thickness: el.thickness.unwrap_or(1.6),
        layers: u8::try_from(el.num_layers.unwrap_or(2)).unwrap_or(2),
    })
}

/// The model's `<id>: <type>: <why>` entries for the physical types, worded
/// as this view words its own failures.
fn unreadable(model: &Model) -> impl Iterator<Item = String> + '_ {
    model.unreadable.iter().filter_map(|entry| {
        let (id, rest) = entry.split_once(": ")?;
        let (kind, why) = rest.split_once(": ")?;
        let what = match kind {
            "pcb_trace" => return Some(format!("{id}: {why}")),
            "pcb_via" => "via without position or diameters",
            "pcb_plated_hole" | "pcb_hole" => "hole without position or diameter",
            "pcb_board" => "no width and height or outline",
            _ => return None,
        };
        Some(format!("{id}: {what} ({why})"))
    })
}

/// Reads every trace segment, via, hole and the board outline of `circuit`.
///
/// A trace's nets come from `source_trace.connected_source_net_ids`, then from the
/// nets of its `connected_source_port_ids`. Elements missing a required field are
/// listed in `unreadable` instead of being dropped.
pub fn read(circuit: &Circuit) -> Geometry {
    let model = circuit.model();
    let nets = NetIndex::new(circuit);
    let mut geometry = Geometry::default();
    for trace in &model.traces {
        read_trace(trace, &nets, &mut geometry);
    }
    geometry.vias = model.vias.iter().map(via).collect();
    let holes = model.plated_holes.iter().map(plated);
    for hole in holes.chain(model.holes.iter().map(unplated)) {
        match hole {
            Ok(hole) => geometry.holes.push(hole),
            Err(why) => geometry.unreadable.push(why),
        }
    }
    for el in &model.boards {
        match board(el) {
            Ok(board) => geometry.board = Some(board),
            Err(why) => geometry.unreadable.push(why),
        }
    }
    geometry.unreadable.extend(unreadable(model));
    geometry
}
