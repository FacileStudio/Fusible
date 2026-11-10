//! `fusible new` projects run with no tscircuit runner on `PATH`.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Output;

use super::{fusible, TestDir};

/// A `PATH` with the Rust toolchain and the system directories only.
pub fn offline_path() -> OsString {
    let cargo = PathBuf::from(env!("CARGO"));
    let toolchain = cargo.parent().map(Path::to_path_buf).unwrap_or_default();
    let dirs = [toolchain, PathBuf::from("/usr/bin"), PathBuf::from("/bin")];
    std::env::join_paths(dirs).expect("PATH should join")
}

/// Runs `fusible args` in `dir` with [`offline_path`].
pub fn offline(dir: &Path, args: &[&str]) -> Output {
    fusible()
        .current_dir(dir)
        .env("PATH", offline_path())
        .env_remove("FUSIBLE_BUN")
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .args(args)
        .output()
        .expect("binary should run")
}

/// Creates `demo-<template>` under `parent` and returns its root.
pub fn create(parent: &TestDir, template: &str) -> PathBuf {
    let name = format!("demo-{template}");
    let out = offline(
        &parent.path,
        &["new", &name, "--template", template, "--no-install"],
    );
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    parent.path.join(name)
}
