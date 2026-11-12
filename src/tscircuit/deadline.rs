//! The one deadline every tscircuit subprocess runs under (`FUSIBLE_BUILD_TIMEOUT_SECS`).

use std::io::Read;
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::process::{format_command, spawn};
use crate::tscircuit::RunnerError;

/// Runs `cmd` with no stdin and piped output, killing the process after `limit`.
pub(super) fn output_within(cmd: &mut Command, limit: Duration) -> Result<Output, RunnerError> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = spawn(cmd)?;
    collect_output(&mut child, &format_command(cmd), limit)
}

/// Waits for `child` while draining both pipes, then returns its full output.
pub(super) fn collect_output(
    child: &mut Child,
    command: &str,
    limit: Duration,
) -> Result<Output, RunnerError> {
    let stdout_handle = drain(child.stdout.take());
    let stderr_handle = drain(child.stderr.take());
    let status = wait_with_timeout(child, command, limit)?;
    Ok(Output {
        status,
        stdout: stdout_handle.join().unwrap_or_default(),
        stderr: stderr_handle.join().unwrap_or_default(),
    })
}

/// Reads `pipe` to the end on a background thread.
fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut buf);
        }
        buf
    })
}

/// Polls `child` until it exits; kills it after `limit` or on a wait failure.
pub(super) fn wait_with_timeout(
    child: &mut Child,
    command: &str,
    limit: Duration,
) -> Result<ExitStatus, RunnerError> {
    let start = Instant::now();
    let result = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) if start.elapsed() >= limit => break Err(timeout_error(command, limit)),
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(source) => {
                break Err(RunnerError::Io {
                    command: command.to_string(),
                    source,
                })
            }
        }
    };
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result
}

/// Builds the error reported when a tscircuit command runs past `limit`.
fn timeout_error(command: &str, limit: Duration) -> RunnerError {
    RunnerError::CommandFailed {
        command: command.to_string(),
        code: None,
        stderr: format!("timed out after {} seconds", limit.as_secs()),
        help: format!(
            "a cold autoroute of a large board takes minutes; raise FUSIBLE_BUILD_TIMEOUT_SECS \
             (now {}) or set `routingDisabled` on <board>",
            limit.as_secs()
        ),
    }
}

/// Returns the tscircuit deadline: `FUSIBLE_BUILD_TIMEOUT_SECS` when set to a positive integer, else 600 seconds.
pub(super) fn timeout() -> Duration {
    let secs = std::env::var("FUSIBLE_BUILD_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|s| *s > 0)
        .unwrap_or(600);
    Duration::from_secs(secs)
}
