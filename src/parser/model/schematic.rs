//! Schematic placement of each part.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::{Kind, Model};

/// Reads any value, `None` when it is absent or of the wrong shape.
fn loose<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|v| T::deserialize(v).ok()))
}

/// Reads a number, or a string that parses as one (`"NaN"` included).
fn coordinate<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|v| {
        v.as_f64()
            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
    }))
}

/// A point whose coordinates may be missing or sent as strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct LoosePoint {
    #[serde(deserialize_with = "coordinate")]
    pub x: Option<f64>,
    #[serde(deserialize_with = "coordinate")]
    pub y: Option<f64>,
}

/// A `schematic_component`. Every field is read leniently: a field of the
/// wrong shape is `None`, never a rejected element.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct SchematicComponent {
    #[serde(alias = "schematicComponentId", deserialize_with = "loose")]
    pub schematic_component_id: Option<String>,
    #[serde(alias = "sourceComponentId", deserialize_with = "loose")]
    pub source_component_id: Option<String>,
    #[serde(deserialize_with = "loose")]
    pub refdes: Option<String>,
    #[serde(deserialize_with = "loose")]
    pub name: Option<String>,
    #[serde(deserialize_with = "loose")]
    pub center: Option<LoosePoint>,
    #[serde(deserialize_with = "coordinate")]
    pub x: Option<f64>,
    #[serde(deserialize_with = "coordinate")]
    pub y: Option<f64>,
}

impl SchematicComponent {
    /// The center coordinates, each falling back to the top-level `x` / `y`.
    pub fn position(&self) -> (Option<f64>, Option<f64>) {
        let center = self.center.unwrap_or_default();
        (center.x.or(self.x), center.y.or(self.y))
    }
}

impl Kind for SchematicComponent {
    const TYPE: &'static str = "schematic_component";
    fn slot(model: &mut Model) -> &mut Vec<Self> {
        &mut model.schematic_components
    }
}
