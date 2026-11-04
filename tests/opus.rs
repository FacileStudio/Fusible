//! `check --opus` and `bom --opus` against a fake `opus` binary.

#![cfg(unix)]

mod common;
mod fake_support;

use fake_support::{log_lines, project, run_fusible, run_with, BLOWN, INTACT};

#[test]
fn a_blown_fuse_logs_one_task_and_a_rerun_logs_none() {
    let dir = project(BLOWN);
    let log = dir.path.join("calls.log");

    let first = run_fusible(&dir.path, &log, &["check", "--opus"]);
    assert_eq!(first.status.code(), Some(1));
    let calls = log_lines(&log);
    assert_eq!(
        calls.iter().filter(|l| l.starts_with("task add")).count(),
        1,
        "{calls:?}"
    );
    assert!(dir.path.join(".fusible/opus.json").is_file());

    let second = run_fusible(&dir.path, &log, &["check", "--opus"]);
    assert_eq!(second.status.code(), Some(1));
    assert_eq!(
        log_lines(&log).len(),
        calls.len(),
        "dedup should skip the known key"
    );
}

#[test]
fn the_task_carries_the_labels_and_the_project() {
    let dir = project(BLOWN);
    let log = dir.path.join("calls.log");

    run_fusible(
        &dir.path,
        &log,
        &["check", "--opus", "--opus-project", "hardware"],
    );

    let call = log_lines(&log).join(" ");
    for part in [
        "--label fusible",
        "--label fuse",
        "--priority high",
        "--project hardware",
        "--json",
    ] {
        assert!(call.contains(part), "missing {part} in {call}");
    }
}

#[test]
fn a_missing_opus_keeps_the_exit_code_and_prints_the_hint() {
    let dir = project(BLOWN);
    let log = dir.path.join("calls.log");
    let empty = common::TestDir::new("empty_path");

    let out = run_with(
        &dir.path,
        empty.path.clone().into_os_string(),
        &log,
        &["check", "--opus"],
    );

    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("opus CLI not found; install it with: facile install opus"));
}

#[test]
fn an_intact_fuse_creates_no_task() {
    let dir = project(INTACT);
    let log = dir.path.join("calls.log");

    let out = run_fusible(&dir.path, &log, &["check", "--opus"]);

    assert_eq!(out.status.code(), Some(0));
    assert!(log_lines(&log).is_empty());
}

#[test]
fn bom_opus_opens_a_task_for_an_unpicked_part() {
    let dir = project(INTACT);
    std::fs::write(
        dir.path.join("bom.yml"),
        "parts:\n  capacitor: {picked: false}\n",
    )
    .unwrap();
    let log = dir.path.join("calls.log");

    let out = run_fusible(&dir.path, &log, &["bom", "--opus"]);

    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let calls = log_lines(&log);
    assert_eq!(calls.len(), 1, "{calls:?}");
    assert!(
        calls[0].contains("Buy 1") && calls[0].contains("--label bom"),
        "{calls:?}"
    );
}
