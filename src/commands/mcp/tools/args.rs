//! Typed access to tool arguments and the JSON Schema helpers tools share.

use serde_json::{json, Map, Value};

use super::outcome::Outcome;

/// A required string argument.
pub fn required_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, Outcome> {
    optional_str(args, key).ok_or_else(|| Outcome::fail(format!("{key} is required")))
}

/// An optional string argument.
pub fn optional_str<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(Value::as_str)
}

/// An optional boolean argument with a default.
pub fn flag(args: &Value, key: &str, default: bool) -> bool {
    args.get(key).and_then(Value::as_bool).unwrap_or(default)
}

/// A string property for an input schema.
pub fn string(description: &str) -> Value {
    json!({"type": "string", "description": description})
}

/// A boolean property for an input schema.
pub fn boolean(description: &str) -> Value {
    json!({"type": "boolean", "description": description})
}

/// A string property limited to `values`.
pub fn choice(description: &str, values: &[&str]) -> Value {
    json!({"type": "string", "description": description, "enum": values})
}

/// An object schema with the given properties and required keys.
pub fn object(properties: &[(&str, Value)], required: &[&str]) -> Value {
    let props: Map<String, Value> = properties
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect();
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": props,
        "required": required,
        "additionalProperties": false,
    })
}

/// A path relative to the project root, with forward slashes on every platform.
pub fn shown(root: &std::path::Path, path: &std::path::Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
