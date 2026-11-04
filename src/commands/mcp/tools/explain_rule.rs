//! Tool `explain_rule`: what a rule id means.

use serde_json::{json, Value};

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::session::Session;
use crate::linter::{
    BypassCapRule, DanglingReferenceRule, GhostNetRule, MalformedElementRule, PinMuxRule, Rule,
    SchematicScaleRule, ShortedNetsRule, TraceClearanceRule, UnconnectedNetRule,
};

pub const DESCRIPTION: &str =
    "Explain a rule id from a violation: its severity and what it checks.";

/// Input schema: the rule id.
pub fn schema() -> Value {
    object(
        &[("rule", string("Rule id, such as unconnected_net"))],
        &["rule"],
    )
}

fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(UnconnectedNetRule),
        Box::new(MalformedElementRule),
        Box::new(GhostNetRule),
        Box::new(DanglingReferenceRule),
        Box::new(ShortedNetsRule),
        Box::new(PinMuxRule),
        Box::new(BypassCapRule),
        Box::new(TraceClearanceRule),
        Box::new(SchematicScaleRule),
    ]
}

/// Looks the rule up by id. Summary, standard and source URL need the rules track.
pub fn run(_session: &mut Session, args: &Value) -> Outcome {
    let id = match required_str(args, "rule") {
        Ok(id) => id,
        Err(outcome) => return outcome,
    };
    match rules().iter().find(|r| r.name() == id) {
        Some(rule) => Outcome::ok(
            json!({"id": id, "severity": format!("{:?}", rule.severity()).to_lowercase()}),
        ),
        None => Outcome::fail("unknown rule"),
    }
}
