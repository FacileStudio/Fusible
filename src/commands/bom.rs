//! Subcommand: `fusible bom` — list the parts of the project with quantities and prices.

use std::path::Path;

use miette::Diagnostic;
use thiserror::Error;

use crate::bom::{self, BomError, BomFile, Part, Report};
use crate::cli::{BomArgs, BomFormat};
use crate::commands::check::{load_circuit_source, CheckError};
use crate::commands::target::{resolve_boards, TargetError};
use crate::project::Project;

/// A BOM that could not be produced.
#[derive(Debug, Error, Diagnostic)]
pub enum BomCommandError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Target(#[from] TargetError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Source(#[from] CheckError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Bom(#[from] BomError),

    #[error("cannot write \"{path}\"")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub(crate) fn report(project: &Project, args: &BomArgs) -> Result<Report, BomCommandError> {
    let boards = resolve_boards(project, args.target.clone())?;
    let scoped = boards.len() > 1;
    let mut parts: Vec<Part> = Vec::new();
    for board in &boards {
        let (_, source) = load_circuit_source(&board.entry)?;
        let scope = board.name.as_deref().filter(|_| scoped);
        parts.extend(bom::detect(&source, scope)?);
    }
    let names: Vec<&str> = boards.iter().filter_map(|b| b.name.as_deref()).collect();
    let file = match &project.manifest {
        Some(manifest) => BomFile::load_or_create(&bom::bom_path(manifest))?,
        None => BomFile::default(),
    };
    let root = project.manifest_dir().unwrap_or(Path::new("."));
    let bodies = project
        .found()
        .map_or_else(Vec::new, |(_, c)| bom::bodies_of(c, &names, root));
    Ok(bom::build(parts, &bodies, &file))
}

/// Prints or writes the bill of materials.
pub fn run(project: &Project, args: BomArgs) -> Result<(), BomCommandError> {
    let report = report(project, &args)?;
    if args.opus.opus {
        crate::commands::opus::report_bom(project, &report, &args.opus);
    }
    let text = match args.format {
        BomFormat::Table => bom::table(&report),
        BomFormat::Csv => bom::csv(&report),
        BomFormat::Json => serde_json::to_string_pretty(&report).unwrap_or_default() + "\n",
    };
    match args.output {
        Some(path) => std::fs::write(&path, text).map_err(|source| BomCommandError::Write {
            path: path.display().to_string(),
            source,
        }),
        None => {
            print!("{text}");
            Ok(())
        }
    }
}
