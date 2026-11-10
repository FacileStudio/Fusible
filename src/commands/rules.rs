//! Subcommand: `fusible rules` — every registered rule, as a table or as JSON.

use serde_json::Value;

use super::explain::{entry, registry};
use crate::project::Project;

/// The JSON array `fusible rules --json` prints, in registration order.
pub fn rules_json(project: &Project) -> Value {
    let rules = registry(project);
    Value::Array(rules.iter().map(|rule| entry(project, rule)).collect())
}

fn row(rule: &Value, width: usize) -> String {
    let field = |key: &str| rule[key].as_str().unwrap_or_default().to_string();
    let id = field("id");
    format!(
        "{id:<width$}  {:<10}  {:<7}  {}",
        field("category"),
        field("level"),
        field("summary")
    )
}

/// Prints the rules; `json` prints one JSON array and nothing else.
pub fn run(project: &Project, json: bool) {
    let rules = rules_json(project);
    if json {
        println!("{rules}");
        return;
    }
    let list = rules.as_array().cloned().unwrap_or_default();
    let width = list
        .iter()
        .filter_map(|r| r["id"].as_str().map(str::len))
        .max()
        .unwrap_or(4);
    println!(
        "{:<width$}  {:<10}  {:<7}  SUMMARY",
        "RULE", "CATEGORY", "LEVEL"
    );
    for rule in &list {
        println!("{}", row(rule, width));
    }
    println!(
        "\n{} rules. `fusible explain <rule>` describes one.",
        list.len()
    );
}
