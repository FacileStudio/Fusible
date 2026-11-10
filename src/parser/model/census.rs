//! Facts read from every element whatever its type.

use serde_json::Value;

use super::Model;

const FOOTPRINT_KEYS: [&str; 2] = ["footprinter_string", "footprint"];
const SHEET_KEYS: [&str; 2] = ["schematic_sheet_id", "schematicSheetId"];

fn first_text<'a>(element: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| element.get(*key)?.as_str())
}

/// Counts `element` by type and records its footprint and schematic sheet.
pub(super) fn count(model: &mut Model, element: &Value) {
    let kind = element.get("type").and_then(Value::as_str).unwrap_or("");
    *model.type_counts.entry(kind.to_string()).or_default() += 1;
    if let Some(footprint) = first_text(element, &FOOTPRINT_KEYS) {
        model.footprints.push(footprint.to_string());
    }
    if let Some(sheet) = first_text(element, &SHEET_KEYS) {
        model.schematic_sheet_ids.insert(sheet.to_string());
    }
}

impl Model {
    /// How many elements of type `kind` the file holds, readable or not.
    pub fn count(&self, kind: &str) -> usize {
        self.type_counts.get(kind).copied().unwrap_or(0)
    }

    /// True when some element's type satisfies `test`.
    pub fn has_type(&self, test: impl Fn(&str) -> bool) -> bool {
        self.type_counts.keys().any(|kind| test(kind))
    }
}
