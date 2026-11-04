//! The reference kit data: board outline, holes, component footprints, and edge openings.

use serde::Serialize;

use super::kit_levels::{keep_out, warnings, KitKeepOut};
use super::kit_scan::{component_names, entry, scan_of};
use super::spec::open_ring;
use super::{BodyError, CLEARANCE, DEFAULT_BOARD_THICKNESS, EDGE_TOLERANCE, NOTCH_MARGIN};
use crate::gerber::PcbModel;

#[derive(Debug, Serialize)]
pub(crate) struct KitHole {
    pub x: f64,
    pub y: f64,
    pub diameter: f64,
    pub plated: bool,
}

/// A component footprint: `x`, `y` is its center, `w` and `h` its size.
#[derive(Debug, Serialize)]
pub(crate) struct KitComponent {
    pub name: String,
    pub layer: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub touches_edge: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

/// A wall opening: the part of a board edge a component reaches, as the generator cuts it.
///
/// `from` and `to` are points on the board edge. The cut runs outward along `normal` for
/// `notch.clearance + wall + notch.extra_depth` millimetres.
#[derive(Debug, Serialize)]
pub(crate) struct KitOpening {
    pub component: String,
    pub edge: usize,
    pub side: &'static str,
    pub normal: [f64; 2],
    pub from: [f64; 2],
    pub to: [f64; 2],
}

/// The constants behind the openings, so a hand-made cut can match the generated one.
#[derive(Debug, Serialize)]
pub(crate) struct KitNotch {
    pub margin: f64,
    pub edge_tolerance: f64,
    pub clearance: f64,
    pub extra_depth: f64,
}

impl KitNotch {
    fn of_the_generator() -> Self {
        Self {
            margin: NOTCH_MARGIN,
            edge_tolerance: EDGE_TOLERANCE,
            clearance: CLEARANCE,
            extra_depth: 1.0,
        }
    }
}

/// Everything a designer needs to model a part around the board, in the generator's frame.
#[derive(Debug, Serialize)]
pub(crate) struct Kit {
    pub board: Option<String>,
    pub units: &'static str,
    pub frame: &'static str,
    pub outline: Vec<[f64; 2]>,
    pub thickness: f64,
    pub holes: Vec<KitHole>,
    pub components: Vec<KitComponent>,
    pub keep_out: Vec<KitKeepOut>,
    pub openings: Vec<KitOpening>,
    pub notch: KitNotch,
    pub warnings: Vec<String>,
}

/// The holes with a usable diameter, and how many had none and were left out.
fn holes(model: &PcbModel) -> (Vec<KitHole>, usize) {
    let plain = model
        .holes
        .iter()
        .map(|h| (h.x, h.y, h.hole_diameter.or(h.hole_width), false));
    let plated = model
        .plated_holes
        .iter()
        .map(|h| (h.x, h.y, h.hole_diameter.or(h.hole_width), true));
    let all: Vec<_> = plain.chain(plated).collect();
    let holes: Vec<KitHole> = all
        .iter()
        .filter_map(|&(x, y, d, plated)| {
            let diameter = d.filter(|d| d.is_finite() && *d > 0.0)?;
            Some(KitHole {
                x,
                y,
                diameter,
                plated,
            })
        })
        .collect();
    let skipped = all.len() - holes.len();
    (holes, skipped)
}

/// Builds the kit of the board in `json`; `board` is the manifest name written to the file.
pub(crate) fn build_kit(json: &str, board: Option<String>) -> Result<Kit, BodyError> {
    let names = component_names(json)?;
    let model = PcbModel::parse(json);
    let elm = model.boards.first().ok_or(BodyError::NoBoard)?;
    let outline = open_ring(elm.outline_points());
    if outline.len() < 3 {
        return Err(BodyError::NoBoard);
    }
    let (holes, skipped) = holes(&model);
    let thickness = elm.thickness.unwrap_or(DEFAULT_BOARD_THICKNESS);
    let scan = scan_of(json, names, &outline, thickness)?;
    let entries = model.components.iter().filter_map(|c| entry(c, &scan));
    let (components, openings): (Vec<_>, Vec<_>) = entries.unzip();
    let keep_out = keep_out(&components);
    let unmeasured = components.len() - keep_out.len();
    Ok(Kit {
        board,
        units: "mm",
        frame: "pcb coordinates, Z-up; the enclosure generator uses the same X/Y",
        outline: outline.iter().map(|p| [p.x, p.y]).collect(),
        thickness,
        holes,
        components,
        keep_out,
        openings: openings.into_iter().flatten().collect(),
        notch: KitNotch::of_the_generator(),
        warnings: warnings(skipped, unmeasured),
    })
}
