//! Integration tests for `fusible migrate`: the 1.x manifest rewrite and its exit codes.

mod common;

use common::{run_in, TestDir};

const LEGACY: &str = "# Sensor node, rev B
name: sensor

# The board
hardware:
  entry: hardware/index.tsx   # tscircuit entry
  out_dir: dist

mcu:
  chip: esp32-s3

firmware:
  path: firmware
  framework: embassy

mechanical:
  format: stl
  wall_thickness: 2.0

rules:
  E_SCHEMATIC_SCALE_EXCESSIVE: warning
";

const MIGRATED: &str = "# Sensor node, rev B
name: sensor

# The board
boards:
  main:
    mcu: esp32-s3
    entry: hardware/index.tsx   # tscircuit entry
    out_dir: dist

firmware:
  firmware:
    path: firmware
    framework: embassy

bodies:
  sensor:
    generate: enclosure
    out_dir: dist/mechanical
    format: stl
    wall_thickness: 2.0

rules:
  schematic_scale: warning
";

fn project(name: &str, manifest: &str) -> TestDir {
    let dir = TestDir::new(name);
    std::fs::write(dir.path.join("fusible.yml"), manifest).unwrap();
    dir
}

fn manifest(dir: &TestDir) -> String {
    std::fs::read_to_string(dir.path.join("fusible.yml")).unwrap()
}

#[test]
fn check_reports_a_needed_rewrite_and_writes_nothing() {
    let dir = project("migrate_check", LEGACY);
    let (code, stdout, _) = run_in(&dir.path, &["migrate", "--check"]);
    assert_eq!(code, Some(1), "{stdout}");
    assert!(stdout.contains("--- fusible.yml"), "{stdout}");
    assert!(stdout.contains("-hardware:"), "{stdout}");
    assert!(stdout.contains("+boards:"), "{stdout}");
    assert!(stdout.contains("+  schematic_scale: warning"), "{stdout}");
    assert_eq!(manifest(&dir), LEGACY);
}

#[test]
fn migrate_rewrites_the_legacy_form_and_keeps_comments() {
    let dir = project("migrate_write", LEGACY);
    let (code, stdout, stderr) = run_in(&dir.path, &["migrate"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert_eq!(manifest(&dir), MIGRATED);
    let (code, stdout, _) = run_in(&dir.path, &["migrate", "--check"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert!(stdout.contains("already in the 2.0 form"), "{stdout}");
}

#[test]
fn a_current_manifest_is_left_alone() {
    let current = "name: demo\nboards:\n  main: { circuit_json: circuit.json }\n";
    let dir = project("migrate_current", current);
    for args in [&["migrate", "--check"][..], &["migrate"][..]] {
        let (code, stdout, _) = run_in(&dir.path, args);
        assert_eq!(code, Some(0), "{stdout}");
    }
    assert_eq!(manifest(&dir), current);
}

#[test]
fn an_mcu_without_hardware_becomes_the_main_board() {
    let dir = project("migrate_mcu", "mcu:\n  chip: rp2040\n");
    let (code, _, stderr) = run_in(&dir.path, &["migrate"]);
    assert_eq!(code, Some(0), "{stderr}");
    assert_eq!(manifest(&dir), "boards:\n  main:\n    mcu: rp2040\n");
}

#[test]
fn crlf_line_endings_survive() {
    let dir = project("migrate_crlf", "hardware:\r\n  entry: a.tsx\r\n");
    let (code, _, stderr) = run_in(&dir.path, &["migrate"]);
    assert_eq!(code, Some(0), "{stderr}");
    assert_eq!(manifest(&dir), "boards:\r\n  main:\r\n    entry: a.tsx\r\n");
}

#[test]
fn unsafe_or_unreadable_manifests_are_tool_errors() {
    let inline = "hardware: { entry: a.tsx }\nmcu:\n  chip: esp32\n";
    let broken = "hardware: [oops\n";
    for (name, text) in [("migrate_inline", inline), ("migrate_broken", broken)] {
        let dir = project(name, text);
        let (code, _, stderr) = run_in(&dir.path, &["migrate", "--check"]);
        assert_eq!(code, Some(2), "{name}: {stderr}");
        assert_eq!(manifest(&dir), text, "{name}");
    }
    let empty = TestDir::new("migrate_missing");
    let (code, _, stderr) = run_in(&empty.path, &["migrate"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.contains("no fusible.yml"), "{stderr}");
}
