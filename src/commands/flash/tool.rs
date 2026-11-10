//! The programs a flash command needs, looked up on `PATH` before it runs.
//!
//! A `cargo run` flash goes through the cargo runner: the one inlined with
//! `--config "...runner='<tool> ...'"`, or the `runner` of the crate's `.cargo/config.toml`.

use std::ffi::OsStr;
use std::path::Path;

use crate::tscircuit::detect::find_binary_in_path;

/// The runner `cmd` inlines with `runner='<tool> ...'`.
fn inline_runner(cmd: &str) -> Option<String> {
    let (_, rest) = cmd.split_once("runner='")?;
    rest.split_whitespace().next().map(str::to_string)
}

/// The program of the first `runner = "..."` line of `<dir>/.cargo/config.toml`.
fn config_runner(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join(".cargo/config.toml")).ok()?;
    text.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        let value = value.trim().trim_matches('"');
        (key.trim() == "runner").then(|| value.split_whitespace().next().map(str::to_string))?
    })
}

/// Every program `cmd` runs, run inside `dir`: its own program and, for `cargo run`, the runner.
pub fn required(cmd: &str, dir: Option<&Path>) -> Vec<String> {
    let program = cmd.split_whitespace().find(|word| !word.contains('='));
    let cargo_run = cmd.split_whitespace().take(2).eq(["cargo", "run"]);
    let runner = inline_runner(cmd).or_else(|| cargo_run.then(|| dir.and_then(config_runner))?);
    program
        .map(str::to_string)
        .into_iter()
        .chain(runner)
        .collect()
}

fn found(tool: &str, path: &OsStr) -> bool {
    match Path::new(tool).components().count() > 1 {
        true => Path::new(tool).is_file(),
        false => find_binary_in_path(tool, path).is_some(),
    }
}

/// The first program of `cmd` that `path` does not hold.
pub fn missing(cmd: &str, dir: Option<&Path>, path: &OsStr) -> Option<String> {
    required(cmd, dir)
        .into_iter()
        .find(|tool| !found(tool, path))
}

/// How to install `tool`.
pub fn install_hint(tool: &str) -> String {
    match tool {
        "probe-rs" => "install it with `cargo install probe-rs-tools --locked` \
                       (https://probe.rs/docs/getting-started/installation/)"
            .to_string(),
        "espflash" => "install it with `cargo install espflash --locked` \
                       (https://github.com/esp-rs/espflash)"
            .to_string(),
        "cargo" => "install Rust with rustup (https://rustup.rs)".to_string(),
        _ => format!("install `{tool}` and put it on PATH, or set `flash_cmd`"),
    }
}

#[cfg(test)]
mod tests {
    use super::{missing, required};

    #[test]
    fn a_cargo_run_needs_cargo_and_its_runner() {
        let dir = std::env::temp_dir().join(format!("fusible-flash-tool-{}", std::process::id()));
        std::fs::create_dir_all(dir.join(".cargo")).unwrap();
        let config = "[target.x]\nrunner = \"espflash flash --monitor\"\n";
        std::fs::write(dir.join(".cargo/config.toml"), config).unwrap();

        assert_eq!(
            required("cargo run --release", Some(&dir)),
            ["cargo", "espflash"]
        );
        let inline = "cargo run --config \"target.'cfg(all())'.runner='probe-rs run --chip X'\"";
        assert_eq!(required(inline, None), ["cargo", "probe-rs"]);
        assert_eq!(required("DEFMT_LOG=info west flash", None), ["west"]);
        assert_eq!(
            missing("cargo run --release", Some(&dir), "".as_ref()).as_deref(),
            Some("cargo")
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
