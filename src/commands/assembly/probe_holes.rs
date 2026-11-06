//! Hole probes: a near cylinder and four ring quadrants on each face, and a bore through the board.

use super::probe::{Frame, Probe, Role, Shape, Side};
use super::rules::{BORE_MARGIN, INSET, NEAR_MARGIN, PROBE_DEPTH, RING_WIDTH};
use super::shrink::{shrunk_polygon, shrunk_span};
use crate::commands::body::Kit;

struct Hole {
    index: usize,
    center: [f64; 2],
    radius: f64,
    plated: bool,
    exempt: bool,
}

struct Face {
    side: Side,
    z0: f64,
    z1: f64,
    tag: String,
}

fn near_probe(frame: &Frame, kit: &Kit, hole: &Hole, face: &Face) -> Probe {
    let near = Shape::Cylinder {
        center: hole.center,
        r: hole.radius + NEAR_MARGIN - INSET,
        z0: face.z0,
        z1: face.z1,
        clip: Some(shrunk_polygon(&kit.outline)),
    };
    let role = Role::HoleNear {
        hole: hole.index,
        side: face.side,
        at: hole.center,
        plated: hole.plated,
        exempt: hole.exempt,
    };
    frame.probe(format!("{}:near", face.tag), role, near)
}

fn quadrant_probe(frame: &Frame, hole: &Hole, face: &Face, quadrant: u8) -> Probe {
    let ring = Shape::Ring {
        center: hole.center,
        r0: hole.radius + INSET,
        r1: hole.radius + RING_WIDTH - INSET,
        z0: face.z0,
        z1: face.z1,
        quadrant,
    };
    let role = Role::HoleQuadrant {
        hole: hole.index,
        side: face.side,
    };
    frame.probe(format!("{}:q{quadrant}", face.tag), role, ring)
}

fn face_probes(frame: &Frame, kit: &Kit, hole: &Hole, side: Side) -> Vec<Probe> {
    let half = kit.thickness / 2.0;
    let (z0, z1) = match side {
        Side::Bottom => shrunk_span(-half - PROBE_DEPTH, -half),
        Side::Top => shrunk_span(half, half + PROBE_DEPTH),
    };
    let tag = format!("hole:{}:{}", hole.index, side.label());
    let face = Face { side, z0, z1, tag };
    let mut probes = vec![near_probe(frame, kit, hole, &face)];
    probes.extend((0..4u8).map(|q| quadrant_probe(frame, hole, &face, q)));
    probes
}

fn bore_probe(frame: &Frame, kit: &Kit, hole: &Hole) -> Option<Probe> {
    if hole.radius <= BORE_MARGIN {
        return None;
    }
    let half = kit.thickness / 2.0;
    let (z0, z1) = shrunk_span(-half - PROBE_DEPTH, half + PROBE_DEPTH);
    let bore = Shape::Cylinder {
        center: hole.center,
        r: hole.radius - BORE_MARGIN,
        z0,
        z1,
        clip: None,
    };
    let role = Role::HoleBore {
        hole: hole.index,
        at: hole.center,
    };
    Some(frame.probe(format!("hole:{}:bore", hole.index), role, bore))
}

/// Whether a component footprint of the kit covers the center of the hole.
fn under_a_part(kit: &Kit, center: [f64; 2]) -> bool {
    kit.components
        .iter()
        .any(|c| (center[0] - c.x).abs() <= c.w / 2.0 && (center[1] - c.y).abs() <= c.h / 2.0)
}

/// The probes of every hole of the kit, bottom face first.
///
/// A hole under a part or narrower than `min_hole` is exempt from the missing boss rule.
pub(super) fn hole_probes(frame: &Frame, kit: &Kit, min_hole: f64) -> Vec<Probe> {
    kit.holes
        .iter()
        .enumerate()
        .flat_map(|(index, h)| {
            let hole = Hole {
                index,
                center: [h.x, h.y],
                radius: h.diameter / 2.0,
                plated: h.plated,
                exempt: h.diameter < min_hole || under_a_part(kit, [h.x, h.y]),
            };
            let faces = [Side::Bottom, Side::Top]
                .into_iter()
                .flat_map(|side| face_probes(frame, kit, &hole, side));
            faces
                .chain(bore_probe(frame, kit, &hole))
                .collect::<Vec<_>>()
        })
        .collect()
}
