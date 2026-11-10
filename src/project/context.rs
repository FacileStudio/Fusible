//! Loading the project from the nearest manifest.

use std::path::{Path, PathBuf};

use crate::config::{ConfigError, DesignRules, FusibleConfig};

/// A loaded project: where it lives, its manifest when there is one, and its rules.
///
/// Without a manifest, `config` and `design` are defaults and `root` is the start directory.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Project {
    /// Directory of the manifest, or the start directory when there is none.
    pub root: PathBuf,
    /// Path of `fusible.yml` or `fusible.yaml`, when one was found.
    pub manifest: Option<PathBuf>,
    /// The parsed manifest, or the default configuration.
    pub config: FusibleConfig,
    /// The manifest's design rules with defaults applied.
    pub design: DesignRules,
    start: PathBuf,
}

impl Project {
    /// Loads the manifest found at `start` or in one of its ancestors.
    ///
    /// No manifest is not an error; an unreadable or invalid one is.
    pub fn load(start: &Path) -> Result<Self, ConfigError> {
        let found = FusibleConfig::find_and_load(start)?;
        Ok(Self::from_parts(start, found))
    }

    /// The project of the working directory; a manifest that does not load counts as none.
    ///
    /// `main` reports manifest errors before any command runs, so long-running hosts can
    /// treat an unreadable manifest like a missing one.
    pub fn current() -> Self {
        Self::discover(Path::new("."))
    }

    /// Loads the project again from the directory this one was loaded from.
    pub fn reload(&self) -> Self {
        Self::discover(&self.start)
    }

    /// The manifest path and configuration, when a manifest was found.
    pub fn found(&self) -> Option<(&Path, &FusibleConfig)> {
        self.manifest.as_deref().map(|path| (path, &self.config))
    }

    /// The directory holding the manifest, when a manifest was found.
    pub fn manifest_dir(&self) -> Option<&Path> {
        self.manifest.as_deref().and_then(Path::parent)
    }

    /// The design rules declared by the manifest, when a manifest was found.
    pub fn design_rules(&self) -> Option<&DesignRules> {
        self.manifest.as_ref().map(|_| &self.design)
    }

    fn discover(start: &Path) -> Self {
        Self::load(start).unwrap_or_else(|_| Self::from_parts(start, None))
    }

    /// Builds a project from a manifest already loaded from `start`, or from none.
    pub fn from_parts(start: &Path, found: Option<(PathBuf, FusibleConfig)>) -> Self {
        let (manifest, config) = found.unzip();
        let config = config.unwrap_or_default();
        let root = manifest
            .as_deref()
            .and_then(Path::parent)
            .map_or_else(|| start.to_path_buf(), Path::to_path_buf);
        let design = DesignRules::from_config(&config).with_bom(manifest.as_deref());
        Self {
            root,
            manifest,
            design,
            config,
            start: start.to_path_buf(),
        }
    }
}
