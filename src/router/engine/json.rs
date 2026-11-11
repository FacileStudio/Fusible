//! Field readers for raw Circuit JSON elements.

use serde_json::Value;

/// Half width and half height of a pad's axis-aligned bounding box,
/// accounting for `ccw_rotation`.
pub(super) fn pad_half_extent(el: &Value) -> Option<[f64; 2]> {
    let (w, h) = match (num(el, "width"), num(el, "height")) {
        (Some(w), Some(h)) => (w, h),
        _ => {
            let d = num(el, "outer_diameter")
                .or_else(|| num(el, "radius").map(|r| r * 2.0))
                .map(|d| (d, d));
            let outer = num(el, "outer_width").zip(num(el, "outer_height"));
            let rect = num(el, "rect_pad_width").zip(num(el, "rect_pad_height"));
            d.or(outer).or(rect)?
        }
    };
    let rot = ["ccw_rotation", "rect_ccw_rotation"]
        .iter()
        .find_map(|k| num(el, k))
        .unwrap_or(0.0)
        .to_radians();
    let (s, c) = (rot.sin().abs(), rot.cos().abs());
    Some([(w * c + h * s) / 2.0, (w * s + h * c) / 2.0])
}

/// Center and half extents of a pad's bounding box. A polygon pad carries
/// `points` instead of a center: its box is the box of its outline, and the
/// box center must lie inside the outline to serve as the routing anchor.
pub(super) fn pad_box(el: &Value) -> Result<([f64; 2], [f64; 2]), &'static str> {
    if let Some((x, y)) = num(el, "x").zip(num(el, "y")) {
        return Ok(([x, y], pad_half_extent(el).ok_or("missing pad size")?));
    }
    let points: Vec<[f64; 2]> = el
        .get("points")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|p| Some([num(p, "x")?, num(p, "y")?]))
        .collect();
    if points.len() < 3 {
        return Err("missing x/y");
    }
    let span = |axis: usize| {
        let low = points.iter().map(|p| p[axis]).fold(f64::MAX, f64::min);
        let high = points.iter().map(|p| p[axis]).fold(f64::MIN, f64::max);
        ((low + high) / 2.0, (high - low) / 2.0)
    };
    let ((cx, hw), (cy, hh)) = (span(0), span(1));
    if !polygon_contains(&points, [cx, cy]) {
        return Err("polygon pad whose center lies outside its outline");
    }
    Ok(([cx, cy], [hw, hh]))
}

fn polygon_contains(points: &[[f64; 2]], [x, y]: [f64; 2]) -> bool {
    let edges = points.iter().zip(points.iter().cycle().skip(1));
    let crossings = edges
        .filter(|(a, b)| (a[1] > y) != (b[1] > y))
        .filter(|(a, b)| x < a[0] + (y - a[1]) * (b[0] - a[0]) / (b[1] - a[1]))
        .count();
    crossings % 2 == 1
}

pub(super) fn type_of(el: &Value) -> &str {
    str_of(el, "type").unwrap_or("")
}

pub(super) fn str_of<'a>(el: &'a Value, key: &str) -> Option<&'a str> {
    el.get(key).and_then(Value::as_str)
}

pub(super) fn num(el: &Value, key: &str) -> Option<f64> {
    el.get(key)
        .and_then(Value::as_f64)
        .filter(|v| v.is_finite())
}
