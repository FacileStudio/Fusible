//! `share nuage` against a fake `nuage` binary.

#![cfg(unix)]

mod common;
mod fake_support;

use common::TestDir;
use fake_support::{fake_path, log_lines, project, BLOWN, INTACT};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn share(dir: &TestDir, sync: Option<&TestDir>, extra: &[&str]) -> std::process::Output {
    let log = dir.path.join("calls.log");
    let mut command = common::fusible();
    command
        .current_dir(&dir.path)
        .env("PATH", fake_path())
        .env("FAKE_LOG", &log);
    if let Some(sync) = sync {
        command.env("FAKE_SYNC_DIR", &sync.path);
    }
    command
        .args(["share", "nuage", "--space", "hardware", "--preset", "docs"])
        .args(extra)
        .output()
        .expect("binary should run")
}

#[test]
fn the_export_lands_in_the_sync_folder_and_nuage_syncs() {
    let dir = project(INTACT);
    let sync = TestDir::new("sync");

    let out = share(&dir, Some(&sync), &[]);

    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let dest = sync.path.join("Fusible/mcp-fixture").join(VERSION);
    assert!(dest.join("pcb.svg").is_file());
    let calls = log_lines(&dir.path.join("calls.log"));
    assert_eq!(calls, ["spaces list --json", "sync --json"]);
}

#[test]
fn link_prints_the_share_url() {
    let dir = project(INTACT);
    let sync = TestDir::new("sync");

    let out = share(&dir, Some(&sync), &["--link", "--expires", "7d"]);

    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("https://nuage.test/s/abc"));
    let calls = log_lines(&dir.path.join("calls.log"));
    assert_eq!(
        calls.last().unwrap(),
        &format!("share Fusible/mcp-fixture/{VERSION} --json --expires 7d")
    );
}

#[test]
fn a_space_without_a_sync_folder_exits_2() {
    let dir = project(INTACT);

    let out = share(&dir, None, &[]);

    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("is not synced on this machine"));
}

#[test]
fn a_blown_fuse_copies_nothing_and_exits_1() {
    let dir = project(BLOWN);
    let sync = TestDir::new("sync");

    let out = share(&dir, Some(&sync), &[]);

    assert_eq!(out.status.code(), Some(1));
    assert!(!sync.path.join("Fusible").exists());
}

#[test]
fn an_occupied_folder_needs_force() {
    let dir = project(INTACT);
    let sync = TestDir::new("sync");
    let dest = sync.path.join("Fusible/mcp-fixture").join(VERSION);
    std::fs::create_dir_all(&dest).unwrap();
    std::fs::write(dest.join("old.txt"), "x").unwrap();

    let refused = share(&dir, Some(&sync), &[]);
    let forced = share(&dir, Some(&sync), &["--force"]);

    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(forced.status.code(), Some(0));
    assert!(dest.join("pcb.svg").is_file());
    assert!(!dest.join("old.txt").exists());
}

#[test]
fn a_failed_share_link_still_exits_0_with_a_hint() {
    let dir = project(INTACT);
    let sync = TestDir::new("sync");
    let log = dir.path.join("calls.log");

    let out = common::fusible()
        .current_dir(&dir.path)
        .env("PATH", fake_path())
        .env("FAKE_LOG", &log)
        .env("FAKE_SYNC_DIR", &sync.path)
        .env("FAKE_SHARE_FAIL", "1")
        .args([
            "share", "nuage", "--space", "hardware", "--preset", "docs", "--link",
        ])
        .output()
        .expect("binary should run");

    assert_eq!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("files are synced; create the link from the Nuage app"));
}

#[test]
fn a_manifest_name_that_escapes_the_sync_folder_exits_2() {
    let dir = project(INTACT);
    let sync = TestDir::new("sync");
    let manifest = dir.path.join("fusible.yml");
    let text = std::fs::read_to_string(&manifest).unwrap();
    let rewritten: Vec<String> = text
        .lines()
        .map(|l| match l.starts_with("name:") {
            true => "name: ../../escape".to_string(),
            false => l.to_string(),
        })
        .collect();
    std::fs::write(&manifest, rewritten.join("\n") + "\n").unwrap();

    let out = share(&dir, Some(&sync), &[]);

    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unsafe project name"));
    assert!(!sync.path.join("Fusible").exists());
}
