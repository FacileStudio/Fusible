//! Diagnostics as JSON values.

use serde_json::Value;

use super::report::BoardReport;
use crate::linter::{Diagnostic, FuseBlown};

/// One diagnostic as the JSON object tools return: the same shape as the check report.
pub fn diagnostic_json(d: &Diagnostic) -> Value {
    serde_json::to_value(d).unwrap_or(Value::Null)
}

/// The diagnostics of a report as a JSON array.
pub fn diagnostics_json(report: &BoardReport) -> Vec<Value> {
    report.diagnostics.iter().map(diagnostic_json).collect()
}

/// The violations of a blown fuse as a JSON array.
pub fn violations_json(blown: &FuseBlown) -> Vec<Value> {
    blown.violations.iter().map(diagnostic_json).collect()
}
