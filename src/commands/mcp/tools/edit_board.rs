//! Tool `edit_board`: change a board source file, checked and diffed.

use serde_json::{json, Value};

use super::args::{boolean, flag, object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::edit;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str =
    "Edit the source file of a board with a unified diff (patch) or full new content, \
then check it. dry_run defaults to true: the file is restored and you get the diff and the fuse. \
Nothing is committed.";

/// Input schema: board, patch or content, dry_run.
pub fn schema() -> Value {
    object(
        &[
            ("board", string("Board name")),
            (
                "patch",
                string(
                    "Unified diff against the project root; paths must name the board file only",
                ),
            ),
            ("content", string("Full new content of the board file")),
            (
                "dry_run",
                boolean("Preview without keeping the change (default true)"),
            ),
        ],
        &["board"],
    )
}

fn prepare(session: &mut Session, args: &Value) -> Result<Outcome, Outcome> {
    let board = required_str(args, "board")?;
    let project = session.project();
    let target = report::select(&project, Some(board))
        .map_err(Outcome::fail)?
        .remove(0);
    let change = edit::change_from(args).map_err(Outcome::fail)?;
    let check = || {
        let lint = report::lint_board(&project, &target);
        (
            json!(report::fuse(std::slice::from_ref(&lint))),
            report::diagnostics_json(&lint),
        )
    };
    Ok(edit::edit(
        session,
        &target.entry,
        &change,
        flag(args, "dry_run", true),
        &check,
    ))
}

/// Applies the change under the write-safety rules.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    prepare(session, args).unwrap_or_else(|outcome| outcome)
}
