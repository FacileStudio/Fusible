//! The `.fusible/opus.json` file that remembers which tasks already exist.

use std::collections::BTreeMap;
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

const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

/// A 32-hex-char dedup key for `text`: FNV-1a 128, stable across Rust versions; not a security hash.
fn hash32(text: &str) -> String {
    let hash = text.bytes().fold(FNV_OFFSET, |h, byte| {
        (h ^ u128::from(byte)).wrapping_mul(FNV_PRIME)
    });
    format!("{hash:032x}")
}

/// The dedup key of a blown-fuse error.
pub fn fuse_key(rule: &str, detail: &[String], board: &str) -> String {
    let mut sorted = detail.to_vec();
    sorted.sort();
    hash32(&format!("{rule}\n{}\n{board}", sorted.join(",")))
}

#[cfg(test)]
mod tests {
    use super::hash32;

    #[test]
    fn hash32_matches_the_fnv1a_128_vectors() {
        assert_eq!(hash32(""), "6c62272e07bb014262b821756295c58d");
        assert_eq!(hash32("a"), "d228cb696f1a8caf78912b704e4a8964");
        assert_eq!(hash32("foobar"), "343e1662793c64bf6f0d3597ba446f18");
    }
}
