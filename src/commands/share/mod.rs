//! Subcommand: `fusible share nuage` — put the fuse-gated export in a synced Nuage space.

mod nuage;
mod revision;
mod stage;

use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

use crate::cli::{NuageArgs, ShareArgs, ShareTarget};
use crate::commands::check::CheckError;
use crate::commands::export::{self, ExportError};
use crate::commands::target::{resolve_board, TargetError};
use crate::config::FusibleConfig;

/// A share that could not complete.
#[derive(Debug, Error, Diagnostic)]
pub enum ShareError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Export(#[from] ExportError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Target(#[from] TargetError),

    #[error("{0}")]
    #[diagnostic(code(fusible::share::nuage))]
    Nuage(String),

    #[error("space \"{0}\" is not synced on this machine; run nuage to set a sync folder")]
    #[diagnostic(code(fusible::share::unsynced))]
    Unsynced(String),

    #[error("\"{}\" already holds files", .0.display())]
    #[diagnostic(code(fusible::share::exists), help("pass --force to overwrite it"))]
    Exists(PathBuf),

    #[error(
        "unsafe project name \"{0}\"; use a plain name without separators, \"..\" or a leading dot"
    )]
    #[diagnostic(code(fusible::share::name))]
    BadName(String),

    #[error("cannot copy the export: {0}")]
    #[diagnostic(code(fusible::share::io))]
    Io(#[from] std::io::Error),
}

impl ShareError {
    /// Whether the error is a blown fuse, which maps to exit code 1.
    pub fn is_fuse_blown(&self) -> bool {
        matches!(
            self,
            Self::Export(ExportError::Check(CheckError::FuseBlown(_)))
        )
    }
}

fn export_to_temp(preset: &str) -> Result<PathBuf, ShareError> {
    let board = resolve_board(None)?;
    let tmp = stage::unique_dir()?;
    export::run(
        Some(board.entry),
        Some(preset),
        "gerbers",
        Some(tmp.clone()),
        false,
    )?;
    Ok(tmp)
}

fn project_name() -> String {
    let (manifest, cfg) = FusibleConfig::nearest().unzip();
    let dir = manifest
        .as_deref()
        .and_then(Path::parent)
        .map(Path::to_path_buf);
    let folder = dir.or_else(|| std::env::current_dir().ok());
    let folder = folder.and_then(|d| d.file_name().map(|n| n.to_string_lossy().into_owned()));
    cfg.and_then(|c| c.name)
        .or(folder)
        .unwrap_or_else(|| "project".into())
}

/// Exports, copies into the space's sync folder, syncs, and optionally links.
pub fn run(args: ShareArgs) -> Result<(), ShareError> {
    let ShareTarget::Nuage(args) = args.target;
    share_nuage(&args)
}

fn share_nuage(args: &NuageArgs) -> Result<(), ShareError> {
    let sync_dir = nuage::sync_dir(&args.space)?;
    let project = project_name();
    stage::valid_name(&project)?;
    let tmp = export_to_temp(&args.preset)?;
    let rev = revision::current();
    let dest = stage::destination(&sync_dir, &project, &rev, args.force)?;
    let count = stage::copy_tree(&tmp, &dest);
    let _ = std::fs::remove_dir_all(&tmp);
    let count = count?;
    nuage::sync()?;
    if args.link {
        nuage::link(&format!("Fusible/{project}/{rev}"), &args.expires);
    }
    println!("Shared {count} file(s) to {}", dest.display());
    Ok(())
}
