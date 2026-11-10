//! Best-effort Opus tasks for blown fuses and unbought parts.
//!
//! This module only creates tasks. It never updates, completes or deletes one, and it never
//! changes the verdict of the command that called it.

mod dedup;
mod task;

use std::path::{Path, PathBuf};

use crate::bom::{Line, Report};
use crate::cli::OpusArgs;
use crate::commands::mcp::report;
use crate::commands::target::resolve_boards;
use crate::linter::{Diagnostic, Severity};
use crate::project::Project;
use dedup::{fuse_key, Store};
use task::{clean_title, create};

const MISSING: &str = "opus CLI not found; install it with: facile install opus";

fn on_path() -> bool {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path).any(|dir| dir.join("opus").is_file())
}

fn project_root(project: &Project) -> PathBuf {
    project
        .manifest_dir()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn fuse_args(board: &str, d: &Diagnostic) -> Vec<String> {
    let help = d.help.clone().unwrap_or_default();
    let description = format!("{}\n{help}", d.rule);
    let title = clean_title(&format!("{board}: {}", d.message));
    let labels = ["fusible", "fuse"]
        .into_iter()
        .flat_map(|l| ["--label".to_string(), l.to_string()]);
    let flags = [
        "--description".to_string(),
        description,
        "--priority".into(),
        "high".into(),
    ];
    std::iter::once(title).chain(flags).chain(labels).collect()
}

/// Opens a task for each new blown-fuse error of the boards `file` selects.
pub fn report_check(project: &Project, file: Option<PathBuf>, opus: &OpusArgs) {
    if !on_path() {
        return eprintln!("{MISSING}");
    }
    let Ok(boards) = resolve_boards(project, file) else {
        return;
    };
    let mut store = Store::load(&project_root(project));
    for board in boards.iter().map(|b| report::lint_board(project, b)) {
        for d in board
            .diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
        {
            let key = fuse_key(&d.rule, std::slice::from_ref(&d.message), &board.name);
            create(&mut store, key, fuse_args(&board.name, d), opus);
        }
    }
}

fn bom_task(line: &Line) -> (String, Vec<String>) {
    let designators = line.designators.join(", ");
    let value = line.value.as_ref().unwrap_or(&line.description);
    let title = clean_title(&format!("Buy {} × {value} ({designators})", line.quantity));
    let id = if designators.is_empty() {
        &line.id
    } else {
        &designators
    };
    let labels = ["fusible", "bom"]
        .into_iter()
        .flat_map(|l| ["--label".to_string(), l.to_string()]);
    (
        format!("bom:{id}"),
        std::iter::once(title).chain(labels).collect(),
    )
}

/// Opens a "Buy" task for each BOM row with no price or not picked.
pub fn report_bom(project: &Project, report: &Report, opus: &OpusArgs) {
    if !on_path() {
        return eprintln!("{MISSING}");
    }
    let mut store = Store::load(&project_root(project));
    for line in report
        .lines
        .iter()
        .filter(|l| l.unit_price.is_none() || !l.picked)
    {
        let (key, args) = bom_task(line);
        create(&mut store, key, args, opus);
    }
}
