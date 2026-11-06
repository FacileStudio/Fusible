//! Diagnostics as JSON values.

use serde_json::{json, Value};

use super::report::BoardReport;
use crate::linter::{Diagnostic, FuseBlown, Severity};

/// One diagnostic as the JSON object tools return.
pub fn diagnostic_json(d: &Diagnostic) -> Value {
    let severity = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    let mut value =
        json!({"rule": d.rule, "message": d.message, "help": d.help, "severity": severity});
    let extra = [
        ("standard", json!(d.standard)),
        ("source_url", json!(d.source_url)),
        ("fix", json!(d.fix)),
    ];
    for (key, field) in extra.into_iter().filter(|(_, f)| !f.is_null()) {
        value[key] = field;
    }
    if !d.elements.is_empty() {
        value["elements"] = json!(d.elements);
    }
    value
}

/// The diagnostics of a report as a JSON array.
pub fn diagnostics_json(report: &BoardReport) -> Vec<Value> {
    report.diagnostics.iter().map(diagnostic_json).collect()
}

/// The violations of a blown fuse as a JSON array.
pub fn violations_json(blown: &FuseBlown) -> Vec<Value> {
    blown.violations.iter().map(diagnostic_json).collect()
}
