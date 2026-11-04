//! Tool `edit_body`: change the TypeScript source of a code-first body.

use serde_json::Value;

use super::args::{boolean, flag, object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::edit;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str = "Edit the .ts source of a code-first body with full new content. \
dry_run defaults to true. Mechanical rules are not wired yet, so fuse is null.";

/// Input schema: body, content, dry_run.
pub fn schema() -> Value {
    object(
        &[
            ("name", string("Body name")),
            ("content", string("Full new content of the body source")),
            (
                "dry_run",
                boolean("Preview without keeping the change (default true)"),
            ),
        ],
        &["name", "content"],
    )
}

fn source_of(session: &Session, name: &str) -> Result<std::path::PathBuf, String> {
    let cfg = session.config()?.ok_or("the project has no manifest")?;
    let body = cfg
        .bodies
        .get(name)
        .ok_or_else(|| format!("unknown body \"{name}\""))?;
    let source = body
        .source
        .clone()
        .ok_or_else(|| format!("body \"{name}\" has no source file"))?;
    match source.extension().is_some_and(|e| e == "ts") {
        true => Ok(source),
        false => Err(format!(
            "body \"{name}\" is not a code part: its source is not a .ts file"
        )),
    }
}

fn apply(session: &mut Session, args: &Value) -> Result<Outcome, Outcome> {
    let name = required_str(args, "name")?;
    let content = required_str(args, "content")?;
    let source = source_of(session, name).map_err(Outcome::fail)?;
    let change = edit::Change::Content(content.to_string());
    let check = || (Value::Null, Vec::<Value>::new());
    Ok(edit::edit(
        session,
        &source,
        &change,
        flag(args, "dry_run", true),
        &check,
    ))
}

/// Applies the change under the write-safety rules.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    apply(session, args).unwrap_or_else(|outcome| outcome)
}
