//! Quiet linting of boards: diagnostics as data, nothing printed to stdout.

pub use super::diag_json::{diagnostics_json, violations_json};
use crate::commands::check::load_circuit_source;
use crate::commands::target::{resolve_boards, BoardTarget};
use crate::linter::{Diagnostic, LintContext, Linter, Severity};
use crate::netlist::Netlist;
use crate::parser::{self, Circuit};

/// The lint result of one board.
pub struct BoardReport {
    pub name: String,
    pub blown: bool,
    pub diagnostics: Vec<Diagnostic>,
}

/// The name a board goes by in tool output.
pub fn board_name(target: &BoardTarget) -> String {
    target.name.clone().unwrap_or_else(|| target.title.clone())
}

/// Resolves the named board, or every board of the project when `name` is `None`.
pub fn select(name: Option<&str>) -> Result<Vec<BoardTarget>, String> {
    let boards = resolve_boards(None).map_err(|e| e.to_string())?;
    let Some(name) = name else {
        return Ok(boards);
    };
    let known: Vec<String> = boards.iter().map(board_name).collect();
    let found: Vec<BoardTarget> = boards
        .into_iter()
        .filter(|b| board_name(b) == name)
        .collect();
    if found.is_empty() {
        return Err(format!(
            "unknown board \"{name}\"; boards: {}",
            known.join(", ")
        ));
    }
    Ok(found)
}

/// Loads and parses the Circuit JSON of a board.
pub fn load(target: &BoardTarget) -> Result<Circuit, String> {
    let (path, source) = load_circuit_source(&target.entry).map_err(|e| e.to_string())?;
    parser::parse(&source, &path).map_err(|e| e.to_string())
}

fn source_error(message: String) -> Diagnostic {
    Diagnostic {
        rule: "source_error".into(),
        message,
        help: Some("Ensure the circuit file compiles and is valid JSON/TSX".into()),
        severity: Severity::Error,
    }
}

fn lint(target: &BoardTarget) -> Result<Vec<Diagnostic>, String> {
    let circuit = load(target)?;
    let netlist = Netlist::from_circuit(&circuit);
    let ctx = LintContext::new(&circuit, &netlist).with_mcu(target.mcu.as_deref());
    Ok(Linter::default().run_with_context(&ctx))
}

/// Lints one board; a source that cannot load blows the fuse with a `source_error`.
pub fn lint_board(target: &BoardTarget) -> BoardReport {
    let diagnostics = lint(target).unwrap_or_else(|message| vec![source_error(message)]);
    let blown = diagnostics.iter().any(|d| d.severity == Severity::Error);
    BoardReport {
        name: board_name(target),
        blown,
        diagnostics,
    }
}

/// `"blown"` when any board blew the fuse, else `"intact"`.
pub fn fuse(reports: &[BoardReport]) -> &'static str {
    match reports.iter().any(|r| r.blown) {
        true => "blown",
        false => "intact",
    }
}
