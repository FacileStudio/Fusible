//! Renamed rule ids, rewritten only where the manifest names a rule: the keys of `rules:`
//! and the `rule:` values of `waivers:`. A comment, a part or a net that happens to spell an
//! old id is left alone.

use crate::commands::body::yaml_lines::{indent_of, key_value};

/// Rule ids renamed in 2.0, old to new.
const RENAMED_RULES: &[(&str, &str)] = &[("E_SCHEMATIC_SCALE_EXCESSIVE", "schematic_scale")];

fn renamed(id: &str) -> Option<&'static str> {
    RENAMED_RULES
        .iter()
        .find(|(old, _)| *old == id)
        .map(|(_, new)| *new)
}

fn replace_after(line: &str, at: usize, old: &str, new: &str) -> String {
    let (head, tail) = line.split_at(at);
    format!("{head}{}", tail.replacen(old, new, 1))
}

fn rename_key(line: &str) -> Option<String> {
    let indent = indent_of(line);
    let (key, _) = key_value(line).filter(|_| indent > 0)?;
    Some(replace_after(line, indent, key, renamed(key)?))
}

fn rename_value(line: &str) -> Option<String> {
    let body = line.trim_start().trim_start_matches('-').trim_start();
    let (_, value) = key_value(body).filter(|(key, _)| *key == "rule")?;
    let id = value.split('#').next().unwrap_or("").trim();
    let at = line.find("rule:")? + "rule:".len();
    Some(replace_after(line, at, id, renamed(id)?))
}

/// `text` with every renamed rule id of `rules:` and `waivers:` replaced by its new id.
pub(super) fn rename(text: &str) -> String {
    let mut section = "";
    text.split_inclusive('\n')
        .map(|full| {
            let line = full.trim_end_matches(['\n', '\r']);
            if indent_of(line) == 0 {
                section = key_value(line).map_or(section, |(key, _)| key);
            }
            let new = match section {
                "rules" => rename_key(line),
                "waivers" => rename_value(line),
                _ => None,
            };
            new.map_or_else(|| full.to_string(), |new| full.replacen(line, &new, 1))
        })
        .collect()
}
