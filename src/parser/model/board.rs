//! The board, the placed parts and the pins they join.

use serde::Deserialize;

use super::{Kind, Model, Point};

/// A `pcb_board`. Units are millimetres.
///
/// tscircuit writes `width` and `height` for a rectangle, or `outline` for
/// any other shape. The `min_*` fields are the board's own DRC limits.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbBoard {
    pub pcb_board_id: Option<String>,
    pub source_board_id: Option<String>,
    pub center: Option<Point>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub outline: Option<Vec<Point>>,
    pub thickness: Option<f64>,
    pub num_layers: Option<u32>,
    pub material: Option<String>,
    pub min_trace_width: Option<f64>,
    pub min_via_hole_diameter: Option<f64>,
    pub min_via_pad_diameter: Option<f64>,
    pub min_via_hole_edge_to_via_hole_edge_clearance: Option<f64>,
    pub min_trace_to_hole_edge_clearance: Option<f64>,
    pub min_trace_to_pad_edge_clearance: Option<f64>,
    pub min_pad_edge_to_pad_edge_clearance: Option<f64>,
    pub min_plated_hole_drill_edge_to_drill_edge_clearance: Option<f64>,
    pub min_board_edge_clearance: Option<f64>,
}

/// A `pcb_component`: where a part sits. `rotation` is in degrees.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbComponent {
    pub pcb_component_id: String,
    pub source_component_id: String,
    pub center: Point,
    pub width: f64,
    pub height: f64,
    pub layer: String,
    #[serde(default)]
    pub rotation: f64,
    pub do_not_place: Option<bool>,
    pub obstructs_within_bounds: Option<bool>,
    pub is_allowed_to_be_off_board: Option<bool>,
}

/// A `source_component_internal_connection`: pins a part joins inside itself.
///
/// Only the port list matters to the netlist, so the ids are optional.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct InternalConnection {
    pub source_component_internal_connection_id: Option<String>,
    pub source_component_id: Option<String>,
    #[serde(default)]
    pub source_port_ids: Vec<String>,
}

impl Kind for PcbBoard {
    const TYPE: &'static str = "pcb_board";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.boards
    }
}

impl Kind for PcbComponent {
    const TYPE: &'static str = "pcb_component";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.components
    }
}

impl Kind for InternalConnection {
    const TYPE: &'static str = "source_component_internal_connection";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.internal_connections
    }
}
