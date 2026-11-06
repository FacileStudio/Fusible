//! The `.fusible/opus.json` file that remembers which tasks already exist.

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

const PATH: &str = ".fusible/opus.json";

/// Keys of tasks already created, mapped to their Opus task ids.
pub struct Store {
    path: PathBuf,
    tasks: BTreeMap<String, String>,
}

impl Store {
    /// Loads the store next to the manifest, or starts an empty one.
    pub fn load(root: &std::path::Path) -> Self {
        let path = root.join(PATH);
        let tasks = std::fs::read_to_string(&path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        Self { path, tasks }
    }

    /// Whether a task for `key` was already created.
    pub fn has(&self, key: &str) -> bool {
        self.tasks.contains_key(key)
    }

    /// Remembers the task id for `key` and writes the file.
    pub fn insert(&mut self, key: String, id: String) -> std::io::Result<()> {
        self.tasks.insert(key, id);
        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let text = serde_json::to_string_pretty(&self.tasks).unwrap_or_default();
        std::fs::write(&self.path, text + "\n")
    }
}

fn digest(seed: u8, text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    text.hash(&mut hasher);
    hasher.finish()
}

/// A 32-hex-char dedup key for `text`; not a security hash.
fn hash32(text: &str) -> String {
    format!("{:016x}{:016x}", digest(0, text), digest(1, text))
}

/// The dedup key of a blown-fuse error.
pub fn fuse_key(rule: &str, detail: &[String], board: &str) -> String {
    let mut sorted = detail.to_vec();
    sorted.sort();
    hash32(&format!("{rule}\n{}\n{board}", sorted.join(",")))
}
