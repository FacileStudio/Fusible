//! The check pipeline shared by `fusible check`, `--json`, the studio and MCP.
//!
//! [`run`] parses every board once, lints it, applies the manifest's `rules:` levels and
//! `waivers:`, then runs the cross-world checks: links once every board was checked,
//! assembly and bodies once every board and link passed. It returns a [`Report`] and
//! prints nothing; callers render it. A tool error is an `Err`, never a blown fuse.

mod board;
mod cross;
mod error;
mod levels;
mod report;
#[cfg(test)]
mod tests;
mod waive;

pub use board::{check_entry, check_source, Checked};
pub use error::{CheckError, ReadError};
pub use levels::known_rule_ids;
pub use report::{BoardReport, Fuse, Report, WaivedDiagnostic, WaiverEntry, REPORT_VERSION};

use crate::commands::source::load_circuit_source;
use crate::commands::target::{resolve_boards, BoardTarget};
use crate::config::Date;
use crate::project::Project;
use board::Input;
use levels::Levels;

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
    levels: &mut Levels<'_>,
    boards: &[BoardTarget],
    report: &mut Report,
) -> Result<(), CheckError> {
    for target in boards {
        let (json_path, source) = load_circuit_source(&target.entry)?;
        let name = board_name(target);
        let input = Input {
            name: Some(&name),
            mcu: target.mcu.as_deref(),
            json_path: &json_path,
            source: &source,
        };
        let checked = board::check_with(project, levels, input)?;
        report.waived.extend(checked.waived);
        let diagnostics = checked.diagnostics;
        report.boards.push(BoardReport { name, diagnostics });
    }
    Ok(())
}

/// Checks the boards of `scope`, then the links and the assembly, into one report.
///
/// Unused waivers are reported only when `scope` covers every board of the manifest.
pub fn run(project: &Project, scope: Scope) -> Result<Report, CheckError> {
    let boards = match scope {
        Scope::All => resolve_boards(project, None)?,
        Scope::Boards(boards) => boards,
    };
    let mut levels = Levels::new(&project.config, Date::today())?;
    let mut report = Report::default();
    check_boards(project, &mut levels, &boards, &mut report)?;
    let complete = cross::every_board(project, &boards);
    let links = cross::links(project, complete)?;
    report.links = levels.apply(links, None, None, &mut report.waived);
    let assembly_ran = complete && !report.has_errors();
    if assembly_ran {
        let assembly = cross::assembly(project, complete)?;
        report.assembly = levels.apply(assembly, None, None, &mut report.waived);
    }
    if complete {
        report.unused_waivers = levels.unused(assembly_ran);
    }
    report.expired_waivers = levels.expired();
    report.seal();
    Ok(report)
}
