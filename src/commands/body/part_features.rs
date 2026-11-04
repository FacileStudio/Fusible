//! The features a code part records while it is built, and the report cached with them.

use serde::{Deserialize, Serialize};

/// The component that reaches deepest under the board, with how far below its bottom face.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Below {
    pub component: String,
    pub depth: f64,
}

/// A feature recorded by a helper of `Cad`, or measured on the finished shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Feature {
    Wall {
        thickness: f64,
        #[serde(default)]
        part: Option<String>,
    },
    Boss {
        #[serde(default)]
        insert: Option<String>,
        #[serde(default)]
        bore: Option<f64>,
        #[serde(default)]
        pilot: Option<f64>,
        outer: f64,
        #[serde(default)]
        h: Option<f64>,
        #[serde(default)]
        below: Option<Below>,
    },
    Standoff {
        h: f64,
        #[serde(default)]
        below: Option<Below>,
    },
    Vent {
        w: f64,
    },
    Port {
        margin: f64,
    },
    Draft {
        deg: f64,
    },
    Clearance {
        gap: f64,
    },
    Interference {
        component: String,
        volume: f64,
        gap: f64,
    },
    #[serde(other)]
    Other,
}

/// What a build prints and the cache keeps: features, bounding box size and volume.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PartReport {
    pub features: Vec<Feature>,
    #[serde(default)]
    pub bbox: Option<[f64; 3]>,
    #[serde(default)]
    pub volume_mm3: Option<f64>,
}

impl PartReport {
    /// Reads the JSON line a build script printed; OpenCascade may print noise before it.
    pub(crate) fn from_output(stdout: &str) -> Option<Self> {
        stdout
            .lines()
            .rev()
            .find(|line| line.starts_with('{'))
            .and_then(|line| serde_json::from_str(line).ok())
    }
}
