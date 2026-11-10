//! `fusible flash`, `fusible bringup` and `firmware.outputs` end to end.

#![cfg(unix)]

mod common;

use std::path::Path;

use common::{run_in, TestDir};

fn project(name: &str, circuit: &str, firmware: &str) -> TestDir {
    let dir = TestDir::new(name);
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(circuit);
    std::fs::copy(example, dir.path.join("circuit.json")).unwrap();
    let boards = "boards:\n  main: { circuit_json: circuit.json, mcu: stm32g431 }\n";
    let manifest = format!("{boards}firmware:\n  app: {{ board: main, {firmware} }}\n");
    std::fs::write(dir.path.join("fusible.yml"), manifest).unwrap();
    dir
}

#[test]
fn flash_runs_flash_cmd_once_the_fuse_is_intact() {
    let dir = project("flash_ok", "circuit.json", "flash_cmd: touch flashed");
    let (code, stdout, stderr) = run_in(&dir.path, &["flash"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert!(dir.path.join("flashed").exists());
}

#[test]
fn a_failing_flash_cmd_is_a_tool_error() {
    let dir = project("flash_fail", "circuit.json", "flash_cmd: \"false\"");
    let (code, _, stderr) = run_in(&dir.path, &["flash", "app"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.contains("flash command failed"), "{stderr}");
}

#[test]
fn a_blown_fuse_exits_one_before_the_flash_cmd_runs() {
    let dir = project(
        "flash_blown",
        "circuit-flottant.json",
        "flash_cmd: touch flashed",
    );
    let (code, _, stderr) = run_in(&dir.path, &["flash"]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(!dir.path.join("flashed").exists());
}

#[test]
fn flash_without_a_command_or_framework_is_a_tool_error() {
    let dir = project("flash_none", "circuit.json", "path: fw");
    let (code, _, stderr) = run_in(&dir.path, &["flash"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.contains("no flash command"), "{stderr}");
}

#[test]
fn bringup_writes_the_sheet_into_dist() {
    let dir = project("bringup", "circuit.json", "path: fw");
    let (code, stdout, stderr) = run_in(&dir.path, &["bringup"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    let sheet = std::fs::read_to_string(dir.path.join("dist/bringup.md")).unwrap();
    assert!(sheet.contains("## Power rails"), "{sheet}");
    assert!(sheet.contains("| 1 | +3V3 | 3.3 V |"), "{sheet}");
}

#[test]
fn check_write_bindings_writes_every_declared_output() {
    let dir = project(
        "outputs",
        "circuit.json",
        "path: fw, outputs: [rust, c, zephyr]",
    );
    let (code, stdout, stderr) = run_in(&dir.path, &["check", "--write-bindings"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    for file in ["fw/src/pins.rs", "fw/include/pins.h", "fw/app.overlay"] {
        assert!(dir.path.join(file).exists(), "{file} missing: {stdout}");
    }
    let (code, _, _) = run_in(&dir.path, &["check"]);
    assert_eq!(code, Some(0));
}
