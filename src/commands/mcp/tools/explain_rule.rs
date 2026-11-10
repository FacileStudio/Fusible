//! Tool `explain_rule`: what a rule id means.

use serde_json::{json, Value};

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::session::Session;
use crate::linter::Linter;

pub const DESCRIPTION: &str =
    "Explain a rule id from a violation: its summary, severity, standard and source.";

/// Input schema: the rule id.
pub fn schema() -> Value {
    object(
        &[("rule", string("Rule id, such as trace_current"))],
        &["rule"],
    )
}

/// Looks the rule up by id in the registered rule set.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let id = match required_str(args, "rule") {
        Ok(id) => id,
        Err(outcome) => return outcome,
    };
    let project = session.project();
    let rules = Linter::default().describe(project.design_rules());
    match rules.iter().find(|r| r.id == id) {
        Some(rule) => Outcome::ok(json!({
            "id": rule.id,
            "summary": rule.summary,
            "severity": rule.severity,
            "standard": rule.standard,
            "source_url": rule.source_url,
            "enabled": rule.enabled,
        })),
        None => Outcome::fail("unknown rule"),
    }
}
