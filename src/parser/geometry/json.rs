//! Tolerant readers for loosely typed Circuit JSON values.

use serde_json::Value;

pub(super) fn num(el: &Value, key: &str) -> Option<f64> {
    el.get(key)?.as_f64().filter(|v| v.is_finite())
}

pub(super) fn text<'a>(el: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| el.get(*key)?.as_str())
}

pub(super) fn strings(el: &Value, keys: &[&str]) -> Vec<String> {
    let list = keys.iter().find_map(|key| el.get(*key)?.as_array());
    let items = list.into_iter().flatten();
    items
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect()
}
