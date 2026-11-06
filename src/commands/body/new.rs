//! `fusible body new`: scaffolds a code part and registers it in `fusible.yml`.

use std::path::Path;

use super::new_manifest::add_entry;
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
        let help = "use letters, digits, `-` and `_`";
        return Err(refuse(name, "the name is not a plain YAML key", help));
    }
    let taken = cfg.boards.contains_key(name)
        || cfg.firmware.contains_key(name)
        || cfg.bodies.contains_key(name)
        || ["eject", "kit", "new"].contains(&name);
    let help = "names are unique across boards, firmware and bodies";
    match taken {
        true => Err(refuse(name, "the name is already used", help)),
        false => Ok(()),
    }
}

fn check_choices(
    cfg: &FusibleConfig,
    name: &str,
    board: Option<&String>,
    made: (&str, &str),
) -> Result<(), BodyError> {
    crate::bom::validate_material(Some(made.0), Some(made.1))
        .map_err(|reason| refuse(name, reason, "see src/bom/materials.yml for the library"))?;
    match board.is_some_and(|b| !cfg.boards.contains_key(b)) {
        true => Err(refuse(name, "unknown board", "pick a board of fusible.yml")),
        false => Ok(()),
    }
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

fn edited_manifest(old: &str, name: &str, fields: &[String]) -> Result<String, BodyError> {
    let by_hand = format!("add `{name}: {{source: mech/{name}.ts}}` under `bodies:` yourself");
    let edited = add_entry(old, name, fields).map_err(|reason| refuse(name, reason, by_hand))?;
    let parsed = FusibleConfig::from_str(&edited).map_err(|e| {
        refuse(
            name,
            format!("the edit does not parse: {e}"),
            "edit fusible.yml by hand",
        )
    })?;
    match parsed.bodies.contains_key(name) {
        true => Ok(edited),
        false => Err(refuse(
            name,
            "the edit lost the body",
            "edit fusible.yml by hand",
        )),
    }
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
    check_choices(&cfg, &name, board.as_ref(), (&material, &process))?;
    let mut fields = vec![format!("source: mech/{name}.ts")];
    fields.extend([
        format!("material: {material}"),
        format!("process: {process}"),
    ]);
    fields.extend(["format: all".to_string()]);
    fields.extend(board.map(|b| format!("board: {b}")));
    let edited = edited_manifest(&std::fs::read_to_string(&manifest)?, &name, &fields)?;
    write_files(manifest.parent().unwrap_or(Path::new(".")), &name)?;
    std::fs::write(&manifest, edited)?;
    println!("✅ Body {name} added to {}", manifest.display());
    Ok(())
}
