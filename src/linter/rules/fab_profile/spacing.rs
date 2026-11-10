//! Same-layer spacing between copper of distinct nets.

use std::collections::BTreeMap;

use super::{violation, EPSILON};
use crate::config::FabProfile;
use crate::linter::rules::copper::{collect, neighbours, Copper, Shape};
use crate::linter::rules::physics::mm;
use crate::linter::{Diagnostic, LintContext};
use crate::parser::geometry::Geometry;

type Worst = (f64, usize, usize);

fn worst_pairs(items: &[Copper], min: f64) -> BTreeMap<(String, String), Worst> {
    let mut worst: BTreeMap<(String, String), Worst> = BTreeMap::new();
    for (i, j) in neighbours(items, min) {
        let (a, b) = (&items[i], &items[j]);
        let gap = a.shape.gap(&b.shape);
        let trace = |c: &&Copper| matches!(c.shape, Shape::Seg { .. }) && !c.shape.is_disc();
        let has_trace = [a, b].iter().any(trace);
        let key = (
            a.net.clone().min(b.net.clone()),
            a.net.clone().max(b.net.clone()),
        );
        if has_trace && gap < min - EPSILON && worst.get(&key).is_none_or(|w| gap < w.0) {
            worst.insert(key, (gap, i, j));
        }
    }
    worst
}

/// Spacing violations involving at least one trace. Pad-to-pad spacing belongs
/// to `trace_clearance`, which reads the profile's `min_space` when a fab is set.
pub(super) fn tight_spacing(
    ctx: &LintContext<'_>,
    profile: &FabProfile,
    geometry: &Geometry,
) -> Vec<Diagnostic> {
    let items = collect(ctx, geometry);
    let found = worst_pairs(&items, profile.min_space).into_iter();
    found
        .map(|((net_a, net_b), (gap, i, j))| {
            let message = format!(
                "spacing {} mm on {} between {net_a} and {net_b} is below {} minimum {} mm",
                mm(gap),
                items[i].layer,
                profile.name,
                mm(profile.min_space)
            );
            violation(
                profile,
                message,
                vec![items[i].id.clone(), items[j].id.clone()],
            )
        })
        .collect()
}
