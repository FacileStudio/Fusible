//! Subcommand: `fusible migrate` — rewrite a 1.x `fusible.yml` into the 2.0 form.
//!
//! The legacy single-board sections (`hardware:`, `mcu:`, one unnamed `firmware:`,
//! `mechanical:`) move under named `boards:`, `firmware:` and `bodies:` entries, and renamed
//! rule ids take their new names. The rewrite works on the text, not on a serde round trip:
//! `serde_yaml` drops comments and reorders keys, so every line outside a legacy section is
//! kept byte for byte and moved lines keep their comments. The result must load to exactly
//! the configuration the old file loaded to, or nothing is written.

mod diff;
mod rewrite;

use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

use crate::config::{FusibleConfig, CONFIG_FILENAMES};

/// A migration that could not run; every variant is a tool error (exit 2).
#[derive(Debug, Error, Diagnostic)]
pub enum MigrateError {
    #[error("no fusible.yml found in this directory or its parents")]
    #[diagnostic(
        code(fusible::migrate::missing),
        help("run `fusible init` to create one")
    )]
    Missing,

    #[error("cannot read or write \"{}\"", path.display())]
    #[diagnostic(code(fusible::migrate::io))]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("cannot migrate \"{}\": {reason}", path.display())]
    #[diagnostic(
        code(fusible::migrate::unsafe_rewrite),
        help("fusible.yml was left untouched; edit it by hand")
    )]
    Rewrite { path: PathBuf, reason: String },
}

/// What a migration found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Migration {
    /// The manifest is already in the 2.0 form.
    Current,
    /// The manifest needs a rewrite; `--check` wrote nothing.
    Needed,
    /// The manifest was rewritten.
    Written,
}

/// Rewrites the nearest manifest, printing the diff; with `check`, writes nothing.
pub fn run(check: bool) -> Result<Migration, MigrateError> {
    let path = locate().ok_or(MigrateError::Missing)?;
    let io = |source| MigrateError::Io {
        path: path.clone(),
        source,
    };
    let old = std::fs::read_to_string(&path).map_err(io)?;
    let new = migrated(&path, &old)?;
    if new == old {
        println!("✅ {} is already in the 2.0 form", path.display());
        return Ok(Migration::Current);
    }
    let label = path
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    print!("{}", diff::unified(&old, &new, &label));
    if check {
        return Ok(Migration::Needed);
    }
    std::fs::write(&path, &new).map_err(io)?;
    println!("✅ Migrated {}", path.display());
    Ok(Migration::Written)
}

fn locate() -> Option<PathBuf> {
    let cwd = std::path::absolute(".").ok()?;
    cwd.ancestors()
        .flat_map(|dir| CONFIG_FILENAMES.iter().map(move |name| dir.join(name)))
        .find(|candidate| candidate.is_file())
}

/// The migrated text of `old`, proven to load to the same configuration.
fn migrated(path: &Path, old: &str) -> Result<String, MigrateError> {
    let fail = |reason: String| MigrateError::Rewrite {
        path: path.to_path_buf(),
        reason,
    };
    let crlf = old.contains("\r\n");
    let text = old.replace("\r\n", "\n");
    let renamed = rewrite::RENAMED_RULES
        .iter()
        .fold(text.clone(), |acc, (from, to)| acc.replace(from, to));
    let expected = FusibleConfig::from_str(&renamed).map_err(|e| fail(e.to_string()))?;
    let new = rewrite::migrate(&text).map_err(fail)?;
    let loaded = FusibleConfig::from_str(&new).map_err(|e| fail(e.to_string()))?;
    if loaded != expected {
        return Err(fail(
            "the rewrite would change what the manifest means".into(),
        ));
    }
    Ok(match crlf {
        true => new.replace('\n', "\r\n"),
        false => new,
    })
}
