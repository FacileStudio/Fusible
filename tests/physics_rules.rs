//! `fusible check --json` on the physics and fab examples, with the manifest in `tests/fixtures/rules`.

mod common;

use std::path::PathBuf;
use std::process::Output;

use common::fusible;
use serde_json::Value;

fn check(example: &str, extra: &[&str]) -> (Output, Value) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out = fusible()
        .current_dir(root.join("tests/fixtures/rules"))
        .arg("check")
        .arg(root.join("examples").join(example))
        .args(extra)
        .output()
        .expect("the binary must run");
    let json = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    (out, json)
}

fn diagnostics(json: &Value) -> Vec<&Value> {
    json["boards"][0]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .collect()
}

#[test]
fn an_intact_circuit_prints_valid_json_and_exits_zero() {
    let (out, json) = check("circuit.json", &["--json"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(json["fuse"], "intact");
    assert!(diagnostics(&json).is_empty());
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn trace_current_blows_with_standard_elements_and_fix() {
    let (out, json) = check("circuit-trace-current.json", &["--json"]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(json["fuse"], "blown");
    let found = diagnostics(&json);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0]["rule"], "trace_current");
    assert_eq!(found[0]["standard"], "IPC-2152");
    assert_eq!(found[0]["severity"], "error");
    assert_eq!(found[0]["elements"][0], "pcb_trace_0");
    assert!(found[0]["fix"]
        .as_str()
        .unwrap()
        .starts_with("widen VBUS to at least"));
}

#[test]
fn voltage_clearance_blows_on_mains_next_to_ground() {
    let (out, json) = check("circuit-voltage-clearance.json", &["--json"]);
    assert_eq!(out.status.code(), Some(1));
    let found = diagnostics(&json);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0]["rule"], "voltage_clearance");
    assert_eq!(found[0]["standard"], "IPC-2221B");
}

#[test]
fn fab_profile_blows_on_a_thin_trace_and_a_small_drill() {
    let (out, json) = check("circuit-fab-profile.json", &["--json"]);
    assert_eq!(out.status.code(), Some(1));
    let found = diagnostics(&json);
    assert!(
        found
            .iter()
            .all(|d| d["rule"] == "fab_profile" && d["standard"] == "jlcpcb-2l"),
        "{found:?}"
    );
    assert!(found
        .iter()
        .any(|d| d["message"].as_str().unwrap().contains("trace 0.10 mm")));
    assert!(found
        .iter()
        .any(|d| d["message"].as_str().unwrap().contains("via drill 0.25 mm")));
}

#[test]
fn human_output_still_blows_with_exit_one() {
    let (out, _) = check("circuit-trace-current.json", &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("trace_current"));
}

#[test]
fn a_missing_file_is_a_tool_error_in_json() {
    let (out, json) = check("does-not-exist.json", &["--json"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(json["fuse"], "error");
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("does-not-exist.json"));
}
