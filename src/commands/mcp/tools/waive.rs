//! Tool `waive`: append a reasoned waiver to `fusible.yml`, then check the project.

use serde_json::{json, Value};

use super::args::{boolean, flag, object, optional_str, required_str, string};
use super::outcome::Outcome;
use crate::check::{self, known_rule_ids, Scope};
use crate::commands::mcp::edit::{self, Change};
use crate::commands::mcp::session::Session;
use crate::commands::mcp::waiver_text::{append, NewWaiver};
use crate::config::{is_netlist_rule, Date, FusibleConfig};
use crate::project::Project;

pub const DESCRIPTION: &str =
    "Accept one diagnostic on purpose: append a waiver (rule, match, reason, optional until \
date) to fusible.yml, keeping its comments, then check. The reason is required and must say why \
the case is safe. Netlist rules cannot be waived. Returns the diff, the fuse and, as diagnostics, \
the waived ones. dry_run defaults to true.";

/// Input schema: rule, match, reason, until, dry_run.
pub fn schema() -> Value {
    object(
        &[
            ("rule", string("Rule id of the diagnostic")),
            (
                "match",
                string("Element id or REFDES.PIN the diagnostic names, such as U1.VDD"),
            ),
            ("reason", string("Why this case is safe; never empty")),
            ("until", string("Last day the waiver applies, YYYY-MM-DD")),
            (
                "dry_run",
                boolean("Preview without keeping the change (default true)"),
            ),
        ],
        &["rule", "match", "reason"],
    )
}

fn waiver<'a>(args: &'a Value) -> Result<NewWaiver<'a>, Outcome> {
    let rule = required_str(args, "rule")?;
    let matches = required_str(args, "match")?.trim();
    let reason = required_str(args, "reason")?.trim();
    let until = optional_str(args, "until");
    if !known_rule_ids().contains(&rule) {
        return Err(Outcome::fail(format!("unknown rule `{rule}`")));
    }
    if is_netlist_rule(rule) {
        return Err(Outcome::fail(format!(
            "`{rule}` guards the netlist and cannot be waived"
        )));
    }
    if matches.is_empty() || reason.is_empty() {
        return Err(Outcome::fail("match and reason must not be empty"));
    }
    if until.is_some_and(|day| Date::parse(day).is_none()) {
        return Err(Outcome::fail("until must be a date written YYYY-MM-DD"));
    }
    Ok(NewWaiver {
        rule,
        matches,
        reason,
        until,
    })
}

fn rewritten(text: &str, waiver: &NewWaiver) -> Result<String, String> {
    let before = FusibleConfig::from_str(text).map_err(|e| e.to_string())?;
    let after_text = append(text, waiver)?;
    let after = FusibleConfig::from_str(&after_text).map_err(|e| e.to_string())?;
    match after.waivers.len() == before.waivers.len() + 1 {
        true => Ok(after_text),
        false => Err("the waiver did not land in `waivers:`; add it by hand".into()),
    }
}

/// The fuse and the waived diagnostics once the manifest holds the change.
fn verdict() -> (Value, Vec<Value>) {
    match check::run(&Project::current(), Scope::All) {
        Ok(report) => {
            let waived = report.waived.iter().map(|w| json!(w)).collect();
            (json!(report.fuse), waived)
        }
        Err(err) => (json!("error"), vec![json!({"error": err.to_string()})]),
    }
}

fn prepare(session: &mut Session, args: &Value) -> Result<Outcome, Outcome> {
    let waiver = waiver(args)?;
    let manifest = session
        .manifest
        .clone()
        .ok_or_else(|| Outcome::fail("this project has no fusible.yml"))?;
    let text = std::fs::read_to_string(&manifest).map_err(|e| Outcome::fail(e.to_string()))?;
    let change = Change::Content(rewritten(&text, &waiver).map_err(Outcome::fail)?);
    let dry = flag(args, "dry_run", true);
    Ok(edit::edit(session, &manifest, &change, dry, &verdict))
}

/// Appends the waiver under the write-safety rules of `edit_board`.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    prepare(session, args).unwrap_or_else(|outcome| outcome)
}
