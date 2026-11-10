//! Silkscreen and courtyard elements. Units are millimetres.

use serde::Deserialize;

use super::{Kind, Model, Point};

/// A `pcb_silkscreen_text`. `font_size` is the text height.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct SilkscreenText {
    pub pcb_silkscreen_text_id: String,
    pub pcb_component_id: Option<String>,
    pub text: String,
    pub layer: String,
    pub font_size: f64,
    pub anchor_position: Point,
    pub anchor_alignment: Option<String>,
    pub ccw_rotation: Option<f64>,
    pub font: Option<String>,
}

/// A `pcb_silkscreen_path`: a polyline drawn `stroke_width` wide.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct SilkscreenPath {
    pub pcb_silkscreen_path_id: String,
    pub pcb_component_id: Option<String>,
    pub layer: String,
    pub route: Vec<Point>,
    pub stroke_width: f64,
}

/// A `pcb_courtyard_rect`. tscircuit does not apply the part rotation to
/// it; `ccw_rotation` is only set when the footprint itself is turned.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct CourtyardRect {
    pub pcb_courtyard_rect_id: String,
    pub pcb_component_id: String,
    pub layer: String,
    pub center: Point,
    pub width: f64,
    pub height: f64,
    pub ccw_rotation: Option<f64>,
}

/// A `pcb_courtyard_outline`: a closed polygon around a part.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct CourtyardOutline {
    pub pcb_courtyard_outline_id: String,
    pub pcb_component_id: String,
    pub layer: String,
    pub outline: Vec<Point>,
}

impl Kind for SilkscreenText {
    const TYPE: &'static str = "pcb_silkscreen_text";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.silkscreen_texts
    }
}

impl Kind for SilkscreenPath {
    const TYPE: &'static str = "pcb_silkscreen_path";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.silkscreen_paths
    }
}

impl Kind for CourtyardRect {
    const TYPE: &'static str = "pcb_courtyard_rect";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.courtyard_rects
    }
}

impl Kind for CourtyardOutline {
    const TYPE: &'static str = "pcb_courtyard_outline";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.courtyard_outlines
    }
}
