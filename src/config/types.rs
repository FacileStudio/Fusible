//! Named manifest sections: boards, firmware, and bodies.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Deserialize;

/// A circuit board: its tscircuit entry, output directory, dev port, and target silicon.
#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoardConfig {
    #[serde(default)]
    pub entry: Option<PathBuf>,
    #[serde(default, alias = "out-dir")]
    pub out_dir: Option<PathBuf>,
    #[serde(default, alias = "circuit-json")]
    pub circuit_json: Option<PathBuf>,
    #[serde(default)]
    pub port: Option<u16>,
    /// Target silicon identifier, e.g. `stm32g431` or `esp32-s3`.
    #[serde(default)]
    pub mcu: Option<String>,
    /// The `.kicad_pcb` file KiCad edits for this board; the studio syncs its placements.
    #[serde(default)]
    pub kicad: Option<PathBuf>,
}

/// A firmware crate bound to one board, with its generated pin bindings and build command.
#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FirmwareConfig {
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub framework: Option<String>,
    #[serde(default, alias = "pins-rs")]
    pub pins_rs: Option<PathBuf>,
    #[serde(default, alias = "build-cmd")]
    pub build_cmd: Option<String>,
    /// Board the firmware runs on; optional when the project has a single board.
    #[serde(default)]
    pub board: Option<String>,
}

/// A physical body: either a file authored in any CAD tool or a model Fusible generates.
#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
pub struct BodyConfig {
    /// Authored model file (STEP, STL, ...). Fusible reads it and never overwrites it.
    #[serde(default)]
    pub source: Option<PathBuf>,
    /// Command that produces `source`, for code-CAD tools such as build123d or OpenSCAD.
    #[serde(default)]
    pub build: Option<String>,
    /// Generator that owns this body, regenerated on every run: `enclosure`.
    #[serde(default)]
    pub generate: Option<String>,
    /// Board the body belongs to. Required for a generated body when the project has several
    /// boards. An authored body is modeled in this board's coordinate frame.
    #[serde(default, alias = "around")]
    pub board: Option<String>,
    /// Key in the material library (`pla`, `petg`, `al6061`, ...); sets the mass and the cost.
    #[serde(default)]
    pub material: Option<String>,
    /// Manufacturing process: `fdm`, `sla`, `mjf`, `injection` or `cnc`.
    #[serde(default)]
    pub process: Option<String>,
    #[serde(flatten)]
    pub enclosure: EnclosureConfig,
}

impl BodyConfig {
    /// A code part: its `source` is a TypeScript module that builds the shape.
    pub fn is_code(&self) -> bool {
        self.source
            .as_ref()
            .is_some_and(|s| s.extension().is_some_and(|e| e.eq_ignore_ascii_case("ts")))
    }
}

/// Settings of the `enclosure` generator (base + lid around a board outline).
#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
pub struct EnclosureConfig {
    /// Export format: `step`, `stl`, `glb`, `brep`, or `all`.
    #[serde(default)]
    pub format: Option<String>,
    /// Directory receiving the generated models.
    #[serde(default, alias = "out-dir")]
    pub out_dir: Option<PathBuf>,
    /// Wall, floor, and lid thickness in millimetres.
    #[serde(default, alias = "wall-thickness")]
    pub wall_thickness: Option<f64>,
    /// Total external height of the base in millimetres.
    #[serde(default)]
    pub height: Option<f64>,
    /// Cut wall notches for components that reach the board edge.
    #[serde(default)]
    pub cutouts: Option<bool>,
    /// Lid ventilation pattern: `none`, `grid`, or `slots`.
    #[serde(default)]
    pub ventilation: Option<String>,
    /// Add screw bosses under every `pcb_hole` of the board.
    #[serde(default)]
    pub mounting: Option<bool>,
}

/// Where one board or body sits in the assembly: a translation in mm and rotations in degrees.
///
/// Rotations turn around the fixed X, then Y, then Z axes, and apply before the translation.
#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PlacementConfig {
    #[serde(default)]
    pub at: Option<[f64; 3]>,
    #[serde(default)]
    pub rotate: Option<[f64; 3]>,
}

/// Boards and bodies placed in one coordinate frame and exported as one model.
#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssemblyConfig {
    /// Directory receiving `assembly.<format>`.
    #[serde(default, alias = "out-dir")]
    pub out_dir: Option<PathBuf>,
    /// Export format: `glb`, `step`, or `all`.
    #[serde(default)]
    pub format: Option<String>,
    /// Board and body names, each with its placement.
    #[serde(default)]
    pub parts: BTreeMap<String, PlacementConfig>,
    /// Holes narrower than this (mm) need no boss; default 1.0.
    #[serde(default, alias = "hole-min-diameter")]
    pub hole_min_diameter: Option<f64>,
}

/// Narrowest hole that needs a boss, in mm; narrower holes are acoustic ports or vias.
pub const DEFAULT_HOLE_MIN: f64 = 1.0;

impl AssemblyConfig {
    /// The narrowest hole that needs a boss, in mm.
    pub fn hole_min(&self) -> f64 {
        self.hole_min_diameter.unwrap_or(DEFAULT_HOLE_MIN)
    }
}

/// A pin of a connector, written as a number (`1`) or a name (`GND`).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PinId {
    Number(u32),
    Name(String),
}

impl std::fmt::Display for PinId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Name(name) => f.write_str(name),
        }
    }
}

/// One wire of a link: pin `a` of the `from` connector goes to pin `b` of the `to` connector.
///
/// `net` names the net both ends must carry. `nets` names the net of each end, for a crossed
/// wire such as `[TX, RX]`. Without either, both ends must carry the same net name.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WireConfig {
    pub a: PinId,
    pub b: PinId,
    #[serde(default)]
    pub net: Option<String>,
    #[serde(default)]
    pub nets: Option<[String; 2]>,
}

/// Board-to-board wiring: a connector of one board cabled to a connector of another.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LinkConfig {
    /// `board.REFDES` of the first connector.
    pub from: String,
    /// `board.REFDES` of the second connector.
    pub to: String,
    #[serde(default)]
    pub wires: Vec<WireConfig>,
}
