//! Checks of the design-rule keys: `nets`, `fab`, `clearance_class` and `copper`.

use super::design::{ClearanceClass, FabProfile, FabSpec};
use super::fab::builtin_fab_names;
use super::FusibleConfig;

/// Fails on an unsupported class, an unknown fab or a non-positive quantity.
pub(super) fn validate_design(cfg: &FusibleConfig) -> Result<(), String> {
    if let Some(class) = &cfg.clearance_class {
        clearance_class(class)?;
    }
    match &cfg.fab {
        Some(FabSpec::Named(name)) => named_fab(name)?,
        Some(FabSpec::Custom(profile)) => custom_fab(profile)?,
        None => {}
    }
    nets(cfg)?;
    if let Some(copper) = &cfg.copper {
        positive("copper.outer_oz", Some(copper.outer_oz))?;
        positive("copper.inner_oz", Some(copper.inner_oz))?;
    }
    Ok(())
}

fn clearance_class(class: &str) -> Result<(), String> {
    match ClearanceClass::parse(class) {
        Some(_) => Ok(()),
        None => Err(format!(
            "clearance_class `{class}` is not supported: B3, A5, A6, A7 are not supported yet; \
             use B1, B2 or B4"
        )),
    }
}

fn named_fab(name: &str) -> Result<(), String> {
    if builtin_fab_names().contains(&name) {
        return Ok(());
    }
    Err(format!(
        "fab `{name}` is not a built-in profile; known profiles: {}. \
         Write the limits inline for another fab",
        builtin_fab_names().join(", ")
    ))
}

fn custom_fab(profile: &FabProfile) -> Result<(), String> {
    let limits = [
        ("min_trace", profile.min_trace),
        ("min_space", profile.min_space),
        ("min_drill", profile.min_drill),
        ("min_via_pad", profile.min_via_pad),
        ("min_annular", profile.min_annular),
        ("min_via_to_via", profile.min_via_to_via),
        ("drill_step", profile.drill_step),
    ];
    for (key, value) in limits {
        positive(&format!("fab.{key}"), Some(value))?;
    }
    for value in profile.board_min.iter().chain(&profile.board_max) {
        positive("fab.board_min/board_max", Some(*value))?;
    }
    Ok(())
}

fn nets(cfg: &FusibleConfig) -> Result<(), String> {
    for (name, spec) in &cfg.nets {
        positive(&format!("nets.{name}.voltage"), spec.voltage)?;
        positive(&format!("nets.{name}.current"), spec.current)?;
        positive(&format!("nets.{name}.max_rise"), spec.max_rise)?;
        positive(&format!("nets.{name}.limit"), spec.limit)?;
        for (part, amps) in &spec.loads {
            positive(&format!("nets.{name}.loads.{part}"), Some(*amps))?;
        }
    }
    Ok(())
}

fn positive(key: &str, value: Option<f64>) -> Result<(), String> {
    match value {
        Some(v) if !(v.is_finite() && v > 0.0) => Err(format!(
            "{key} must be a finite number greater than 0, got {v}"
        )),
        _ => Ok(()),
    }
}
