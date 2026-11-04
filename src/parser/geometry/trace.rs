//! Trace routes split into straight segments.

use serde_json::Value;

use super::json::{num, text};
use super::nets::NetIndex;
use super::{Geometry, Segment};

struct Point {
    x: f64,
    y: f64,
    width: Option<f64>,
    layer: String,
}

fn point(el: &Value) -> Result<Point, String> {
    let via = text(el, &["route_type"]) == Some("via");
    let layer_key = if via { "to_layer" } else { "layer" };
    let layer = text(el, &[layer_key]).ok_or(format!("route point without {layer_key}"))?;
    let (x, y) = (num(el, "x"), num(el, "y"));
    let width = num(el, "width").filter(|w| *w > 0.0);
    if !via && width.is_none() {
        return Err("wire point without a positive width".into());
    }
    let at = x.zip(y).ok_or("route point without x and y")?;
    Ok(Point {
        x: at.0,
        y: at.1,
        width,
        layer: layer.to_ascii_lowercase(),
    })
}

fn segment(id: &str, names: &[String], p: &Point, q: &Point) -> Result<Segment, String> {
    let width = p
        .width
        .or(q.width)
        .ok_or("segment between two vias has no width")?;
    Ok(Segment {
        trace_id: id.to_string(),
        net_names: names.to_vec(),
        layer: p.layer.clone(),
        a: (p.x, p.y),
        b: (q.x, q.y),
        width,
    })
}

pub(super) fn read_trace(el: &Value, nets: &NetIndex, geometry: &mut Geometry) {
    let id = text(el, &["pcb_trace_id"]).unwrap_or("pcb_trace");
    let names = text(el, &["source_trace_id"]).map(|s| nets.names(s));
    let route = el.get("route").and_then(Value::as_array);
    let built = route
        .ok_or("trace without a route".to_string())
        .and_then(|route| route.iter().map(point).collect::<Result<Vec<_>, _>>())
        .and_then(|points| {
            let names = names.unwrap_or_default();
            let pairs = points.windows(2);
            pairs
                .map(|w| segment(id, &names, &w[0], &w[1]))
                .collect::<Result<Vec<_>, _>>()
        });
    match built {
        Ok(segments) => geometry.segments.extend(segments),
        Err(why) => geometry.unreadable.push(format!("{id}: {why}")),
    }
}
