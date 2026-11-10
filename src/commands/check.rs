//! Subcommand: `fusible check` / `lint` — run the electrical linter and render its report.

use std::path::{Path, PathBuf};

use crate::check::{self, Report, Scope};
pub use crate::check::{CheckError, ReadError};
use crate::commands::bindings;
pub use crate::commands::source::{find_fallback_circuit_json, load_circuit_source};
use crate::commands::target::{resolve_boards, BoardTarget};
use crate::linter::{Diagnostic, FuseBlown, Severity};
use crate::project::Project;

/// Loads, lints and renders the circuit at `entry` without writing any file: the fuse gate
/// of exports, bodies and assemblies.
///
/// Returns the path and text of the Circuit JSON that passed the fuse check.
pub fn lint_source(
    project: &Project,
    entry: &Path,
    target_mcu: Option<String>,
) -> Result<(PathBuf, String), CheckError> {
    let (json_path, source, checked) = check::check_entry(project, entry, target_mcu.as_deref())?;
    let errors = print_warnings(checked.diagnostics);
    if !errors.is_empty() {
        return Err(FuseBlown::new(errors).into());
    }
    print_intact(target_mcu.as_deref());
    Ok((json_path, source))
}

/// Runs the electrical linter on the target board, or on every board of the manifest.
///
/// `mcu` overrides the manifest MCU of each board. Once the fuse is intact, pin bindings
/// are written to `pins_rs`, else to the manifest's firmware paths.
pub fn run(
    project: &Project,
    file: Option<PathBuf>,
    mcu: Option<String>,
    pins_rs: Option<PathBuf>,
) -> Result<(), CheckError> {
    let boards = with_mcu(resolve_boards(project, file)?, mcu);
    run_boards(project, &boards)?;
    bindings::write(&boards, pins_rs)
}

/// Sets `mcu`, when given, as the target of every board.
pub fn with_mcu(boards: Vec<BoardTarget>, mcu: Option<String>) -> Vec<BoardTarget> {
    let set = |board: BoardTarget| BoardTarget {
        mcu: mcu.clone().or(board.mcu.clone()),
        ..board
    };
    boards.into_iter().map(set).collect()
}

/// Checks `boards` and renders the report; a blown fuse is `Err(FuseBlown)` with every error.
pub fn run_boards(project: &Project, boards: &[BoardTarget]) -> Result<(), CheckError> {
    let report = check::run(project, Scope::Boards(boards.to_vec()))?;
    render(report, boards)
}

fn print_warnings(diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let (warnings, errors): (Vec<_>, Vec<_>) = diagnostics
        .into_iter()
        .partition(|d| d.severity != Severity::Error);
    for warning in warnings {
        eprintln!("{:?}", miette::Report::new(warning));
    }
    errors
}

fn print_intact(mcu: Option<&str>) {
    println!("✅ Circuit validated (fuse intact)");
    if let Some(mcu) = mcu {
        println!("   silicon target: {mcu}");
    }
}

/// Prints each board's warnings and verdict, then returns every error as one blown fuse.
///
/// With several boards, each error message is prefixed with its board name.
fn render(mut report: Report, boards: &[BoardTarget]) -> Result<(), CheckError> {
    let several = report.boards.len() > 1;
    let mut errors = Vec::new();
    for (board, target) in std::mem::take(&mut report.boards).into_iter().zip(boards) {
        if several {
            println!("\n▸ board {}", board.name);
        }
        let found = print_warnings(board.diagnostics);
        if found.is_empty() {
            print_intact(target.mcu.as_deref());
        }
        let label = |mut d: Diagnostic| {
            d.message = format!("[{}] {}", board.name, d.message);
            d
        };
        errors.extend(
            found
                .into_iter()
                .map(|d| if several { label(d) } else { d }),
        );
    }
    errors.extend(print_warnings(std::mem::take(&mut report.links)));
    errors.extend(print_warnings(std::mem::take(&mut report.assembly)));
    match errors.is_empty() {
        true => Ok(()),
        false => Err(FuseBlown::new(errors).into()),
    }
}
