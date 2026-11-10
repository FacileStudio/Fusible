//! The electrical rules on their `examples/erc-*.json` fixtures, through `fusible check --json`.

mod common;

use std::path::PathBuf;

use common::{fusible, TestDir};
use serde_json::Value;

/// One expected diagnostic: rule id, severity and element ids.
type Expected = (&'static str, &'static str, &'static [&'static str]);

const VOLTAGES: &str = "nets:\n  VDD3V3: { voltage: 3.3 }\n  VDD1V8: { voltage: 1.8 }\n";
const BUDGET: &str = "nets:\n  VDD3V3: { limit: 0.6, loads: { U1: 0.35, MODEM1: 2.0 } }\n";
const BATTERY: &str = "nets:\n  VBAT: { voltage: 4.2 }\n";

/// Fixture, manifest lines, and every non-info diagnostic of the rule under test.
const CASES: &[(&str, &str, &[Expected])] = &[
    (
        "pin_conflict",
        "",
        &[("pin_conflict", "error", &["sp_u1_out", "sp_u2_out"])],
    ),
    (
        "power_undriven",
        "",
        &[("power_undriven", "error", &["sp_u2_vin", "sp_u1_vddio"])],
    ),
    (
        "pin_unconnected",
        "",
        &[("pin_unconnected", "error", &["sp_u1_en"])],
    ),
    (
        "nc_connected",
        "",
        &[("nc_connected", "error", &["sp_u1_nc"])],
    ),
    (
        "missing_pullup",
        "",
        &[("missing_pullup", "error", &["sp_u1_sda"])],
    ),
    (
        "voltage_domain",
        VOLTAGES,
        &[
            ("voltage_domain", "error", &["sp_u3_vdd"]),
            ("voltage_domain", "error", &["sp_u1_tx", "sp_u2_rx"]),
        ],
    ),
    (
        "rail_budget",
        BUDGET,
        &[("rail_budget", "error", &["MODEM1", "U1"])],
    ),
    ("derating", BATTERY, &[("derating", "error", &["sc_c1"])]),
    (
        "led_series_resistor",
        "",
        &[("led_series_resistor", "warning", &["sc_d1"])],
    ),
    (
        "bring_up",
        "",
        &[
            ("rail_test_point", "warning", &["sp_u1_vdd", "sp_c1_1"]),
            ("debug_connector", "warning", &["U1"]),
            ("connector_clamp", "warning", &["sp_j1_1"]),
        ],
    ),
];

fn check(fixture: &str, manifest: &str) -> (Option<i32>, Value) {
    let dir = TestDir::new(&format!("erc_{fixture}"));
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = root.join(format!("examples/erc-{fixture}.json"));
    std::fs::copy(source, dir.path.join("circuit.json")).unwrap();
    let boards = "boards:\n  main:\n    entry: circuit.json\n";
    std::fs::write(dir.path.join("fusible.yml"), format!("{boards}{manifest}")).unwrap();
    let out = fusible()
        .current_dir(&dir.path)
        .args(["check", "--json"])
        .output()
        .expect("the binary must run");
    let json = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    (out.status.code(), json)
}

/// `(rule, severity, elements)` of every diagnostic of `json`'s board.
fn found(json: &Value) -> Vec<(String, String, Vec<String>)> {
    let text = |v: &Value| v.as_str().unwrap_or_default().to_string();
    let diagnostics = json["boards"][0]["diagnostics"].as_array().cloned();
    diagnostics
        .unwrap_or_default()
        .iter()
        .map(|d| {
            let elements = d["elements"].as_array().unwrap().iter().map(text).collect();
            (text(&d["rule"]), text(&d["severity"]), elements)
        })
        .collect()
}

#[test]
fn each_fixture_reports_exactly_its_rule() {
    for (fixture, manifest, expected) in CASES {
        let (_, json) = check(fixture, manifest);
        let all = found(&json);
        for (rule, severity, elements) in *expected {
            let hit = all
                .iter()
                .any(|(r, s, e)| r == rule && s == severity && e == elements);
            assert!(
                hit,
                "{fixture}: no {severity} {rule} on {elements:?} in {all:?}"
            );
            let count = all
                .iter()
                .filter(|(r, s, _)| r == rule && s != "info")
                .count();
            let wanted = expected.iter().filter(|(r, ..)| r == rule).count();
            assert_eq!(count, wanted, "{fixture}: {rule} in {all:?}");
        }
    }
}

#[test]
fn an_attribute_no_pin_declares_is_reported_once_at_info() {
    let (code, json) = check("pin_conflict", "");
    assert_eq!(code, Some(1));
    let infos: Vec<_> = found(&json)
        .into_iter()
        .filter(|(rule, severity, _)| rule == "power_undriven" && severity == "info")
        .collect();
    assert_eq!(infos.len(), 1, "{infos:?}");
}

fn errors(json: &Value) -> Vec<String> {
    let all = found(json).into_iter().filter(|(_, s, _)| s == "error");
    let mut rules: Vec<String> = all.map(|(rule, ..)| rule).collect();
    rules.dedup();
    rules
}

#[test]
fn the_combined_fixture_blows_with_three_rule_ids() {
    let (code, json) = check("combined", "");
    assert_eq!(code, Some(1), "{json}");
    assert_eq!(json["fuse"], "blown");
    assert_eq!(
        errors(&json),
        ["pin_conflict", "power_undriven", "nc_connected"]
    );
}

#[test]
fn a_reasoned_waiver_moves_one_rule_to_waived_and_the_others_keep_the_fuse_blown() {
    let waiver = concat!(
        "waivers:\n  - rule: nc_connected\n    match: U1.NC\n",
        "    reason: NC is bonded to ground inside this package, checked on the datasheet\n"
    );
    let (code, json) = check("combined", waiver);
    assert_eq!(code, Some(1), "{json}");
    assert_eq!(errors(&json), ["pin_conflict", "power_undriven"]);
    assert_eq!(json["waived"][0]["diagnostic"]["rule"], "nc_connected");
    assert!(json["waived"][0]["reason"]
        .as_str()
        .unwrap()
        .contains("datasheet"));
}
