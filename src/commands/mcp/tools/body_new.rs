//! Tool `body_new`: scaffold a code-first body (needs the bodies track).

use serde_json::Value;

use super::args::{object, string};
use super::outcome::Outcome;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str =
    "Create a code-first body. Not available until Fusible 1.0 bodies land.";

/// Input schema: name, material, process.
pub fn schema() -> Value {
    object(
        &[
            ("name", string("Body name")),
            ("material", string("Material, such as petg")),
            (
                "process",
                string("Process: fdm, sla, mjf, injection or cnc"),
            ),
        ],
        &["name"],
    )
}

/// Always reports that the feature is missing.
pub fn run(_session: &mut Session, _args: &Value) -> Outcome {
    Outcome::fail("body_new needs Fusible 1.0 bodies")
}
