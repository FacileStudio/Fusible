//! The `firmware:` section: a crate bound to one board, its generated files and commands.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// A firmware crate bound to one board, with its generated bindings, build and flash commands.
#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FirmwareConfig {
    #[serde(default)]
    pub path: Option<PathBuf>,
    /// `embassy`, `cortex-m`, `esp-idf`, `esp-idf-c` or `zephyr`.
    #[serde(default)]
    pub framework: Option<String>,
    #[serde(default, alias = "pins-rs")]
    pub pins_rs: Option<PathBuf>,
    /// Generated binding files: a list (`[rust, c]`) or a map of each kind to its path.
    #[serde(default)]
    pub outputs: Option<Outputs>,
    #[serde(default, alias = "build-cmd")]
    pub build_cmd: Option<String>,
    /// Command `fusible flash` runs; the framework picks one when absent.
    #[serde(default, alias = "flash-cmd")]
    pub flash_cmd: Option<String>,
    /// Board the firmware runs on; optional when the project has a single board.
    #[serde(default)]
    pub board: Option<String>,
}

/// A generated binding file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputKind {
    /// `pins.rs`, plus `board.rs` beside it for Embassy.
    Rust,
    /// `pins.h`.
    C,
    /// A Zephyr devicetree overlay (experimental).
    Zephyr,
}

/// `outputs:` as written: kinds with default paths, or kinds with explicit paths.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Outputs {
    Kinds(Vec<OutputKind>),
    Paths(BTreeMap<OutputKind, PathBuf>),
}

/// The firmware framework, which picks the default flash command and `board.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framework {
    Embassy,
    CortexM,
    EspIdf,
    EspIdfC,
    Zephyr,
}

impl Framework {
    /// Reads a `framework:` value; `None` for one Fusible does not know.
    pub fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "embassy" | "embassy-stm32" | "embassy-rp" => Some(Self::Embassy),
            "cortex-m" | "rtic" => Some(Self::CortexM),
            "esp-idf" | "esp-idf-rs" | "esp-hal" => Some(Self::EspIdf),
            "esp-idf-c" => Some(Self::EspIdfC),
            "zephyr" => Some(Self::Zephyr),
            _ => None,
        }
    }
}

impl FirmwareConfig {
    /// The parsed `framework:`, if Fusible knows it.
    pub fn framework_kind(&self) -> Option<Framework> {
        self.framework.as_deref().and_then(Framework::parse)
    }

    /// Every binding file to write, with its path. Without `outputs:`, the 1.x behaviour:
    /// `rust` at `pins_rs`, or nothing.
    pub fn output_paths(&self) -> Vec<(OutputKind, PathBuf)> {
        match &self.outputs {
            None => self
                .pins_rs
                .iter()
                .map(|p| (OutputKind::Rust, p.clone()))
                .collect(),
            Some(Outputs::Paths(map)) => map.iter().map(|(k, p)| (*k, p.clone())).collect(),
            Some(Outputs::Kinds(kinds)) => {
                kinds.iter().map(|k| (*k, self.default_path(*k))).collect()
            }
        }
    }

    /// True for an Embassy firmware whose `outputs:` names `rust`: it also gets `board.rs`.
    pub fn wants_board_rs(&self) -> bool {
        let rust = self
            .output_paths()
            .iter()
            .any(|(k, _)| *k == OutputKind::Rust);
        self.outputs.is_some() && rust && self.framework_kind() == Some(Framework::Embassy)
    }

    fn default_path(&self, kind: OutputKind) -> PathBuf {
        let root = self.path.clone().unwrap_or_default();
        match kind {
            OutputKind::Rust => self
                .pins_rs
                .clone()
                .unwrap_or_else(|| root.join("src/pins.rs")),
            OutputKind::C => root.join("include/pins.h"),
            OutputKind::Zephyr => root.join("app.overlay"),
        }
    }

    /// Resolves the relative paths of an `outputs:` map against `dir`.
    pub(super) fn rebase_outputs(&mut self, dir: &Path) {
        if let Some(Outputs::Paths(map)) = self.outputs.as_mut() {
            for path in map.values_mut().filter(|p| p.is_relative()) {
                *path = dir.join(&*path);
            }
        }
    }
}
