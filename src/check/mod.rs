//! The check pipeline shared by `fusible check`, `--json`, the studio and MCP.
//!
//! [`run`] parses every board once, lints it, then runs the cross-world checks: links once
//! every board was checked,
//! assembly and bodies once every board and link passed. It returns a [`Report`] and
//! prints nothing; callers render it. A tool error is an `Err`, never a blown fuse.

mod board;
mod cross;
mod error;
mod report;

pub use board::{check_entry, check_source, Checked};
pub use error::{CheckError, ReadError};
pub use report::{BoardReport, Fuse, Report, REPORT_VERSION};

use crate::commands::source::load_circuit_source;
use crate::commands::target::{resolve_boards, BoardTarget};
use crate::project::Project;
use board::Input;

/// Which boards one check run covers.
pub enum Scope {
    /// Every board of the manifest, or the conventional entry without one.
    All,
    /// These boards, each with the MCU it targets already set.
    Boards(Vec<BoardTarget>),
}

/// The name a board goes by in the report: its manifest name, else its entry file stem.
pub fn board_name(target: &BoardTarget) -> String {
    let stem = target
        .entry
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned());
    target.name.clone().or(stem).unwrap_or(target.title.clone())
}

fn check_boards(
    project: &Project,
    boards: &[BoardTarget],
    report: &mut Report,
) -> Result<(), CheckError> {
    for target in boards {
        let (json_path, source) = load_circuit_source(&target.entry)?;
        let name = board_name(target);
        let input = Input {
            mcu: target.mcu.as_deref(),
            json_path: &json_path,
            source: &source,
        };
        let checked = board::check_with(project, input)?;
        let diagnostics = checked.diagnostics;
        report.boards.push(BoardReport { name, diagnostics });
    }
    Ok(())
}

/// Checks the boards of `scope`, then the links and the assembly, into one report.
pub fn run(project: &Project, scope: Scope) -> Result<Report, CheckError> {
    let boards = match scope {
        Scope::All => resolve_boards(project, None)?,
        Scope::Boards(boards) => boards,
    };
    let mut report = Report::default();
    check_boards(project, &boards, &mut report)?;
    let complete = cross::every_board(project, &boards);
    report.links = cross::links(project, complete)?;
    if complete && !report.has_errors() {
        report.assembly = cross::assembly(project, complete)?;
    }
    report.seal();
    Ok(report)
}
