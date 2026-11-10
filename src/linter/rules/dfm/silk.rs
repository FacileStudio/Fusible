//! Silkscreen strokes and text boxes, and the pad copper they must avoid.
//!
//! Text boxes follow the built-in vector font of the Gerber writer: glyphs
//! are 4 by 6 units on a 5 unit advance, the text height is 6 units.

use super::owners::Owners;
use crate::linter::rules::copper::Shape;
use crate::linter::rules::pad_geometry::{hole_shape, pad_shape};
use crate::linter::LintContext;
use crate::parser::model::{Model, SilkscreenText};

const GLYPH_WIDTH: f64 = 4.0;
const GLYPH_HEIGHT: f64 = 6.0;
const ADVANCE: f64 = 5.0;

/// One piece of silkscreen, or of pad copper, on one side.
#[derive(Debug, Clone)]
pub(super) struct Item {
    pub id: String,
    pub layer: String,
    pub shape: Shape,
    /// Designator of the part, for pads.
    pub owner: Option<String>,
}

fn align(alignment: Option<&str>, w: f64, h: f64) -> (f64, f64) {
    let (vertical, horizontal) = match alignment.unwrap_or("center") {
        "center" => ("center", "center"),
        other => other.split_once('_').unwrap_or(("center", "center")),
    };
    let x = match horizontal {
        "left" => 0.0,
        "right" => w,
        _ => w / 2.0,
    };
    let y = match vertical {
        "bottom" => 0.0,
        "top" => h,
        _ => h / 2.0,
    };
    (x, y)
}

/// The axis-aligned box around a text as the Gerber writer strokes it.
pub(super) fn text_box(text: &SilkscreenText) -> Shape {
    let unit = text.font_size / GLYPH_HEIGHT;
    let count = text.text.chars().count() as f64;
    let (w, h) = (
        (((count - 1.0) * ADVANCE + GLYPH_WIDTH).max(0.0)) * unit,
        text.font_size,
    );
    let (ox, oy) = align(text.anchor_alignment.as_deref(), w, h);
    let mirror = if text.layer == "bottom" { -1.0 } else { 1.0 };
    let local = (mirror * (w / 2.0 - ox), h / 2.0 - oy);
    let turn = text.ccw_rotation.unwrap_or(0.0).to_radians();
    let (sin, cos) = turn.sin_cos();
    let anchor = text.anchor_position;
    let c = (
        anchor.x + local.0 * cos - local.1 * sin,
        anchor.y + local.0 * sin + local.1 * cos,
    );
    let hw = (w / 2.0 * cos).abs() + (h / 2.0 * sin).abs();
    let hh = (w / 2.0 * sin).abs() + (h / 2.0 * cos).abs();
    Shape::Rect { c, hw, hh }
}

/// Every drawn silkscreen stroke and text box. Paths with a zero stroke are
/// not plotted, so they are left out.
pub(super) fn silk(model: &Model) -> Vec<Item> {
    let paths = model
        .silkscreen_paths
        .iter()
        .filter(|p| p.stroke_width > 0.0);
    let strokes = paths.flat_map(|p| {
        p.route.windows(2).map(|w| Item {
            id: p.pcb_silkscreen_path_id.clone(),
            layer: p.layer.clone(),
            shape: Shape::Seg {
                a: (w[0].x, w[0].y),
                b: (w[1].x, w[1].y),
                half: p.stroke_width / 2.0,
            },
            owner: None,
        })
    });
    let texts = model.silkscreen_texts.iter().map(|t| Item {
        id: t.pcb_silkscreen_text_id.clone(),
        layer: t.layer.clone(),
        shape: text_box(t),
        owner: None,
    });
    strokes.chain(texts).collect()
}

/// Every pad's copper on its side; plated-hole pads sit on both sides.
pub(super) fn pads(ctx: &LintContext<'_>) -> Vec<Item> {
    let owners = Owners::new(ctx);
    let smt = ctx.circuit.pads().enumerate().filter_map(|(i, pad)| {
        Some(Item {
            shape: pad_shape(pad)?,
            id: pad.pad_id.clone().unwrap_or_else(|| format!("#{i}")),
            layer: pad.layer.as_deref().unwrap_or("top").to_ascii_lowercase(),
            owner: owners.of_pad(pad).map(str::to_string),
        })
    });
    let plated = ctx.circuit.model().plated_holes.iter().filter_map(|el| {
        let shape = hole_shape(el)?;
        let owner = el
            .pcb_component_id
            .as_deref()
            .and_then(|c| owners.of_component(c));
        let side = |layer: &str| Item {
            id: el.pcb_plated_hole_id.clone(),
            layer: layer.to_string(),
            shape: shape.clone(),
            owner: owner.map(str::to_string),
        };
        Some([side("top"), side("bottom")])
    });
    smt.chain(plated.flatten()).collect()
}
