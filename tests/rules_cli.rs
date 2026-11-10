//! `fusible rules` and `fusible explain`: the registry as a table, as JSON, and one rule.

mod common;

use common::{fusible, TestDir};
use serde_json::Value;

fn run(args: &[&str]) -> (Option<i32>, String) {
    let dir = TestDir::new("rules_cli");
    let out = fusible()
        .current_dir(&dir.path)
        .args(args)
        .output()
        .expect("binary should run");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into(),
    )
}

#[test]
fn rules_json_lists_every_rule_with_its_metadata() {
    let (code, stdout) = run(&["rules", "--json"]);
    let rules: Vec<Value> = serde_json::from_str(&stdout).expect("stdout is one JSON array");

    assert_eq!(code, Some(0));
    assert!(rules.len() >= 20, "{}", rules.len());
    for rule in &rules {
        for key in ["id", "category", "summary", "severity", "level"] {
            assert!(rule[key].is_string(), "{key} missing in {rule}");
        }
    }
    let netlist = rules.iter().find(|r| r["id"] == "unconnected_net").unwrap();
    assert_eq!(netlist["waivable"], false);
}

#[test]
fn rules_prints_a_table_with_one_row_per_rule() {
    let (code, stdout) = run(&["rules"]);

    assert_eq!(code, Some(0));
    assert!(stdout.starts_with("RULE"));
    assert!(stdout.lines().any(|l| l.starts_with("trace_current ")));
}

#[test]
fn explain_describes_one_rule_and_an_unknown_id_is_a_tool_error() {
    let (code, stdout) = run(&["explain", "trace_current", "--json"]);
    let rule: Value = serde_json::from_str(&stdout).unwrap();
    let (text_code, text) = run(&["explain", "decoupling_capacitor"]);
    let (unknown, _) = run(&["explain", "no_such_rule"]);

    assert_eq!(code, Some(0));
    assert_eq!(rule["standard"], "IPC-2152");
    assert_eq!(rule["category"], "drc");
    assert_eq!(text_code, Some(0));
    assert!(text.contains("waivers:"), "{text}");
    assert_eq!(unknown, Some(2));
}
