//! Subcommand: `fusible release` — fuse-gated fabrication bundle with a checksum manifest.

mod manifest;
mod pours;

use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

use crate::commands::check::{self, load_circuit_source, CheckError};
use crate::commands::export::{write_fab_bundle, ExportError};
use crate::commands::target::{resolve_boards, BoardTarget};
use crate::gerber::generate_gerber_files;
use crate::linter::{LintContext, Linter, Severity};
use crate::netlist::Netlist;
use crate::parser;
use crate::project::Project;

/// A release that could not be produced.
#[derive(Debug, Error, Diagnostic)]
pub enum ReleaseError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Check(#[from] CheckError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Export(#[from] ExportError),

    #[error("board \"{board}\" has {count} warning(s)")]
    #[diagnostic(
        code(fusible::release::warnings),
        help("fix the warnings, or pass --allow-warnings to release anyway")
    )]
    Warnings { board: String, count: usize },

    #[error("board \"{board}\": layer {layer} has a copper pour but its gerber has no G36 region")]
    #[diagnostic(code(fusible::release::missing_pour))]
    MissingPour { board: String, layer: String },

    #[error("circuit JSON is unreadable: {0}")]
    #[diagnostic(code(fusible::release::unreadable))]
    Unreadable(String),

    #[error("fabrication files are invalid: {0}")]
    #[diagnostic(code(fusible::release::gerber))]
    Gerber(#[from] crate::gerber::GerberError),

    #[error("cannot write the release: {0}")]
    #[diagnostic(code(fusible::release::io))]
    Io(#[from] std::io::Error),
}

/// One board that passed every check, with its fabrication files generated in memory.
struct Staged {
    name: String,
    title: String,
    circuit_json: String,
}

/// Checks every board, then writes `gerbers.zip`, `bom.csv`, `centroid.csv`, `netlist.ipc`,
/// `README.txt` and `RELEASE.txt` into `out`.
/// Nothing is written unless every board passes every check.
pub fn run(
    project: &Project,
    file: Option<PathBuf>,
    out: &Path,
    allow_warnings: bool,
) -> Result<(), ReleaseError> {
    let boards = resolve_boards(project, file).map_err(CheckError::from)?;
    check::run_boards(project, &boards)?;
    let staged = boards
        .iter()
        .map(|board| stage(project, board, allow_warnings))
        .collect::<Result<Vec<_>, _>>()?;
    for board in &staged {
        let dir = if staged.len() > 1 {
            out.join(&board.name)
        } else {
            out.to_path_buf()
        };
        let manifest = project.manifest.as_deref();
        write_fab_bundle(&board.circuit_json, &board.title, &dir, manifest)?;
    }
    let names: Vec<&str> = staged.iter().map(|b| b.name.as_str()).collect();
    manifest::write(out, &names)?;
    println!(
        "✅ Release written to \"{}\" ({} board(s))",
        out.display(),
        staged.len()
    );
    Ok(())
}

fn stage(
    project: &Project,
    board: &BoardTarget,
    allow_warnings: bool,
) -> Result<Staged, ReleaseError> {
    let name = board.name.clone().unwrap_or_else(|| board.title.clone());
    let (path, source) = load_circuit_source(&board.entry)?;
    let count = warning_count(project, &path, &source, board.mcu.as_deref())?;
    if count > 0 && !allow_warnings {
        return Err(ReleaseError::Warnings { board: name, count });
    }
    let files = generate_gerber_files(&source, &board.title)?;
    pours::verify(&name, &source, &files)?;
    Ok(Staged {
        name,
        title: board.title.clone(),
        circuit_json: source,
    })
}

fn warning_count(
    project: &Project,
    path: &Path,
    source: &str,
    mcu: Option<&str>,
) -> Result<usize, CheckError> {
    let circuit = parser::parse(source, path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let ctx = LintContext::new(&circuit, &netlist)
        .with_mcu(mcu)
        .with_design(project.design_rules());
    let found = Linter::default().run_with_context(&ctx);
    Ok(found
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count())
}
