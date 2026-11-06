//! State of one MCP server run: the project root and the files this run wrote.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::{FusibleConfig, CONFIG_FILENAMES};

/// The project served by this process.
pub struct Session {
    pub root: PathBuf,
    pub manifest: Option<PathBuf>,
    written: HashMap<PathBuf, Vec<u8>>,
}

impl Session {
    /// Resolves the project root, moves the process into it and starts an empty session.
    pub fn open(root: Option<PathBuf>) -> Result<Self, String> {
        let root = match root {
            Some(dir) => dir
                .canonicalize()
                .map_err(|e| format!("{}: {e}", dir.display()))?,
            None => nearest_root()?,
        };
        std::env::set_current_dir(&root).map_err(|e| format!("{}: {e}", root.display()))?;
        let manifest = CONFIG_FILENAMES
            .iter()
            .map(|n| root.join(n))
            .find(|p| p.is_file());
        Ok(Self {
            root,
            manifest,
            written: HashMap::new(),
        })
    }

    /// Loads the manifest, or `None` when the project has none.
    pub fn config(&self) -> Result<Option<FusibleConfig>, String> {
        let Some(path) = &self.manifest else {
            return Ok(None);
        };
        FusibleConfig::from_file(path)
            .map(Some)
            .map_err(|e| e.to_string())
    }

    /// Records the bytes this session left in `file`.
    pub fn remember(&mut self, file: &Path) {
        if let Ok(bytes) = std::fs::read(file) {
            self.written.insert(file.to_path_buf(), bytes);
        }
    }

    /// Whether `file` still holds exactly what this session wrote.
    pub fn owns(&self, file: &Path, current: &[u8]) -> bool {
        self.written.get(file).is_some_and(|bytes| bytes == current)
    }
}

fn nearest_root() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let found = FusibleConfig::find_and_load(&cwd).map_err(|e| e.to_string())?;
    let dir = found.and_then(|(path, _)| path.parent().map(Path::to_path_buf));
    dir.unwrap_or(cwd).canonicalize().map_err(|e| e.to_string())
}
