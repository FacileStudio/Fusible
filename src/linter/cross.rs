//! Rules that check more than one board: links between boards, the assembly and the bodies.
//!
//! They run outside the [`super::Linter`], but a manifest names them in `rules:` and
//! `waivers:` like any other, so their metadata lives in the registry too.

use super::{Category, RuleMeta, Severity};

const fn meta(id: &'static str, category: Category, summary: &'static str) -> RuleMeta {
    RuleMeta {
        id,
        category,
        summary,
        standard: None,
        source_url: None,
        default_severity: Severity::Error,
    }
}

const fn mech(id: &'static str, summary: &'static str) -> RuleMeta {
    meta(id, Category::Mechanical, summary)
}

/// Every cross-world rule, links first, then assembly and bodies.
pub const CROSS_WORLD: &[RuleMeta] = &[
    meta(
        "harness_mismatch",
        Category::Erc,
        "Both ends of a `links:` wire name an existing part, pin and net.",
    ),
    mech(
        "body_interference",
        "Two bodies, or a body and a board, never share volume.",
    ),
    mech(
        "hole_misaligned",
        "Every non-plated hole of a fitted board meets a boss and an empty bore.",
    ),
    mech(
        "connector_blocked",
        "A connector's opening is clear of body material.",
    ),
    mech(
        "mech.wall_thickness",
        "Walls are thick enough for the process, and not so thick that a moulding sinks.",
    ),
    mech(
        "mech.boss",
        "Screw bosses are wide enough for their hole and bored for their insert.",
    ),
    mech(
        "mech.port_clearance",
        "A connector cutout leaves the margin the process needs on each side.",
    ),
    mech(
        "mech.vent_width",
        "Vent slots are at least as wide as the process allows.",
    ),
    mech(
        "mech.standoff_height",
        "Standoffs are taller than the deepest part under the board.",
    ),
    mech(
        "mech.component_clearance",
        "A body keeps its gap around the board and the parts inside it.",
    ),
    mech("mech.draft", "A moulded body declares enough draft."),
    mech(
        "mech.stale",
        "A body was rebuilt after its source last changed.",
    ),
    mech("mech.not_built", "A declared body has been built."),
];

/// The ids of the cross-world rules in `category`.
pub fn cross_world_ids(category: Category) -> impl Iterator<Item = &'static str> {
    CROSS_WORLD
        .iter()
        .filter(move |meta| meta.category == category)
        .map(|meta| meta.id)
}
