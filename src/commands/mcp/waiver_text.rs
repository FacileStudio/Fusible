//! Appending one waiver to the text of `fusible.yml`, every other line and comment kept.

use crate::commands::body::yaml_lines::{indent_of, is_skippable, key_value};

/// The waiver to write.
pub struct NewWaiver<'a> {
    pub rule: &'a str,
    pub matches: &'a str,
    pub reason: &'a str,
    pub until: Option<&'a str>,
}

fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}

fn entry(waiver: &NewWaiver, indent: usize) -> Vec<String> {
    let pad = " ".repeat(indent);
    let mut lines = vec![
        format!("{pad}- rule: {}", quoted(waiver.rule)),
        format!("{pad}  match: {}", quoted(waiver.matches)),
        format!("{pad}  reason: {}", quoted(waiver.reason)),
    ];
    lines.extend(
        waiver
            .until
            .map(|day| format!("{pad}  until: {}", quoted(day))),
    );
    lines
}

fn in_block(line: &str) -> bool {
    is_skippable(line) || indent_of(line) > 0 || line.starts_with('-')
}

/// The end (exclusive) of the `waivers:` block at `header`, trailing blank lines left out.
fn block_end(lines: &[&str], header: usize) -> usize {
    let mut end = header + 1;
    for (i, line) in lines.iter().enumerate().skip(header + 1) {
        if !in_block(line) {
            break;
        }
        if !is_skippable(line) {
            end = i + 1;
        }
    }
    end
}

/// Returns `text` with `waiver` appended to its `waivers:` list, or a new list at the end.
///
/// Fails when `waivers:` is written inline (`waivers: []`): that form is left to a human.
pub fn append(text: &str, waiver: &NewWaiver) -> Result<String, String> {
    let lines: Vec<&str> = text.lines().collect();
    let header = lines
        .iter()
        .position(|l| indent_of(l) == 0 && key_value(l).is_some_and(|(k, _)| k == "waivers"));
    let Some(header) = header else {
        let mut out = text.trim_end_matches('\n').to_string();
        out.push_str("\n\nwaivers:\n");
        out.push_str(&entry(waiver, 2).join("\n"));
        out.push('\n');
        return Ok(out);
    };
    let inline =
        key_value(lines[header]).is_some_and(|(_, v)| !v.is_empty() && !v.starts_with('#'));
    if inline {
        return Err("`waivers:` is written inline; add the waiver by hand".into());
    }
    let end = block_end(&lines, header);
    let indent = lines[header + 1..end]
        .iter()
        .find(|l| l.trim_start().starts_with('-'))
        .map_or(2, |l| indent_of(l));
    let mut out: Vec<String> = lines.iter().map(|l| (*l).to_string()).collect();
    out.splice(end..end, entry(waiver, indent));
    Ok(out.join("\n") + "\n")
}
