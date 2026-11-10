//! The text rewrite behind `fusible migrate`: legacy sections nested under named ones, comments kept.

use std::ops::Range;

use serde_yaml::Value;

use crate::commands::body::yaml_lines::{block_end, indent_of, is_skippable, key_value};
use crate::config::{FirmwareConfig, LEGACY_BOARD};

const LEGACY_FIRMWARE: &str = "firmware";
const LEGACY_BODY: &str = "enclosure";
const LEGACY_BODY_DIR: &str = "dist/mechanical";

type Edit = (Range<usize>, Vec<String>);

/// Returns `text` in the 2.0 form: named `boards:`, `firmware:` and `bodies:`, and new rule ids.
///
/// Lines outside the legacy sections, comments included, are kept as they are.
pub(super) fn migrate(text: &str) -> Result<String, String> {
    let value: Value = serde_yaml::from_str(text).map_err(|e| e.to_string())?;
    let lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut edits = boards(&lines, &value)?;
    edits.extend(firmware(&lines, &value)?);
    edits.extend(bodies(&lines, &value)?);
    edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    let mut out = lines;
    for (range, replacement) in edits {
        out.splice(range, replacement);
    }
    let mut migrated = out.join("\n");
    if text.ends_with('\n') {
        migrated.push('\n');
    }
    Ok(super::rule_ids::rename(&migrated))
}

fn top_block(lines: &[String], key: &str) -> Option<Range<usize>> {
    let start = lines
        .iter()
        .position(|l| indent_of(l) == 0 && key_value(l).is_some_and(|(k, _)| k == key))?;
    Some(start..block_end(lines, start, 0, lines.len()))
}

fn present(value: &Value, key: &str) -> bool {
    value.get(key).is_some_and(|v| !v.is_null())
}

/// The block at `range` moved under `header:` as the entry `name:`, with `extra` fields first.
fn nest(
    lines: &[String],
    range: Range<usize>,
    header: &str,
    name: &str,
    extra: &[String],
) -> Result<Vec<String>, String> {
    let (key, rest) = key_value(&lines[range.start]).unwrap_or_default();
    if !extra.is_empty() && !rest.is_empty() && !rest.starts_with('#') {
        return Err(format!(
            "`{key}:` is written inline; move it under `{header}:` by hand"
        ));
    }
    let children = &lines[range.start + 1..range.end];
    let pad = children
        .iter()
        .find(|l| !is_skippable(l))
        .map_or(4, |l| indent_of(l) + 2);
    let entry = match rest.is_empty() {
        true => format!("  {name}:"),
        false => format!("  {name}: {rest}"),
    };
    let mut out = vec![format!("{header}:"), entry];
    out.extend(
        extra
            .iter()
            .map(|field| format!("{}{field}", " ".repeat(pad))),
    );
    out.extend(children.iter().map(|l| match l.trim().is_empty() {
        true => String::new(),
        false => format!("  {l}"),
    }));
    Ok(out)
}

fn boards(lines: &[String], value: &Value) -> Result<Vec<Edit>, String> {
    let chip = value["mcu"]["chip"].as_str().map(scalar);
    let mcu = top_block(lines, "mcu").map(|range| (range, Vec::new()));
    let Some(hardware) = top_block(lines, "hardware").filter(|_| present(value, "hardware")) else {
        let entry = match &chip {
            Some(chip) => format!("  {LEGACY_BOARD}:\n    mcu: {chip}"),
            None => format!("  {LEGACY_BOARD}: {{}}"),
        };
        let only_mcu = mcu.filter(|_| present(value, "mcu"));
        let edit = only_mcu.map(|(range, _)| (range, vec!["boards:".into(), entry]));
        return Ok(edit.into_iter().collect());
    };
    let own_mcu = !value["hardware"]["mcu"].is_null();
    let extra: Vec<String> = chip
        .filter(|_| !own_mcu)
        .map(|c| format!("mcu: {c}"))
        .into_iter()
        .collect();
    let nested = nest(lines, hardware.clone(), "boards", LEGACY_BOARD, &extra)?;
    let blank_after = |end: usize| lines.get(end).is_some_and(|l| l.trim().is_empty());
    let removed = mcu.map(|(r, gone)| (r.start..r.end + usize::from(blank_after(r.end)), gone));
    Ok(std::iter::once((hardware, nested)).chain(removed).collect())
}

fn firmware(lines: &[String], value: &Value) -> Result<Vec<Edit>, String> {
    let single = value
        .get("firmware")
        .filter(|v| v.is_mapping())
        .is_some_and(|v| serde_yaml::from_value::<FirmwareConfig>(v.clone()).is_ok());
    match top_block(lines, "firmware").filter(|_| single) {
        Some(range) => Ok(vec![(
            range.clone(),
            nest(lines, range, "firmware", LEGACY_FIRMWARE, &[])?,
        )]),
        None => Ok(Vec::new()),
    }
}

fn bodies(lines: &[String], value: &Value) -> Result<Vec<Edit>, String> {
    let Some(range) = top_block(lines, "mechanical").filter(|_| present(value, "mechanical"))
    else {
        return Ok(Vec::new());
    };
    let project = value["name"].as_str();
    let name = project
        .filter(|n| ![LEGACY_BOARD, LEGACY_FIRMWARE].contains(n))
        .unwrap_or(LEGACY_BODY);
    let mechanical = &value["mechanical"];
    let mut extra = vec!["generate: enclosure".to_string()];
    if mechanical["out_dir"].is_null() && mechanical["out-dir"].is_null() {
        extra.push(format!("out_dir: {LEGACY_BODY_DIR}"));
    }
    Ok(vec![(
        range.clone(),
        nest(lines, range, "bodies", &scalar(name), &extra)?,
    )])
}

/// `text` as a plain YAML scalar when it is safe to write bare, else double-quoted.
fn scalar(text: &str) -> String {
    let plain = !text.is_empty()
        && text
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric())
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c));
    match plain {
        true => text.to_string(),
        false => format!("{text:?}"),
    }
}
