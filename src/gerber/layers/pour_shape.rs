//! Pour contours in millimetres: shape decoding and the outside-of-board clip region.

use super::elements::{PcbCopperPourElm, PcbPoint, PcbRing};
use super::math::{add, finite_or, required, rotate};

type Contour = Vec<[f64; 2]>;
pub(super) type PourRings = (Contour, Vec<Contour>);

const OUTSIDE_MARGIN_MM: f64 = 1.0;

/// Returns a region covering everything outside `ring`: a bounding frame joined to the ring by a cut-in.
pub(super) fn outside_region(ring: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let span = |axis: usize| {
        let lo = ring.iter().map(|p| p[axis]).fold(f64::MAX, f64::min);
        let hi = ring.iter().map(|p| p[axis]).fold(f64::MIN, f64::max);
        (lo - OUTSIDE_MARGIN_MM, hi + OUTSIDE_MARGIN_MM)
    };
    let ((x0, x1), (y0, y1)) = (span(0), span(1));
    let mut region = vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1], [x0, y0], ring[0]];
    region.extend_from_slice(&ring[1..]);
    region.extend([ring[0], [x0, y0]]);
    region
}

/// Returns the outer contour and the hole contours of a pour, in millimetres.
pub(super) fn pour_rings(pour: &PcbCopperPourElm) -> Result<PourRings, String> {
    let (outer, holes) = match pour.shape.as_deref() {
        Some("rect") => (rect_ring(pour)?, Vec::new()),
        Some("polygon") => (points(pour.points.as_deref().unwrap_or(&[])), Vec::new()),
        Some("brep") => {
            let brep = pour
                .brep_shape
                .as_ref()
                .ok_or("brep pour without brep_shape")?;
            (
                ring(&brep.outer_ring),
                brep.inner_rings.iter().map(ring).collect(),
            )
        }
        Some(other) => return Err(format!("unsupported pour shape '{other}'")),
        None => return Err("missing shape".to_string()),
    };
    let finite = |r: &Vec<[f64; 2]>| r.iter().flatten().all(|v| v.is_finite());
    if outer.len() < 3 || !finite(&outer) || !holes.iter().all(finite) {
        return Err("pour needs a contour of at least 3 finite points".to_string());
    }
    Ok((outer, holes.into_iter().filter(|h| h.len() >= 3).collect()))
}

fn rect_ring(pour: &PcbCopperPourElm) -> Result<Vec<[f64; 2]>, String> {
    let c = pour.center.as_ref().ok_or("rect pour without center")?;
    let w = required(pour.width, "width")?;
    let h = required(pour.height, "height")?;
    let rot = finite_or(pour.rotation, 0.0, "rotation")?;
    Ok([[-w, -h], [w, -h], [w, h], [-w, h]]
        .map(|[x, y]| add([c.x, c.y], rotate([x / 2.0, y / 2.0], rot)))
        .to_vec())
}

fn points(pts: &[PcbPoint]) -> Vec<[f64; 2]> {
    pts.iter().map(|p| [p.x, p.y]).collect()
}

fn ring(r: &PcbRing) -> Vec<[f64; 2]> {
    points(&r.vertices)
}
