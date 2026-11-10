//! The board outline as a closed polygon, and how far a shape sits inside it.

use crate::linter::rules::copper::Shape;
use crate::linter::rules::geom2d::Point;
use crate::parser::model::{Model, PcbBoard};

/// The outline of the first board, vertices in order.
pub(in crate::linter::rules) struct Outline {
    points: Vec<Point>,
}

fn rect(board: &PcbBoard) -> Option<Vec<Point>> {
    let (w, h) = board.width.zip(board.height)?;
    let c = board.center.map(|c| (c.x, c.y))?;
    let (hw, hh) = (w / 2.0, h / 2.0);
    Some(vec![
        (c.0 - hw, c.1 - hh),
        (c.0 + hw, c.1 - hh),
        (c.0 + hw, c.1 + hh),
        (c.0 - hw, c.1 + hh),
    ])
}

/// The outline of the model's first board, from `outline` or its centred
/// rectangle. A rectangle without a `center` has no known position: `None`.
pub(in crate::linter::rules) fn outline(model: &Model) -> Option<Outline> {
    let board = model.boards.first()?;
    let drawn = board.outline.as_ref().filter(|p| p.len() >= 3);
    let points = match drawn {
        Some(points) => points.iter().map(|p| (p.x, p.y)).collect(),
        None => rect(board)?,
    };
    Some(Outline { points })
}

impl Outline {
    fn edges(&self) -> impl Iterator<Item = (Point, Point)> + '_ {
        let n = self.points.len();
        (0..n).map(move |i| (self.points[i], self.points[(i + 1) % n]))
    }

    /// Whether `p` lies inside the outline (even-odd rule).
    pub fn contains(&self, p: Point) -> bool {
        let crossing = |&(a, b): &(Point, Point)| {
            (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1)
        };
        self.edges().filter(crossing).count() % 2 == 1
    }

    /// Distance from `shape`'s edge to the nearest board edge; 0 when the
    /// shape's anchor lies outside the board.
    pub fn gap(&self, shape: &Shape) -> f64 {
        let anchor = match shape {
            Shape::Seg { a, .. } => *a,
            Shape::Rect { c, .. } => *c,
        };
        if !self.contains(anchor) {
            return 0.0;
        }
        let edge = |(a, b): (Point, Point)| shape.gap(&Shape::Seg { a, b, half: 0.0 });
        self.edges().map(edge).fold(f64::MAX, f64::min)
    }

    /// The mean of the outline vertices, for locating a board-wide finding.
    pub fn centre(&self) -> Point {
        let n = self.points.len() as f64;
        let sum = self
            .points
            .iter()
            .fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
        (sum.0 / n, sum.1 / n)
    }
}
