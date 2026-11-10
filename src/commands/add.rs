//! Subcommand: `fusible add` — install tscircuit packages or modules into the project.

use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

use crate::project::Project;
use crate::tscircuit::error::RunnerError;
use crate::tscircuit::Runner;

/// Error raised when adding packages fails.
#[derive(Debug, Error, Diagnostic)]
pub enum AddError {
    #[error("failed to detect tscircuit runner: {0}")]
    #[diagnostic(help("run 'fusible doctor --install' to set up a runner"))]
    Runner(#[from] RunnerError),

    #[error("failed to locate hardware directory or package.json: {0}")]
    Io(#[from] std::io::Error),
}

/// Discovers the hardware workspace directory.
pub fn find_hardware_dir(project: &Project) -> PathBuf {
    if let Some(dir) = manifest_hardware_dir(project) {
        return dir;
    }

    if Path::new("hardware/package.json").exists() || Path::new("hardware").is_dir() {
        return PathBuf::from("hardware");
    }

    PathBuf::from(".")
}

/// Returns the existing parent directory of the manifest's hardware entry, if any.
fn manifest_hardware_dir(project: &Project) -> Option<PathBuf> {
    let (_, config) = project.found()?;
    let (_, board) = config
        .boards
        .iter()
        .next()
        .filter(|_| config.boards.len() == 1)?;
    let parent = board.entry.as_ref()?.parent()?.to_path_buf();
    parent.exists().then_some(parent)
}

/// Installs the given packages into the project's hardware workspace.
pub fn run(project: &Project, packages: &[String]) -> Result<(), AddError> {
    let hw_dir = find_hardware_dir(project);
    let runner = Runner::detect()?;

    println!(
        "📦 Adding {} package(s) via {} in \"{}\"...",
        packages.len(),
        runner.kind().description(),
        hw_dir.display()
    );

    runner.add(packages, Some(&hw_dir))?;

    println!("✅ Successfully added package(s): {}", packages.join(", "));
    Ok(())
}
