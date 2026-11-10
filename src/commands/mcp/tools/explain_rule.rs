//! Tool `explain_rule`: what a rule id means.

use serde_json::Value;

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::commands::explain;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str =
    "Explain a rule id from a violation: its category, summary, default severity, level in \
this project, standard, source and whether it can be waived.";

/// Input schema: the rule id.
pub fn schema() -> Value {
    object(
        &[("rule", string("Rule id, such as trace_current"))],
        &["rule"],
    )
}

/// Looks the rule up through the same registry as `fusible explain`.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let id = match required_str(args, "rule") {
        Ok(id) => id,
        Err(outcome) => return outcome,
    };
    let project = session.project();
    match explain::find(&project, id) {
        Ok(rule) => Outcome::ok(explain::entry(&project, &rule)),
        Err(unknown) => Outcome::fail(format!("{unknown}; call with an id from `fusible rules`")),
    }
}
