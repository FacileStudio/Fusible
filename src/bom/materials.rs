//! The material library: density and indicative price per material, for body mass and cost.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

const LIBRARY: &str = include_str!("materials.yml");

/// Manufacturing processes a body can declare.
pub const PROCESSES: &[&str] = &["fdm", "sla", "mjf", "injection", "cnc"];

/// Material kept when a body names none.
pub const DEFAULT_MATERIAL: &str = "pla";

/// One row of `materials.yml`. Prices are indicative euros.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Material {
    pub name: String,
    pub density: f64,
    pub cost_per_kg: f64,
    pub max_temp_c: f64,
    pub processes: Vec<String>,
}

/// Every known material, keyed by the name used in `fusible.yml`.
pub fn library() -> &'static BTreeMap<String, Material> {
    static PARSED: OnceLock<BTreeMap<String, Material>> = OnceLock::new();
    PARSED.get_or_init(|| serde_yaml::from_str(LIBRARY).expect("materials.yml is valid"))
}

/// Fails unless `material` is in the library and `process` is one it supports.
pub fn validate(material: Option<&str>, process: Option<&str>) -> Result<(), String> {
    if let Some(process) = process.filter(|p| !PROCESSES.contains(p)) {
        return Err(format!(
            "unknown process `{process}`; known processes: {}",
            PROCESSES.join(", ")
        ));
    }
    let Some(key) = material else {
        return Ok(());
    };
    let found = library().get(key).ok_or_else(|| {
        let known: Vec<&str> = library().keys().map(String::as_str).collect();
        format!("unknown material `{key}`; known materials: {}", known.join(", "))
    })?;
    match process {
        Some(p) if !found.processes.iter().any(|s| s == p) => Err(format!(
            "material `{key}` does not support process `{p}`; it supports: {}",
            found.processes.join(", ")
        )),
        _ => Ok(()),
    }
}

/// The material key and process of a body: PLA when it names no material, and the first
/// process of the material when it names none.
pub fn resolve(material: Option<&str>, process: Option<&str>) -> (String, String) {
    let key = material.unwrap_or(DEFAULT_MATERIAL);
    let first = library()
        .get(key)
        .and_then(|m| m.processes.first().cloned())
        .unwrap_or_else(|| "fdm".to_string());
    (key.to_string(), process.map_or(first, str::to_string))
}
