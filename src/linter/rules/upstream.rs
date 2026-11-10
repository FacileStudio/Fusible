//! Errors and warnings tscircuit recorded itself, one rule id per kind.
//!
//! tscircuit writes a failed placement, overlap or autoroute as an
//! `*_error` element and keeps going. Each error is its own diagnostic and
//! blows the fuse. Warnings of one kind are folded into one diagnostic, so
//! thirty unnamed traces print one line.

pub(super) mod kinds;

use std::collections::BTreeMap;

use crate::linter::{Diagnostic, LintContext, Rule, RuleMeta};
use crate::parser::model::UpstreamIssue;
use kinds::{rule_for, EMITS, UPSTREAM_ERROR};

const ERROR_HELP: &str = "fix the error tscircuit reported; fab files from this board are unusable";
const WARNING_HELP: &str = "tscircuit reported this while building the board";
const THERMALPAD_HINT: &str =
    "a thermalpad only connects when the chip has a pinLabel named thermalpad";

/// Maps every `*_error` and `*_warning` element to its rule id.
pub struct UpstreamRule;

fn message(issue: &UpstreamIssue) -> String {
    let message = issue.message.as_deref().unwrap_or("no message");
    format!("{}: {message}", issue.kind)
}

fn diagnostic(meta: &RuleMeta, message: String, help: String) -> Diagnostic {
    Diagnostic {
        rule: meta.id.to_string(),
        message,
        help: Some(help),
        severity: meta.default_severity,
        ..Default::default()
    }
}

fn error(issue: &UpstreamIssue) -> Diagnostic {
    let text = message(issue);
    let help = match text.contains("thermalpad") {
        true => format!("{ERROR_HELP}; {THERMALPAD_HINT}"),
        false => ERROR_HELP.to_string(),
    };
    Diagnostic {
        elements: issue.element_ids.clone(),
        ..diagnostic(rule_for(&issue.kind, true), text, help)
    }
}

fn warnings<'a>(issues: impl Iterator<Item = &'a UpstreamIssue>) -> Vec<Diagnostic> {
    let mut by_kind: BTreeMap<&str, Vec<&UpstreamIssue>> = BTreeMap::new();
    for issue in issues {
        by_kind.entry(issue.kind.as_str()).or_default().push(issue);
    }
    by_kind
        .into_iter()
        .map(|(kind, group)| {
            let text = match group.len() {
                1 => message(group[0]),
                n => format!("{} (and {} more {kind})", message(group[0]), n - 1),
            };
            diagnostic(rule_for(kind, false), text, WARNING_HELP.to_string())
        })
        .collect()
}

impl Rule for UpstreamRule {
    fn meta(&self) -> &'static RuleMeta {
        &UPSTREAM_ERROR
    }

    fn emits(&self) -> &'static [&'static RuleMeta] {
        &EMITS
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let issues = &ctx.circuit.model().issues;
        let mut found: Vec<Diagnostic> = issues.iter().filter(|i| i.is_error).map(error).collect();
        found.extend(warnings(issues.iter().filter(|i| !i.is_error)));
        found
    }
}
