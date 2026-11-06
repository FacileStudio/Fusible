//! The text edit behind `fusible body new`: one entry under `bodies:`, comments kept.

use super::yaml_lines::{bodies_section, indent_of, is_skippable};

fn new_section(old: &str, name: &str, fields: &[String]) -> Result<String, String> {
    let legacy = old
        .lines()
        .any(|l| l.starts_with("bodies:") || l.starts_with("mechanical:"));
    if legacy {
        return Err("`bodies:` is not a block, or the legacy `mechanical:` section is used".into());
    }
    let mut out = old.trim_end().to_string();
    out.push_str(&format!("\n\nbodies:\n  {name}:\n"));
    out.extend(fields.iter().map(|f| format!("    {f}\n")));
    Ok(out)
}

/// Returns `old` with the body `name` added under `bodies:`, creating the section when it is missing.
pub(super) fn add_entry(old: &str, name: &str, fields: &[String]) -> Result<String, String> {
    if old.contains('\r') {
        return Err("the file uses CRLF line endings".into());
    }
    let mut lines: Vec<String> = old.lines().map(str::to_string).collect();
    let Some(section) = bodies_section(&lines) else {
        return new_section(old, name, fields);
    };
    let indent = |l: &String| indent_of(l);
    let child = lines[section.clone()]
        .iter()
        .find(|l| !is_skippable(l))
        .map_or(2, indent);
    let last = (section.start..section.end)
        .rev()
        .find(|&i| !is_skippable(&lines[i]))
        .unwrap_or(section.start.saturating_sub(1));
    let pad = " ".repeat(child);
    let mut added = vec![format!("{pad}{name}:")];
    added.extend(fields.iter().map(|f| format!("{pad}  {f}")));
    lines.splice(last + 1..last + 1, added);
    Ok(lines.join("\n") + "\n")
}
