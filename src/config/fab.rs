//! Built-in fab profiles, embedded from `linter/rules/data/fab/`.

use super::design::FabProfile;

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
