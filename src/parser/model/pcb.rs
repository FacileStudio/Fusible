//! Copper and drill elements. Units are millimetres.

use serde::Deserialize;

use super::{Kind, Model, Point};

/// One vertex of a `pcb_trace` route, tagged by `route_type`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "route_type", rename_all = "snake_case")]
pub enum RoutePoint {
    /// A copper point on one layer.
    Wire {
        x: f64,
        y: f64,
        width: f64,
        layer: String,
        #[serde(default)]
        start_pcb_port_id: Option<String>,
        #[serde(default)]
        end_pcb_port_id: Option<String>,
    },
    /// A layer change inside the route.
    Via {
        x: f64,
        y: f64,
        from_layer: String,
        to_layer: String,
        #[serde(default)]
        via_diameter: Option<f64>,
        #[serde(default)]
        via_hole_diameter: Option<f64>,
    },
}

/// A routed `pcb_trace`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbTrace {
    pub pcb_trace_id: String,
    pub source_trace_id: Option<String>,
    pub connection_name: Option<String>,
    pub route: Vec<RoutePoint>,
}

/// A `pcb_via`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbVia {
    pub pcb_via_id: String,
    pub pcb_trace_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub outer_diameter: f64,
    pub hole_diameter: f64,
    #[serde(default)]
    pub layers: Vec<String>,
    pub from_layer: Option<String>,
    pub to_layer: Option<String>,
}

/// A `pcb_plated_hole`. Which size fields are set depends on `shape`
/// (`circle`, `oval`, `pill`, `circular_hole_with_rect_pad`, ...).
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbPlatedHole {
    pub pcb_plated_hole_id: String,
    pub pcb_component_id: Option<String>,
    pub pcb_port_id: Option<String>,
    pub shape: Option<String>,
    pub x: f64,
    pub y: f64,
    pub hole_diameter: Option<f64>,
    pub hole_width: Option<f64>,
    pub hole_height: Option<f64>,
    pub hole_offset_x: Option<f64>,
    pub hole_offset_y: Option<f64>,
    pub outer_diameter: Option<f64>,
    pub outer_width: Option<f64>,
    pub outer_height: Option<f64>,
    pub rect_pad_width: Option<f64>,
    pub rect_pad_height: Option<f64>,
    pub rect_ccw_rotation: Option<f64>,
    pub ccw_rotation: Option<f64>,
    #[serde(default)]
    pub layers: Vec<String>,
    #[serde(default)]
    pub port_hints: Vec<String>,
}

/// A non-plated `pcb_hole`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbHole {
    pub pcb_hole_id: String,
    pub pcb_component_id: Option<String>,
    pub hole_shape: Option<String>,
    pub x: f64,
    pub y: f64,
    pub hole_diameter: Option<f64>,
    pub hole_width: Option<f64>,
    pub hole_height: Option<f64>,
}

/// A closed ring of a B-rep copper pour.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct Ring {
    pub vertices: Vec<Point>,
}

/// The outline of a `brep` copper pour: one outer ring minus the inner ones.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct BrepShape {
    pub outer_ring: Ring,
    #[serde(default)]
    pub inner_rings: Vec<Ring>,
}

/// A `pcb_copper_pour`. `shape` is `rect`, `polygon` or `brep`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct PcbCopperPour {
    pub pcb_copper_pour_id: String,
    pub layer: String,
    pub shape: String,
    pub source_net_id: Option<String>,
    pub covered_with_solder_mask: Option<bool>,
    pub brep_shape: Option<BrepShape>,
    pub points: Option<Vec<Point>>,
    pub center: Option<Point>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

impl Kind for PcbTrace {
    const TYPE: &'static str = "pcb_trace";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.traces
    }
}

impl Kind for PcbVia {
    const TYPE: &'static str = "pcb_via";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.vias
    }
}

impl Kind for PcbPlatedHole {
    const TYPE: &'static str = "pcb_plated_hole";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.plated_holes
    }
}

impl Kind for PcbHole {
    const TYPE: &'static str = "pcb_hole";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.holes
    }
}

impl Kind for PcbCopperPour {
    const TYPE: &'static str = "pcb_copper_pour";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.copper_pours
    }
}
