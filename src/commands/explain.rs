//! Subcommand: `fusible explain <rule>` — what one rule checks, from the rule registry.
//!
//! The same lookup backs `fusible rules`, the MCP `explain_rule` tool and the generated
//! `docs/rules/` pages, so every surface describes a rule the same way.

use miette::Diagnostic;
use serde_json::{json, Value};
use thiserror::Error;

use crate::config::{is_netlist_rule, RuleLevel};
use crate::linter::{catalog, RuleInfo};
use crate::project::Project;

/// A rule id the registry does not know; a tool error (exit 2).
#[derive(Debug, Error, Diagnostic)]
#[error("unknown rule `{id}`")]
#[diagnostic(
    code(fusible::explain::unknown_rule),
    help("run `fusible rules` for the list of rule ids")
)]
pub struct UnknownRule {
    pub id: String,
}

/// Every registered rule, with whether the project's manifest feeds it.
pub fn registry(project: &Project) -> Vec<RuleInfo> {
    catalog(project.design_rules())
}

/// The registered rule named `id`.
pub fn find(project: &Project, id: &str) -> Result<RuleInfo, UnknownRule> {
    registry(project)
        .into_iter()
        .find(|rule| rule.id == id)
        .ok_or_else(|| UnknownRule { id: id.to_string() })
}

fn level_name(level: RuleLevel) -> &'static str {
    match level {
        RuleLevel::Off => "off",
        RuleLevel::Info => "info",
        RuleLevel::Warning => "warning",
        RuleLevel::Error => "error",
    }
}

/// The level `rules:` gives the rule in this project, else its default severity.
fn level(project: &Project, rule: &RuleInfo) -> &'static str {
    project
        .config
        .rules
        .get(rule.id)
        .map_or(rule.severity, |level| level_name(*level))
}

/// One rule as `fusible rules --json`, `explain --json` and MCP print it.
pub fn entry(project: &Project, rule: &RuleInfo) -> Value {
    json!({
        "id": rule.id,
        "category": rule.category,
        "summary": rule.summary,
        "severity": rule.severity,
        "level": level(project, rule),
        "standard": rule.standard,
        "source_url": rule.source_url,
        "enabled": rule.enabled,
        "waivable": !is_netlist_rule(rule.id),
    })
}

fn render(project: &Project, rule: &RuleInfo) -> String {
    let category = serde_json::to_value(rule.category).unwrap_or_default();
    let mut text = format!(
        "{}  [{}, {}]\n\n{}\n\n",
        rule.id,
        category.as_str().unwrap_or_default(),
        rule.severity,
        rule.summary
    );
    if let Some(standard) = rule.standard {
        text.push_str(&format!("Standard: {standard}\n"));
    }
    if let Some(url) = rule.source_url {
        text.push_str(&format!("Source:   {url}\n"));
    }
    text.push_str(&format!("Level:    {}\n", level(project, rule)));
    if !rule.enabled {
        text.push_str("Status:   off here; the manifest lacks the declarations it reads\n");
    }
    text.push_str(&waiver_hint(rule.id));
    text
}

fn waiver_hint(id: &str) -> String {
    match is_netlist_rule(id) {
        true => "\nThis rule guards the netlist: it cannot be lowered or waived.\n".to_string(),
        false => format!(
            "\nTo accept one case, add to fusible.yml:\n\nwaivers:\n  - rule: {id}\n    \
             match: U1.VDD\n    reason: why this case is safe\n"
        ),
    }
}

/// Prints the rule named `id`, as text or as one JSON object.
pub fn run(project: &Project, id: &str, json: bool) -> Result<(), UnknownRule> {
    let rule = find(project, id)?;
    match json {
        true => println!("{}", entry(project, &rule)),
        false => print!("{}", render(project, &rule)),
    }
    Ok(())
}
