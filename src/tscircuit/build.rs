//! `build` invocation: drains output pipes, enforces a timeout, and locates fresh Circuit JSON.

use std::path::{Path, PathBuf};
use std::process::{Child, Output, Stdio};
use std::time::{Duration, SystemTime};

use super::deadline::{collect_output, timeout};
use super::process::{self, extract_output_details, format_command};
use super::{command, output, resolve_hardware_context, Runner, RunnerError};

const CACHE_KIND: &str = "build";

/// A spawned build process with its display string, its launch time, and the
/// modification time of the Circuit JSON that was on disk before it ran.
struct SpawnedBuild {
    child: Child,
    command: String,
    started: SystemTime,
    before: Option<SystemTime>,
}

impl Runner {
    /// Runs the tscircuit build command to produce circuit json.
    pub fn build(&self, file: &Path, out_file: Option<&Path>) -> Result<PathBuf, RunnerError> {
        let cache = self.cache_for(file);
        if let Some(hit) = cache.as_ref().and_then(|c| c.get(CACHE_KIND)) {
            let key = cache.as_ref().map_or("", |c| c.short_key());
            eprintln!("⚡ Sources unchanged ({key}), reusing the cached build");
            return output::deliver(hit, out_file);
        }
        let mut spawned = self.spawn_build(file, true)?;
        let mut out = collect_output(&mut spawned.child, &spawned.command, timeout())?;
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() && stderr.contains("unknown option '--ignore-errors'") {
            spawned = self.spawn_build(file, false)?;
            out = collect_output(&mut spawned.child, &spawned.command, timeout())?;
        }
        let produced = fresh_circuit_json(file, &out, &spawned);

        if !out.status.success() && produced.is_err() {
            let details = extract_output_details(&out);
            return Err(RunnerError::CommandFailed {
                command: spawned.command,
                code: out.status.code(),
                stderr: details.clone(),
                help: details,
            });
        }

        let produced_path = produced?;
        if let Some(cache) = &cache {
            cache.put(CACHE_KIND, &produced_path);
        }
        output::deliver(produced_path, out_file)
    }

    /// Spawns the build command with piped output in the hardware workspace.
    fn spawn_build(&self, file: &Path, ignore_errors: bool) -> Result<SpawnedBuild, RunnerError> {
        let (working_dir, target_file) = resolve_hardware_context(file);
        let mut cmd = command::build_command(&self.kind, &target_file, ignore_errors);
        if let Some(ref dir) = working_dir {
            cmd.current_dir(dir);
        }
        cmd.stdin(Stdio::null());
        cmd.env("CI", "1");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let command = format_command(&cmd);
        let started = SystemTime::now() - Duration::from_secs(2);
        let before = output::resolve_circuit_json(file, "")
            .ok()
            .and_then(|path| std::fs::metadata(path).ok()?.modified().ok());
        let child = process::spawn(&mut cmd)?;
        Ok(SpawnedBuild {
            child,
            command,
            started,
            before,
        })
    }
}

/// Locates the produced Circuit JSON and rejects it if it predates the build or is
/// the untouched file that was there before it.
fn fresh_circuit_json(
    file: &Path,
    out: &Output,
    build: &SpawnedBuild,
) -> Result<PathBuf, RunnerError> {
    let stdout_str = String::from_utf8_lossy(&out.stdout);
    let path = output::resolve_circuit_json(file, &stdout_str)?;
    let fresh = std::fs::metadata(&path)
        .and_then(|m| m.modified())
        .is_ok_and(|modified| modified >= build.started && Some(modified) != build.before);
    if fresh {
        Ok(path)
    } else {
        Err(RunnerError::OutputNotFound { path })
    }
}
