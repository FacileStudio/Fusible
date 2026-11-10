//! Design rules declared in the manifest: nets, fab profile, clearance class, copper weight.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use super::fab::builtin_fab;
pub use super::fab::FabProfile;
use super::FusibleConfig;

fn default_outer_oz() -> f64 {
    1.0
}

fn default_inner_oz() -> f64 {
    0.5
}

/// What a net must withstand: peak voltage, current and the allowed temperature rise.
#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetSpec {
    #[serde(default)]
    pub voltage: Option<f64>,
    #[serde(default)]
    pub current: Option<f64>,
    #[serde(default)]
    pub max_rise: Option<f64>,
}

/// The `fab:` key: a built-in profile name or a custom profile.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FabSpec {
    Named(String),
    Builtin {
        profile: String,
        #[serde(default)]
        assembly_service: bool,
    },
    Custom(Box<FabProfile>),
}

/// Copper weights in ounces per square foot.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CopperSpec {
    #[serde(default = "default_outer_oz")]
    pub outer_oz: f64,
    #[serde(default = "default_inner_oz")]
    pub inner_oz: f64,
}

impl Default for CopperSpec {
    fn default() -> Self {
        Self {
            outer_oz: default_outer_oz(),
            inner_oz: default_inner_oz(),
        }
    }
}

/// IPC-2221B conductor spacing class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClearanceClass {
    B1,
    #[default]
    B2,
    B4,
}

impl ClearanceClass {
    /// Parses `B1`, `B2` or `B4`.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "B1" => Some(Self::B1),
            "B2" => Some(Self::B2),
            "B4" => Some(Self::B4),
            _ => None,
        }
    }
}

impl std::fmt::Display for ClearanceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// The manifest's design rules with defaults applied.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DesignRules {
    pub nets: BTreeMap<String, NetSpec>,
    pub fab: Option<FabProfile>,
    pub clearance_class: ClearanceClass,
    pub copper: CopperSpec,
    /// BOM line ids that `bom.yml` gives a manufacturer part number.
    pub bom_parts: BTreeSet<String>,
}

impl DesignRules {
    /// Builds the design rules of a validated manifest.
    pub fn from_config(cfg: &FusibleConfig) -> Self {
        let fab = cfg.fab.as_ref().and_then(|spec| match spec {
            FabSpec::Named(name) => builtin_fab(name),
            FabSpec::Builtin {
                profile,
                assembly_service,
            } => builtin_fab(profile).map(|fab| FabProfile {
                assembly_service: *assembly_service,
                ..fab
            }),
            FabSpec::Custom(profile) => Some(FabProfile {
                name: profile_name(profile),
                ..*profile.clone()
            }),
        });
        Self {
            nets: cfg.nets.clone(),
            fab,
            clearance_class: cfg
                .clearance_class
                .as_deref()
                .and_then(ClearanceClass::parse)
                .unwrap_or_default(),
            copper: cfg.copper.clone().unwrap_or_default(),
            bom_parts: BTreeSet::new(),
        }
    }

    /// Reads the `bom.yml` beside `manifest`; an unreadable file adds nothing,
    /// `fusible bom` reports it.
    pub fn with_bom(mut self, manifest: Option<&Path>) -> Self {
        let file = manifest.and_then(|m| crate::bom::BomFile::load(&crate::bom::bom_path(m)).ok());
        let parts = file.map(|f| f.parts).unwrap_or_default();
        let named = parts.into_iter().filter(|(_, p)| p.mpn.is_some());
        self.bom_parts = named.map(|(id, _)| id).collect();
        self
    }
}

fn profile_name(profile: &FabProfile) -> String {
    match profile.name.is_empty() {
        true => "custom".into(),
        false => profile.name.clone(),
    }
}
