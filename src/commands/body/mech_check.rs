//! `fusible check` for bodies: the mechanical rules on the features of the last build.

use std::path::Path;

use super::mech_rules::check;
use super::part::features_path;
use super::part_features::PartReport;
use crate::bom::resolve_material;
use crate::commands::check::CheckError;
use crate::config::{BodyConfig, FusibleConfig};
use crate::linter::{Diagnostic, FuseBlown, Severity};

fn warn(rule: &str, message: String) {
    let diagnostic = Diagnostic {
        rule: rule.to_string(),
        message,
        help: None,
        severity: Severity::Warning,
        ..Default::default()
    };
    eprintln!("{:?}", miette::Report::new(diagnostic));
}

fn newer(source: &Path, cache: &Path) -> bool {
    let time = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    matches!((time(source), time(cache)), (Some(s), Some(c)) if s > c)
}

fn cached(root: &Path, name: &str, body: &BodyConfig) -> Option<PartReport> {
    let path = features_path(root, name, body);
    let report = serde_json::from_str(&std::fs::read_to_string(&path).ok()?).ok();
    let source = body.source.as_ref().map(|s| root.join(s));
    if source.is_some_and(|s| newer(&s, &path)) {
        warn(
            "mech.stale",
            format!(
                "{name} changed since its last build; run fusible body {name} to check it again"
            ),
        );
    }
    report
}

fn findings(name: &str, body: &BodyConfig, report: &PartReport) -> Vec<Diagnostic> {
    let (_, process) = resolve_material(body.material.as_deref(), body.process.as_deref());
    let found = check(name, &process, &report.features);
    found.iter().map(|f| f.diagnostic()).collect()
}

/// Runs the mechanical rules on every body that was built, and warns about code parts that were not.
///
/// Warnings print; any error blows the fuse.
pub fn verify_mech(cfg: &FusibleConfig, root: &Path) -> Result<(), CheckError> {
    let mut errors = Vec::new();
    for (name, body) in &cfg.bodies {
        let Some(report) = cached(root, name, body) else {
            if body.is_code() {
                let text = format!("{name} was not built yet; run fusible body {name} to check it");
                warn("mech.not_built", text);
            }
            continue;
        };
        let (warnings, blown): (Vec<_>, Vec<_>) = findings(name, body, &report)
            .into_iter()
            .partition(|d| d.severity != Severity::Error);
        warnings
            .into_iter()
            .for_each(|d| eprintln!("{:?}", miette::Report::new(d)));
        errors.extend(blown);
    }
    match errors.is_empty() {
        true => Ok(()),
        false => Err(FuseBlown::new(errors).into()),
    }
}
