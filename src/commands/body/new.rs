//! `fusible body new`: scaffolds a code part and registers it in `fusible.yml`.

use std::path::Path;

use super::yaml_lines::{bodies_section, indent_of, is_skippable};
use super::BodyError;
use crate::config::FusibleConfig;

const TYPES: &str = include_str!("../../assets/part.d.ts");
const TEMPLATE: &str = r#"import type { Ctx } from "./fusible";

export default ({ board, cad }: Ctx) => {
  const outline: [number, number][] = board?.outline ?? [[0, 0], [60, 0], [60, 40], [0, 40]];
  const base = cad.shell(outline, 2, 18, 2);
  const posts = (board?.holes ?? []).map((h) => cad.boss(h.x, h.y, { insert: "m3", h: 5 }));
  return cad.union(base, ...posts);
};
"#;

fn refuse(name: &str, reason: impl Into<String>, help: impl Into<String>) -> BodyError {
    BodyError::CannotCreate {
        name: name.to_string(),
        reason: reason.into(),
        help: help.into(),
    }
}

fn check_name(cfg: &FusibleConfig, name: &str) -> Result<(), BodyError> {
    let plain = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !plain {
        return Err(refuse(
            name,
            "the name is not a plain YAML key",
            "use letters, digits, `-` and `_`",
        ));
    }
    let taken = cfg.boards.contains_key(name)
        || cfg.firmware.contains_key(name)
        || cfg.bodies.contains_key(name)
        || ["eject", "kit", "new"].contains(&name);
    match taken {
        true => Err(refuse(
            name,
            "the name is already used",
            "names are unique across boards, firmware and bodies",
        )),
        false => Ok(()),
    }
}

/// Returns `old` with the body `name` added under `bodies:`, creating the section when it is missing.
pub(super) fn add_entry(old: &str, name: &str, fields: &[String]) -> Result<String, String> {
    if old.contains('\r') {
        return Err("the file uses CRLF line endings".into());
    }
    let mut lines: Vec<String> = old.lines().map(str::to_string).collect();
    let Some(section) = bodies_section(&lines) else {
        if old
            .lines()
            .any(|l| l.starts_with("bodies:") || l.starts_with("mechanical:"))
        {
            return Err(
                "`bodies:` is not a block, or the legacy `mechanical:` section is used".into(),
            );
        }
        let mut out = old.trim_end().to_string();
        out.push_str(&format!("\n\nbodies:\n  {name}:\n"));
        out.extend(fields.iter().map(|f| format!("    {f}\n")));
        return Ok(out);
    };
    let child = lines[section.clone()]
        .iter()
        .find(|l| !is_skippable(l))
        .map_or(2, |l| indent_of(l));
    let last = (section.start..section.end)
        .rev()
        .find(|&i| !is_skippable(&lines[i]))
        .unwrap_or(section.start.saturating_sub(1));
    let at = last + 1;
    let pad = " ".repeat(child);
    let mut added = vec![format!("{pad}{name}:")];
    added.extend(fields.iter().map(|f| format!("{pad}  {f}")));
    lines.splice(at..at, added);
    Ok(lines.join("\n") + "\n")
}

fn write_files(root: &Path, name: &str) -> Result<(), BodyError> {
    let dir = root.join("mech");
    let part = dir.join(format!("{name}.ts"));
    if part.exists() {
        return Err(BodyError::Exists { path: part });
    }
    std::fs::create_dir_all(&dir)?;
    std::fs::write(&part, TEMPLATE)?;
    std::fs::write(dir.join("fusible.d.ts"), TYPES)?;
    println!("✅ Part -> {}", part.display());
    Ok(())
}

/// Writes `mech/<name>.ts`, `mech/fusible.d.ts` and a `bodies.<name>` entry next to the manifest.
///
/// The name must be new across boards, firmware and bodies. The manifest is edited as text so
/// its comments survive, and the result must parse with the new body in it.
pub fn new(
    name: String,
    board: Option<String>,
    material: String,
    process: String,
) -> Result<(), BodyError> {
    let (manifest, cfg) = FusibleConfig::nearest()
        .ok_or_else(|| refuse(&name, "no fusible.yml found", "run `fusible init` first"))?;
    check_name(&cfg, &name)?;
    crate::bom::validate_material(Some(&material), Some(&process))
        .map_err(|reason| refuse(&name, reason, "see src/bom/materials.yml for the library"))?;
    if board.as_ref().is_some_and(|b| !cfg.boards.contains_key(b)) {
        return Err(refuse(
            &name,
            "unknown board",
            "pick one of the boards of fusible.yml",
        ));
    }
    let mut fields = vec![
        format!("source: mech/{name}.ts"),
        format!("material: {material}"),
    ];
    fields.extend([format!("process: {process}"), "format: all".to_string()]);
    fields.extend(board.map(|b| format!("board: {b}")));
    let old = std::fs::read_to_string(&manifest)?;
    let edited = add_entry(&old, &name, &fields).map_err(|reason| {
        refuse(
            &name,
            reason,
            format!("add `{name}: {{source: mech/{name}.ts}}` under `bodies:` yourself"),
        )
    })?;
    let parsed = FusibleConfig::from_str(&edited).map_err(|e| {
        refuse(
            &name,
            format!("the edit does not parse: {e}"),
            "edit fusible.yml by hand",
        )
    })?;
    if !parsed.bodies.contains_key(&name) {
        return Err(refuse(
            &name,
            "the edit lost the body",
            "edit fusible.yml by hand",
        ));
    }
    let root = manifest.parent().unwrap_or(Path::new("."));
    write_files(root, &name)?;
    std::fs::write(&manifest, edited)?;
    println!("✅ Body {name} added to {}", manifest.display());
    Ok(())
}
