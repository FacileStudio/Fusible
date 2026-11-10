//! `fusible check --json` on the `examples/dfm-*.json` fixtures: each one
//! blows or warns on its own DFM or DFA rule, with a location.

mod common;

use std::path::{Path, PathBuf};

use common::{fusible, TestDir};
use serde_json::Value;

const PROFILE: &str = "name: dfm\nfab:\n  name: dfm-test\n  source: https://example.invalid/caps\n  \
min_trace: 0.1\n  min_space: 0.1\n  min_drill: 0.1\n  min_via_pad: 0.2\n  min_annular: 0.18\n  \
min_via_to_via: 0.1\n  drill_step: 0.05\n  board_min: [5, 5]\n  board_max: [400, 400]\n  \
min_via_annular: 0.05\n  min_edge_clearance: 0.2\n  min_drill_to_copper: 0.2\n  min_hole_spacing: 0.45\n  \
max_aspect_ratio: 8\n  min_mask_dam: 0.1\n  min_silk_height: 1.0\n  min_silk_stroke: 0.15\n  \
min_silk_to_pad: 0.15\n  min_component_to_edge: 0.5\n  assembly_service: true\n";

fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(format!("dfm-{name}.json"))
}

fn check_with(manifest: &str, name: &str) -> (Option<i32>, Vec<Value>) {
    let dir = TestDir::new(&format!("dfm_{name}"));
    std::fs::write(dir.path.join("fusible.yml"), manifest).unwrap();
    let out = fusible()
        .current_dir(&dir.path)
        .args(["check", "--json"])
        .arg(example(name))
        .output()
        .expect("the binary must run");
    let json: Value = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    let found = json["boards"][0]["diagnostics"].as_array().cloned();
    (out.status.code(), found.unwrap_or_default())
}

fn fires(name: &str, rule: &str, severity: &str) -> Value {
    let (code, found) = check_with(PROFILE, name);
    let hit = found.iter().find(|d| d["rule"] == rule);
    let hit = hit.unwrap_or_else(|| panic!("{rule} did not fire on {name}: {found:?}"));
    assert_eq!(hit["severity"], severity, "{hit}");
    assert!(hit["location"]["x"].is_number(), "{hit}");
    if severity == "error" {
        assert_eq!(code, Some(1), "{found:?}");
    }
    hit.clone()
}

#[test]
fn drill_rules_blow_on_rings_copper_spacing_and_depth() {
    let ring = fires("annular-ring", "annular_ring", "error");
    assert!(
        ring["message"].as_str().unwrap().contains("0.10 mm"),
        "{ring}"
    );
    fires("drill-to-copper", "drill_to_copper", "error");
    fires("hole-spacing", "hole_spacing", "error");
    let deep = fires("via-aspect-ratio", "via_aspect_ratio", "error");
    assert!(
        deep["message"].as_str().unwrap().contains("10.7:1"),
        "{deep}"
    );
}

#[test]
fn surface_rules_flag_edge_copper_silk_mask_and_acid_traps() {
    fires("edge-clearance", "edge_clearance", "error");
    fires("silk-over-pad", "silk_over_pad", "warning");
    fires("silk-text-size", "silk_text_size", "warning");
    fires("mask-sliver", "mask_sliver", "warning");
    fires("acid-trap", "acid_trap", "warning");
}

#[test]
fn courtyard_clearance_turns_the_courtyard_with_the_part() {
    let hit = fires("courtyard-clearance", "courtyard_clearance", "error");
    assert_eq!(hit["elements"], serde_json::json!(["pc0", "pc1"]));
}

#[test]
fn assembly_rules_warn_on_edge_parts_missing_fiducials_and_part_numbers() {
    fires(
        "component-edge-clearance",
        "component_edge_clearance",
        "warning",
    );
    fires("fiducials", "fiducials", "warning");
    let hit = fires("bom-incomplete", "bom_incomplete", "warning");
    assert!(hit["message"].as_str().unwrap().ends_with(": R1"), "{hit}");
}

#[test]
fn a_profile_without_the_number_disables_the_rule_with_one_info() {
    let (_, found) = check_with("name: dfm\nfab: jlcpcb-2l\n", "via-aspect-ratio");
    let info: Vec<&Value> = found
        .iter()
        .filter(|d| d["rule"] == "via_aspect_ratio")
        .collect();
    assert_eq!(info.len(), 1, "{found:?}");
    assert_eq!(info[0]["severity"], "info");
    let (_, plain) = check_with("name: dfm\nfab: jlcpcb-2l\n", "fiducials");
    assert!(plain.iter().all(|d| d["rule"] != "fiducials"), "{plain:?}");
}
