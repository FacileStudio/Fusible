//! Errors and warnings tscircuit itself recorded in the Circuit JSON.

use serde_json::Value;

/// One `*_error` or `*_warning` element, whatever its type.
///
/// tscircuit adds new kinds often, so this keeps the type name as data
/// instead of one struct per kind. `element_ids` lists every id the element
/// points at (`*_id` strings and `*_ids` lists), except its own id and its
/// `subcircuit_id`, sorted and deduplicated. The own id is the `*_id` whose value starts with the type
/// name, because some kinds (`pcb_autorouting_error`) store it as `pcb_error_id`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpstreamIssue {
    /// The element type, e.g. `pcb_trace_error`.
    pub kind: String,
    /// The element's own id, e.g. `pcb_trace_error_0`.
    pub id: Option<String>,
    pub message: Option<String>,
    pub element_ids: Vec<String>,
    /// `true` for an `*_error`, `false` for an `*_warning`.
    pub is_error: bool,
}

const SKIPPED: [&str; 2] = ["subcircuit_id", "source_group_id"];

fn referenced(own: &str, object: &serde_json::Map<String, Value>) -> Vec<String> {
    let refs = object
        .iter()
        .filter(|(key, _)| key.as_str() != own && !SKIPPED.contains(&key.as_str()));
    let ids = refs.flat_map(|(key, value)| match value {
        Value::String(id) if key.ends_with("_id") => vec![id.clone()],
        Value::Array(list) if key.ends_with("_ids") => list
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    });
    let mut ids: Vec<String> = ids.collect();
    ids.sort();
    ids.dedup();
    ids
}

impl UpstreamIssue {
    /// Reads `element` when its `type` ends in `_error` or `_warning`.
    pub fn from_element(element: &Value) -> Option<Self> {
        let object = element.as_object()?;
        let kind = object.get("type")?.as_str()?;
        let is_error = kind.ends_with("_error");
        if !is_error && !kind.ends_with("_warning") {
            return None;
        }
        let named = object.iter().find(|(key, value)| {
            key.ends_with("_id") && value.as_str().is_some_and(|v| v.starts_with(kind))
        });
        let own = named.map_or(format!("{kind}_id"), |(key, _)| key.clone());
        let text = |key: &str| object.get(key).and_then(Value::as_str).map(str::to_string);
        Some(Self {
            kind: kind.to_string(),
            id: text(&own),
            message: text("message"),
            element_ids: referenced(&own, object),
            is_error,
        })
    }
}
