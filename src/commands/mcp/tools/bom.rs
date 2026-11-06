//! Tool `bom`: the bill of materials.

use serde_json::{json, Value};

use super::args::object;
use super::outcome::Outcome;
use crate::cli::{BomArgs, BomFormat, OpusArgs};
use crate::commands::bom;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str = "Return the bill of materials: parts, quantities, prices and totals.";

/// Input schema: no arguments.
pub fn schema() -> Value {
    object(&[], &[])
}

/// Builds the same report as `fusible bom --format json`.
pub fn run(_session: &mut Session, _args: &Value) -> Outcome {
    let args = BomArgs {
        target: None,
        format: BomFormat::Json,
        output: None,
        opus: OpusArgs::default(),
    };
    match bom::report(&args) {
        Ok(report) => Outcome::ok(json!(report)),
        Err(err) => Outcome::fail(err.to_string()),
    }
}
