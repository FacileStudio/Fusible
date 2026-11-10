//! Tool `apply_fix`: apply a diagnostic's `set_prop` fix to the board source, then check it.

use serde_json::Value;

use super::args::{boolean, flag, object, string};
use super::edit_board::{edit_target, target};
use super::outcome::Outcome;
use crate::commands::mcp::edit::Change;
use crate::commands::mcp::session::Session;
use crate::commands::mcp::tsx_prop::set_prop;
use crate::commands::target::is_tsx_target;

pub const DESCRIPTION: &str =
    "Apply one machine-applicable fix from a check diagnostic: pass the `set_prop` object of \
its `fixes` (selector, prop, value). The prop is set on the board's TSX tag through the same \
safety rules as edit_board, then the board is checked. dry_run defaults to true.";

/// Input schema: board, fix, dry_run.
pub fn schema() -> Value {
    let mut schema = object(
        &[
            ("board", string("Board name")),
            (
                "dry_run",
                boolean("Preview without keeping the change (default true)"),
            ),
        ],
        &["board", "fix"],
    );
    schema["properties"]["fix"] = serde_json::json!({
        "type": "object",
        "description": "A fix from the report: {\"set_prop\": {selector, prop, value}} or its inner object",
    });
    schema
}

/// The selector, prop and value of a `set_prop` fix, wrapped or bare.
fn set_prop_fix(args: &Value) -> Result<(&str, &str, &Value), Outcome> {
    let fix = args
        .get("fix")
        .ok_or_else(|| Outcome::fail("fix is required"))?;
    let inner = fix.get("set_prop").unwrap_or(fix);
    let text = |key: &str| inner.get(key).and_then(Value::as_str);
    match (text("selector"), text("prop"), inner.get("value")) {
        (Some(selector), Some(prop), Some(value)) => Ok((selector, prop, value)),
        _ => Err(Outcome::fail(
            "only set_prop fixes apply; give selector, prop and value",
        )),
    }
}

fn prepare(session: &mut Session, args: &Value) -> Result<Outcome, Outcome> {
    let (selector, prop, value) = set_prop_fix(args)?;
    let target = target(session, args)?;
    if !is_tsx_target(&target.entry) {
        return Err(Outcome::fail("apply_fix edits TSX boards only"));
    }
    let source =
        std::fs::read_to_string(&target.entry).map_err(|e| Outcome::fail(e.to_string()))?;
    let content = set_prop(&source, selector, prop, value).map_err(Outcome::fail)?;
    let dry = flag(args, "dry_run", true);
    Ok(edit_target(
        session,
        &target,
        &Change::Content(content),
        dry,
    ))
}

/// Applies the fix under the write-safety rules of `edit_board`.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    prepare(session, args).unwrap_or_else(|outcome| outcome)
}
