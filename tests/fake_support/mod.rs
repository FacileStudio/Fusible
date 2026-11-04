//! Helpers for the tests that run `fusible` against fake `opus` and `nuage` binaries.

#![allow(dead_code)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::common::{fusible, TestDir};

pub const INTACT: &str = "examples/circuit.json";
pub const BLOWN: &str = "examples/circuit-floating.json";

/// Absolute path of a file in the repository.
pub fn repo(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

/// `PATH` with the fake binaries first.
pub fn fake_path() -> OsString {
    let mut dirs = vec![repo("tests/fixtures/bin")];
    dirs.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    std::env::join_paths(dirs).expect("PATH should join")
}

/// A temp project holding the MCP fixture manifest and the given circuit as `circuit.json`.
pub fn project(circuit: &str) -> TestDir {
    let dir = TestDir::new("fake_project");
    std::fs::copy(
        repo("tests/fixtures/mcp/fusible.yml"),
        dir.path.join("fusible.yml"),
    )
    .expect("copy manifest");
    std::fs::copy(repo(circuit), dir.path.join("circuit.json")).expect("copy circuit");
    dir
}

/// Runs `fusible` in `dir` with the fake binaries on `PATH` and calls logged to `log`.
pub fn run_fusible(dir: &Path, log: &Path, args: &[&str]) -> Output {
    run_with(dir, fake_path(), log, args)
}

/// Runs `fusible` in `dir` with an explicit `PATH`.
pub fn run_with(dir: &Path, path: OsString, log: &Path, args: &[&str]) -> Output {
    fusible()
        .current_dir(dir)
        .env("PATH", path)
        .env("FAKE_LOG", log)
        .args(args)
        .output()
        .expect("binary should run")
}

/// Lines the fake binaries logged, or none when nothing ran.
pub fn log_lines(log: &Path) -> Vec<String> {
    std::fs::read_to_string(log)
        .map(|text| text.lines().map(str::to_string).collect())
        .unwrap_or_default()
}

/// Turns `dir` into a git repository with one commit holding everything in it.
pub fn git_commit_all(dir: &Path) {
    let git = |args: &[&str]| {
        let status = Command::new("git")
            .current_dir(dir)
            .args(args)
            .status()
            .expect("git should run");
        assert!(status.success(), "git {args:?} failed");
    };
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&[
        "-c",
        "user.name=t",
        "-c",
        "user.email=t@t",
        "-c",
        "commit.gpgsign=false",
        "commit",
        "-q",
        "--no-verify",
        "-m",
        "init",
    ]);
}
