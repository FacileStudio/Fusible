//! Where a pad sits, how big it is, and which net it is on.

use super::copper::Shape;
use crate::linter::LintContext;
use crate::parser::model::PcbPlatedHole;
use crate::parser::{Pad, PadPoint};

/// The copper outline of a pad: a circle for a `radius` pad, else its box.
pub(super) fn pad_shape(pad: &Pad) -> Option<Shape> {
    if let (Some(x), Some(y), None, None, Some(r)) =
        (pad.x, pad.y, pad.width, pad.height, pad.radius)
    {
        return Some(Shape::disc((x, y), r));
    }
    let (x, y, hw, hh) = pad_box(pad)?;
    Some(Shape::Rect { c: (x, y), hw, hh })
}

/// The copper around a plated hole: a circle for a round ring, a box for a
/// pill ring or a rectangular pad (turned by quarter turns), and the barrel
/// alone when no pad size is written.
pub(super) fn hole_shape(hole: &PcbPlatedHole) -> Option<Shape> {
    let c = (hole.x, hole.y);
    if let Some(d) = hole.outer_diameter {
        return Some(Shape::disc(c, d / 2.0));
    }
    let pill = hole.outer_width.zip(hole.outer_height);
    let Some((w, h)) = pill.or_else(|| hole.rect_pad_width.zip(hole.rect_pad_height)) else {
        return Some(Shape::disc(c, hole.hole_diameter? / 2.0));
    };
    let quarter = |r: f64| (r / 90.0).round() as i64 % 2 != 0;
    let (w, h) = match hole.rect_ccw_rotation.is_some_and(quarter) {
        true => (h, w),
        false => (w, h),
    };
    Some(Shape::Rect {
        c,
        hw: w / 2.0,
        hh: h / 2.0,
    })
}

/// Half extents of a pad: `width`/`height`, or `radius` for circle pads.
fn half_extents(pad: &Pad) -> (f64, f64) {
    match (pad.width, pad.height, pad.radius) {
        (None, None, Some(r)) => (r, r),
        (w, h, _) => (w.unwrap_or(1.0) / 2.0, h.unwrap_or(1.0) / 2.0),
    }
}

/// Center and half extents of a pad: from `x`/`y` and its size, or from the
/// bounding box of its polygon outline, which covers all of its copper.
pub(super) fn pad_box(pad: &Pad) -> Option<(f64, f64, f64, f64)> {
    if let (Some(x), Some(y)) = (pad.x, pad.y) {
        let (half_w, half_h) = half_extents(pad);
        return Some((x, y, half_w, half_h));
    }
    let points = pad.points.as_deref().filter(|p| !p.is_empty())?;
    let fold = |f: fn(f64, f64) -> f64, init: f64, axis: fn(&PadPoint) -> f64| {
        points.iter().map(axis).fold(init, f)
    };
    let (min_x, max_x) = (
        fold(f64::min, f64::MAX, |p| p.x),
        fold(f64::max, f64::MIN, |p| p.x),
    );
    let (min_y, max_y) = (
        fold(f64::min, f64::MAX, |p| p.y),
        fold(f64::max, f64::MIN, |p| p.y),
    );
    Some((
        (min_x + max_x) / 2.0,
        (min_y + max_y) / 2.0,
        (max_x - min_x) / 2.0,
        (max_y - min_y) / 2.0,
    ))
}

/// The net id and display name of a pad. A pad whose net the netlist cannot
/// resolve is isolated copper: it gets a net of its own, so any overlap with
/// another pad is still a violation.
pub(super) fn pad_net(ctx: &LintContext<'_>, pad: &Pad, label: &str) -> (String, String) {
    let node = pad
        .pad_id
        .as_deref()
        .and_then(|id| ctx.netlist.pad_to_net.get(id))
        .and_then(|&idx| ctx.netlist.nets.get(idx));
    match node {
        Some(node) => (node.id.clone(), node.display_name().to_string()),
        None => (
            format!("unresolved pad {label}"),
            format!("<unresolved pad {label}>"),
        ),
    }
}
