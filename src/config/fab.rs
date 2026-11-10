//! Fab profiles: what a fab house can build, and the built-in ones embedded
//! from `linter/rules/data/fab/`.

use serde::Deserialize;

/// What a fab house can build. Lengths are in millimetres.
///
/// The optional limits stay `None` when the fab's capability page does not
/// publish the number; the rule that reads it is then disabled for that
/// profile and says so with one `info` diagnostic.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct FabProfile {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub source: String,
    pub min_trace: f64,
    pub min_space: f64,
    pub min_drill: f64,
    pub min_via_pad: f64,
    /// Plated through-hole ring, pad edge to drill edge.
    pub min_annular: f64,
    pub min_via_to_via: f64,
    pub drill_step: f64,
    pub board_min: [f64; 2],
    pub board_max: [f64; 2],
    /// Via ring, pad edge to drill edge.
    #[serde(default)]
    pub min_via_annular: Option<f64>,
    /// Copper to the routed board edge.
    #[serde(default)]
    pub min_edge_clearance: Option<f64>,
    /// Drill edge to copper of another net.
    #[serde(default)]
    pub min_drill_to_copper: Option<f64>,
    /// Drill edge to drill edge between holes.
    #[serde(default)]
    pub min_hole_spacing: Option<f64>,
    /// Board thickness over the smallest plated drill.
    #[serde(default)]
    pub max_aspect_ratio: Option<f64>,
    /// Solder mask web between two openings.
    #[serde(default)]
    pub min_mask_dam: Option<f64>,
    /// Silkscreen text height.
    #[serde(default)]
    pub min_silk_height: Option<f64>,
    /// Silkscreen line width.
    #[serde(default)]
    pub min_silk_stroke: Option<f64>,
    /// Silkscreen to pad copper.
    #[serde(default)]
    pub min_silk_to_pad: Option<f64>,
    /// Part body to the board edge.
    #[serde(default)]
    pub min_component_to_edge: Option<f64>,
    /// The fab also assembles the board: fiducials become required.
    #[serde(default)]
    pub assembly_service: bool,
}

const BUILTIN_FABS: &[(&str, &str)] = &[
    (
        "jlcpcb-2l",
        include_str!("../linter/rules/data/fab/jlcpcb-2l.yml"),
    ),
    (
        "jlcpcb-4l",
        include_str!("../linter/rules/data/fab/jlcpcb-4l.yml"),
    ),
];

/// Returns the built-in fab profile called `name`.
pub fn builtin_fab(name: &str) -> Option<FabProfile> {
    let (_, text) = BUILTIN_FABS.iter().find(|(known, _)| *known == name)?;
    serde_yaml::from_str(text).ok()
}

/// Names of the built-in fab profiles.
pub fn builtin_fab_names() -> Vec<&'static str> {
    BUILTIN_FABS.iter().map(|(name, _)| *name).collect()
}

/// The optional limits of `profile`, by key, for validation.
pub(super) fn optional_limits(profile: &FabProfile) -> [(&'static str, Option<f64>); 10] {
    [
        ("min_via_annular", profile.min_via_annular),
        ("min_edge_clearance", profile.min_edge_clearance),
        ("min_drill_to_copper", profile.min_drill_to_copper),
        ("min_hole_spacing", profile.min_hole_spacing),
        ("max_aspect_ratio", profile.max_aspect_ratio),
        ("min_mask_dam", profile.min_mask_dam),
        ("min_silk_height", profile.min_silk_height),
        ("min_silk_stroke", profile.min_silk_stroke),
        ("min_silk_to_pad", profile.min_silk_to_pad),
        ("min_component_to_edge", profile.min_component_to_edge),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_profile_parses_with_its_source_and_new_limits() {
        for name in builtin_fab_names() {
            let fab = builtin_fab(name).expect("built-in profile must parse");
            assert_eq!(fab.name, name);
            assert!(fab.source.starts_with("https://"), "{name}");
            assert!(fab.min_edge_clearance.is_some(), "{name}");
            assert!(fab.max_aspect_ratio.is_none(), "{name}");
            assert!(!fab.assembly_service, "{name}");
        }
    }
}
