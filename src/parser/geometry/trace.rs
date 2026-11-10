//! Trace routes split into straight segments.

use super::nets::NetIndex;
use super::{Geometry, Segment};
use crate::parser::model::{PcbTrace, RoutePoint};

struct Point {
    x: f64,
    y: f64,
    width: Option<f64>,
    layer: String,
}

fn point(route: &RoutePoint) -> Result<Point, String> {
    let (x, y, width, layer) = match route {
        RoutePoint::Wire {
            x, y, width, layer, ..
        } => {
            if *width <= 0.0 {
                return Err("wire point without a positive width".into());
            }
            (*x, *y, Some(*width), layer)
        }
        RoutePoint::Via { x, y, to_layer, .. } => (*x, *y, None, to_layer),
    };
    Ok(Point {
        x,
        y,
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

pub(super) fn read_trace(trace: &PcbTrace, nets: &NetIndex, geometry: &mut Geometry) {
    let id = trace.pcb_trace_id.as_str();
    let names = trace
        .source_trace_id
        .as_deref()
        .map(|s| nets.names(s))
        .unwrap_or_default();
    let built = trace
        .route
        .iter()
        .map(point)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|points| {
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
