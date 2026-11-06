//! The paired devices, kept as token hashes in `devices.json`.

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::clock::rfc3339;
use super::token::{ct_eq, sha256_hex};

const TOUCH_EVERY: Duration = Duration::from_secs(60);

/// One paired phone. The token itself is never stored, only its hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Device {
    pub device_id: String,
    pub name: String,
    pub token_sha256: String,
    pub paired_at: String,
    pub last_seen: String,
}

fn private_file(path: &std::path::Path) -> std::io::Result<std::fs::File> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options.open(path)
}

/// The device list and where it is saved.
pub(super) struct Devices {
    path: Option<PathBuf>,
    list: Vec<Device>,
    touched: HashMap<String, Instant>,
}

impl Devices {
    /// Reads the list at `path`; a missing or unreadable file is an empty list.
    pub(super) fn load(path: Option<PathBuf>) -> Self {
        let read = |p: &PathBuf| std::fs::read_to_string(p).ok();
        let list = path
            .as_ref()
            .and_then(read)
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        Self {
            path,
            list,
            touched: HashMap::new(),
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        let tmp = path.with_extension("json.tmp");
        let _ = std::fs::remove_file(&tmp);
        let mut file = private_file(&tmp)?;
        file.write_all(&serde_json::to_vec_pretty(&self.list)?)?;
        file.sync_all()?;
        std::fs::rename(&tmp, path)
    }

    /// Stores a new device and saves the list.
    pub(super) fn add(&mut self, device: Device, now: Instant) -> std::io::Result<()> {
        self.touched.insert(device.device_id.clone(), now);
        self.list.push(device);
        self.save()
    }

    /// The id of the device whose token hashes to `sha256(token)`.
    pub(super) fn authenticate(&self, token: &str) -> Option<String> {
        let hash = sha256_hex(token);
        let mut found = None;
        for device in &self.list {
            if ct_eq(hash.as_bytes(), device.token_sha256.as_bytes()) {
                found = Some(device.device_id.clone());
            }
        }
        found
    }

    /// Records a request from `id`, writing the file at most once a minute per device.
    pub(super) fn touch(&mut self, id: &str, now: Instant, unix: i64) {
        let fresh = self
            .touched
            .get(id)
            .is_some_and(|t| now.duration_since(*t) < TOUCH_EVERY);
        if fresh {
            return;
        }
        if let Some(device) = self.list.iter_mut().find(|d| d.device_id == id) {
            device.last_seen = rfc3339(unix);
            self.touched.insert(id.to_string(), now);
            let _ = self.save();
        }
    }

    /// Removes a device; false when the id is unknown.
    pub(super) fn remove(&mut self, id: &str) -> bool {
        let before = self.list.len();
        self.list.retain(|d| d.device_id != id);
        let removed = self.list.len() != before;
        if removed {
            let _ = self.save();
        }
        removed
    }

    /// The devices without their token hashes.
    pub(super) fn view(&self) -> Vec<Value> {
        let show = |d: &Device| {
            let (id, name) = (&d.device_id, &d.name);
            json!({ "device_id": id, "name": name, "paired_at": d.paired_at, "last_seen": d.last_seen })
        };
        self.list.iter().map(show).collect()
    }
}

#[cfg(all(test, unix))]
#[path = "store_tests.rs"]
mod tests;
