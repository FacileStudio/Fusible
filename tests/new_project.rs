//! `fusible new`: every template checks offline and needs no migration.

mod common;

use std::path::Path;

use common::scaffold::{create, offline};
use common::{copy_dir, fusible, TestDir};

const TEMPLATES: [&str; 4] = ["esp32-s3", "stm32g4", "rp2040", "blank"];

#[test]
fn every_template_passes_check_offline_and_needs_no_migration() {
    let parent = TestDir::new("new_templates");
    for template in TEMPLATES {
        let root = create(&parent, template);
        let check = offline(&root, &["check", "--json"]);
        let report: serde_json::Value = serde_json::from_slice(&check.stdout).unwrap();
        assert_eq!(check.status.code(), Some(0), "{template}: {report}");
        assert_eq!(report["fuse"], "intact");
        let migrate = offline(&root, &["migrate", "--check"]);
        assert_eq!(migrate.status.code(), Some(0), "{template}: {migrate:?}");
    }
}

#[test]
fn mcu_templates_declare_pin_attributes_and_firmware() {
    let parent = TestDir::new("new_attributes");
    for template in ["esp32-s3", "stm32g4", "rp2040"] {
        let root = create(&parent, template);
        let tsx = std::fs::read_to_string(root.join("hardware/index.tsx")).unwrap();
        assert!(tsx.contains("providesVoltage: 3.3"), "{template}");
        assert!(tsx.contains("requiresPower: true"), "{template}");
        assert!(root.join("firmware/Cargo.toml").is_file(), "{template}");
        let manifest = std::fs::read_to_string(root.join("fusible.yml")).unwrap();
        assert!(manifest.contains("generate: enclosure"), "{template}");
    }
}

#[test]
#[ignore = "installs tscircuit from the npm registry"]
fn a_fresh_project_rebuilds_its_circuit_with_its_own_tscircuit() {
    let bun = common::bun().expect("bun on PATH");
    let parent = TestDir::new("new_rebuild");
    let made = fusible()
        .current_dir(&parent.path)
        .args(["new", "fresh", "--template", "stm32g4"])
        .output()
        .expect("binary should run");
    assert_eq!(made.status.code(), Some(0), "{made:?}");
    let root = parent.path.join("fresh");
    std::fs::remove_file(root.join("hardware/dist/index/circuit.json")).unwrap();

    let check = fusible()
        .current_dir(&root)
        .env("FUSIBLE_BUN", bun)
        .env("FUSIBLE_NO_CACHE", "1")
        .arg("check")
        .output()
        .expect("binary should run");

    assert_eq!(check.status.code(), Some(0), "{check:?}");
    assert!(root.join("hardware/dist/index/circuit.json").is_file());
}

#[test]
fn new_refuses_a_used_directory_and_a_bad_name() {
    let parent = TestDir::new("new_refuse");
    std::fs::create_dir_all(parent.path.join("taken")).unwrap();
    std::fs::write(parent.path.join("taken/keep.txt"), "x").unwrap();

    let taken = offline(&parent.path, &["new", "taken", "--no-install"]);
    let bad = offline(&parent.path, &["new", "../up", "--no-install"]);

    assert_eq!(taken.status.code(), Some(2));
    assert_eq!(bad.status.code(), Some(2));
    assert!(!parent.path.join("up").exists());
}

#[test]
fn the_mech_example_needs_no_migration() {
    let dir = TestDir::new("migrate_examples");
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/mech");
    copy_dir(&example, &dir.path);

    let out = offline(&dir.path, &["migrate", "--check"]);

    assert_eq!(out.status.code(), Some(0), "{out:?}");
}
