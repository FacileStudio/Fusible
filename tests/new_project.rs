//! `fusible new`: every template checks offline, builds, and needs no migration.

mod common;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Output;

use common::{copy_dir, fusible, TestDir};

const TEMPLATES: [&str; 4] = ["esp32-s3", "stm32g4", "rp2040", "blank"];

fn offline_path() -> OsString {
    let cargo = PathBuf::from(env!("CARGO"));
    let toolchain = cargo.parent().map(Path::to_path_buf).unwrap_or_default();
    let dirs = [toolchain, PathBuf::from("/usr/bin"), PathBuf::from("/bin")];
    std::env::join_paths(dirs).expect("PATH should join")
}

fn offline(dir: &Path, args: &[&str]) -> Output {
    fusible()
        .current_dir(dir)
        .env("PATH", offline_path())
        .env_remove("FUSIBLE_BUN")
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .args(args)
        .output()
        .expect("binary should run")
}

fn create(parent: &TestDir, template: &str) -> PathBuf {
    let name = format!("demo-{template}");
    let out = offline(
        &parent.path,
        &["new", &name, "--template", template, "--no-install"],
    );
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    parent.path.join(name)
}

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
fn the_esp32_template_builds_its_firmware() {
    let parent = TestDir::new("new_build");
    let root = create(&parent, "esp32-s3");

    let out = offline(&root, &["build"]);

    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let pins = std::fs::read_to_string(root.join("firmware/src/pins.rs")).unwrap();
    assert!(pins.contains("pub const GPIO0"), "{pins}");
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
