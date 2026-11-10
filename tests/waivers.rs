//! `rules:` levels and `waivers:` of the manifest, through `fusible check --json`.

mod common;

use std::path::PathBuf;
use std::process::Output;

use common::{fusible, TestDir};
use serde_json::Value;

const CIRCUIT: &str = "tests/fixtures/waivers/no-decoupling.json";

fn check(manifest: &str) -> (Output, Value) {
    let dir = TestDir::new("waivers");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::fs::copy(root.join(CIRCUIT), dir.path.join("circuit.json")).unwrap();
    let boards = "boards:\n  main:\n    entry: circuit.json\n";
    std::fs::write(dir.path.join("fusible.yml"), format!("{boards}{manifest}")).unwrap();
    let out = fusible()
        .current_dir(&dir.path)
        .args(["check", "--json"])
        .output()
        .expect("the binary must run");
    let json = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    (out, json)
}

fn rules(json: &Value) -> Vec<&str> {
    let diagnostics = json["boards"][0]["diagnostics"].as_array().unwrap();
    diagnostics
        .iter()
        .filter(|d| d["rule"] == "decoupling_capacitor" || d["severity"] == "error")
        .map(|d| d["rule"].as_str().unwrap())
        .collect()
}

#[test]
fn without_a_waiver_the_missing_capacitor_blows_the_fuse() {
    let (out, json) = check("");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(json["version"], 2);
    assert_eq!(json["fuse"], "blown");
    assert_eq!(rules(&json), ["decoupling_capacitor"]);
}

#[test]
fn a_waiver_turns_a_blown_fuse_intact_and_lists_the_diagnostic() {
    let waiver = concat!(
        "waivers:\n  - rule: decoupling_capacitor\n    match: U1.VDD\n",
        "    reason: bulk cap on the carrier board\n"
    );
    let (out, json) = check(waiver);
    assert_eq!(out.status.code(), Some(0), "{json}");
    assert_eq!(json["fuse"], "intact");
    assert!(rules(&json).is_empty());
    assert_eq!(
        json["waived"][0]["diagnostic"]["rule"],
        "decoupling_capacitor"
    );
    assert_eq!(json["waived"][0]["reason"], "bulk cap on the carrier board");
    assert_eq!(json["waived"][0]["board"], "main");
}

#[test]
fn a_rule_turned_off_is_dropped_and_one_lowered_to_warning_keeps_the_fuse() {
    let (out, json) = check("rules:\n  decoupling_capacitor: off\n");
    assert_eq!(out.status.code(), Some(0), "{json}");
    assert!(rules(&json).is_empty());

    let (out, json) = check("rules:\n  decoupling_capacitor: warning\n");
    assert_eq!(out.status.code(), Some(0), "{json}");
    let diagnostics = json["boards"][0]["diagnostics"].as_array().unwrap();
    let lowered = diagnostics
        .iter()
        .find(|d| d["rule"] == "decoupling_capacitor");
    assert_eq!(lowered.unwrap()["severity"], "warning");
}

#[test]
fn an_unknown_rule_id_is_a_tool_error() {
    let (out, json) = check("rules:\n  no_such_rule: off\n");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(json["fuse"], "error");
    assert!(json["error"].as_str().unwrap().contains("no_such_rule"));
}

#[test]
fn an_expired_waiver_no_longer_applies_and_is_listed() {
    let waiver = concat!(
        "waivers:\n  - rule: decoupling_capacitor\n    match: U1.VDD\n",
        "    reason: rev A only\n    until: 2000-01-01\n"
    );
    let (out, json) = check(waiver);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(json["fuse"], "blown");
    assert_eq!(json["expired_waivers"][0]["until"], "2000-01-01");
}

#[test]
fn a_waiver_that_matches_nothing_is_a_warning_not_an_error() {
    let waiver = concat!(
        "rules:\n  decoupling_capacitor: off\n",
        "waivers:\n  - rule: decoupling_capacitor\n    match: U9.VDD\n    reason: gone\n"
    );
    let (out, json) = check(waiver);
    assert_eq!(out.status.code(), Some(0), "{json}");
    assert_eq!(json["unused_waivers"][0]["rule"], "unused_waiver");
    assert_eq!(json["unused_waivers"][0]["severity"], "warning");
}
