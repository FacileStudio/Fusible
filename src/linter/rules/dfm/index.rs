//! An R-tree over copper shapes, to find the few that sit near a feature.

use rstar::primitives::{GeomWithData, Rectangle};
use rstar::RTree;

use crate::linter::rules::copper::Shape;

type Entry = GeomWithData<Rectangle<[f64; 2]>, usize>;

/// Shapes indexed by their position in the slice they came from.
pub(in crate::linter::rules) struct Index {
    tree: RTree<Entry>,
}

impl Index {
    /// Indexes `shapes` by their bounding boxes.
    pub fn new<'a>(shapes: impl Iterator<Item = &'a Shape>) -> Self {
        let entries = shapes
            .enumerate()
            .map(|(i, s)| GeomWithData::new(Rectangle::from_aabb(s.envelope(0.0)), i));
        Self {
            tree: RTree::bulk_load(entries.collect()),
        }
    }

    /// Indices of the shapes whose boxes come within `margin` of `shape`'s box.
    pub fn near(&self, shape: &Shape, margin: f64) -> Vec<usize> {
        let found = self
            .tree
            .locate_in_envelope_intersecting(shape.envelope(margin));
        let mut near: Vec<usize> = found.map(|entry| entry.data).collect();
        near.sort_unstable();
        near
    }
}
