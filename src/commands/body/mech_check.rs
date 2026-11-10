//! `fusible check` for bodies: the mechanical rules on the features of the last build.

use std::path::Path;

use super::mech_rules::check;
use super::part::features_path;
use super::part_features::PartReport;
use crate::bom::resolve_material;
use crate::commands::check::CheckError;
use crate::config::{BodyConfig, FusibleConfig};
use crate::linter::{Diagnostic, FuseBlown, Severity};

fn warning(rule: &str, message: String) -> Diagnostic {
    Diagnostic {
        rule: rule.to_string(),
        message,
        help: None,
        severity: Severity::Warning,
        ..Default::default()
    }
}

fn newer(source: &Path, cache: &Path) -> bool {
    let time = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    matches!((time(source), time(cache)), (Some(s), Some(c)) if s > c)
}

fn cached(root: &Path, name: &str, body: &BodyConfig) -> (Option<PartReport>, Vec<Diagnostic>) {
    let path = features_path(root, name, body);
    let report = std::fs::read_to_string(&path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok());
    let source = body.source.as_ref().map(|s| root.join(s));
    let stale = source.is_some_and(|s| newer(&s, &path)).then(|| {
        let text = format!(
            "{name} changed since its last build; run fusible body {name} to check it again"
        );
        warning("mech.stale", text)
    });
    (report, stale.into_iter().collect())
}

fn findings(name: &str, body: &BodyConfig, report: &PartReport) -> Vec<Diagnostic> {
    let (_, process) = resolve_material(body.material.as_deref(), body.process.as_deref());
    let found = check(name, &process, &report.features);
    found.iter().map(|f| f.diagnostic()).collect()
}

/// Every mechanical finding of the bodies that were built, plus a warning per code part that was not.
pub fn mech_findings(cfg: &FusibleConfig, root: &Path) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    for (name, body) in &cfg.bodies {
        let (report, warnings) = cached(root, name, body);
        found.extend(warnings);
        match report {
            Some(report) => found.extend(findings(name, body, &report)),
            None if body.is_code() => found.push(warning(
                "mech.not_built",
                format!("{name} was not built yet; run fusible body {name} to check it"),
            )),
            None => {}
        }
    }
    found
}

/// Runs the mechanical rules on every body that was built, and warns about code parts that were not.
///
/// Warnings print on stderr; any error blows the fuse.
pub fn verify_mech(cfg: &FusibleConfig, root: &Path) -> Result<(), CheckError> {
    let (blown, warnings): (Vec<_>, Vec<_>) = mech_findings(cfg, root)
        .into_iter()
        .partition(|d| d.severity == Severity::Error);
    warnings
        .into_iter()
        .for_each(|d| eprintln!("{:?}", miette::Report::new(d)));
    match blown.is_empty() {
        true => Ok(()),
        false => Err(FuseBlown::new(blown).into()),
    }
}
