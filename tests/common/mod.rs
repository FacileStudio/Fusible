//! Shared helpers for the subcommand integration tests.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Returns a command for the compiled `fusible` binary.
pub fn fusible() -> Command {
    Command::new(env!("CARGO_BIN_EXE_fusible"))
}

/// Finds `bun` on the real `PATH`.
pub fn bun() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join("bun"))
        .find(|candidate| candidate.is_file())
}

/// Finds `bun`, or panics when `FUSIBLE_REQUIRE_BUN=1` demands it, so CI cannot skip silently.
pub fn require_bun() -> Option<PathBuf> {
    let found = bun();
    if found.is_none() && std::env::var("FUSIBLE_REQUIRE_BUN").as_deref() == Ok("1") {
        panic!("FUSIBLE_REQUIRE_BUN=1 but bun is not on PATH");
    }
    found
}

/// A unique temporary directory removed on drop.
pub struct TestDir {
    pub path: std::path::PathBuf,
}

impl TestDir {
    /// Creates an empty directory unique to this process and call.
    pub fn new(name: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let path = std::env::temp_dir().join(format!("fusible_integration_{pid}_{id}_{name}"));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("failed to create test dir");
        Self { path }
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

pub fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let target = to.join(entry.file_name());
        match entry.file_type().unwrap().is_dir() {
            true => copy_dir(&entry.path(), &target),
            false => drop(std::fs::copy(entry.path(), target).unwrap()),
        }
    }
}

pub fn mech_project(name: &str) -> TestDir {
    let dir = TestDir::new(name);
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/mech");
    for entry in ["fusible.yml", "circuit.json"] {
        std::fs::copy(example.join(entry), dir.path.join(entry)).unwrap();
    }
    copy_dir(&example.join("mech"), &dir.path.join("mech"));
    dir
}

pub fn run_in(dir: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let out = fusible()
        .current_dir(dir)
        .args(args)
        .output()
        .expect("binary should run");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}
