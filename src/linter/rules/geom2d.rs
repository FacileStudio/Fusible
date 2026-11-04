//! Plane distances between points, segments and axis-aligned rectangles.

pub(super) type Point = (f64, f64);

pub(super) fn dist(p: Point, q: Point) -> f64 {
    (p.0 - q.0).hypot(p.1 - q.1)
}

pub(super) fn cross(o: Point, a: Point, b: Point) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

pub(super) fn point_seg(p: Point, a: Point, b: Point) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = match len2 == 0.0 {
        true => 0.0,
        false => (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0),
    };
    dist(p, (a.0 + t * dx, a.1 + t * dy))
}

pub(super) fn crosses(a: Point, b: Point, c: Point, d: Point) -> bool {
    cross(a, b, c) * cross(a, b, d) < 0.0 && cross(c, d, a) * cross(c, d, b) < 0.0
}

pub(super) fn seg_seg(a: Point, b: Point, c: Point, d: Point) -> f64 {
    if crosses(a, b, c, d) {
        return 0.0;
    }
    [
        point_seg(a, c, d),
        point_seg(b, c, d),
        point_seg(c, a, b),
        point_seg(d, a, b),
    ]
    .into_iter()
    .fold(f64::MAX, f64::min)
}

pub(super) fn point_rect(p: Point, c: Point, hw: f64, hh: f64) -> f64 {
    ((p.0 - c.0).abs() - hw)
        .max(0.0)
        .hypot(((p.1 - c.1).abs() - hh).max(0.0))
}

pub(super) fn seg_rect(a: Point, b: Point, c: Point, hw: f64, hh: f64) -> f64 {
    let corners = [
        (c.0 - hw, c.1 - hh),
        (c.0 + hw, c.1 - hh),
        (c.0 + hw, c.1 + hh),
        (c.0 - hw, c.1 + hh),
    ];
    let edges = (0..4).map(|i| (corners[i], corners[(i + 1) % 4]));
    if edges.clone().any(|(p, q)| crosses(a, b, p, q)) {
        return 0.0;
    }
    let ends = point_rect(a, c, hw, hh).min(point_rect(b, c, hw, hh));
    corners
        .iter()
        .map(|k| point_seg(*k, a, b))
        .fold(ends, f64::min)
}
