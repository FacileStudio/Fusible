//! Installs and runs the embedded bun + occt-wasm generator.

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use super::generator_setup::{prepare_generator, OCCT_WASM_VERSION};
use super::spec::Spec;
use super::BodyError;
use crate::tscircuit::bun_path;

const SCRIPT_TIMEOUT: Duration = Duration::from_secs(120);
const MAX_OUTPUT: u64 = 64 * 1024;

/// Returns the cache directory of the generator, keyed by the occt-wasm version so upgrades reuse the install.
pub(crate) fn generator_dir() -> PathBuf {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .or_else(|| std::env::var_os("LOCALAPPDATA").map(PathBuf::from))
        .unwrap_or_else(std::env::temp_dir)
        .join("fusible")
        .join(format!("body-occt-{OCCT_WASM_VERSION}"))
}

/// Pipes `spec` to the generator, which writes every file listed in `spec.outputs`.
pub(super) fn run_generator(spec: &Spec) -> Result<(), BodyError> {
    println!("🧊 Building body model with OpenCascade...");
    let payload = serde_json::to_vec(spec).map_err(std::io::Error::other)?;
    run_script("generator.ts", &payload)
}

/// Runs the embedded script `script` under bun with `payload` on stdin; stdout is discarded.
///
/// Bun is killed when it runs longer than [`SCRIPT_TIMEOUT`].
pub(crate) fn run_script(script: &str, payload: &[u8]) -> Result<(), BodyError> {
    run_capture(script, payload).map(|_| ())
}

/// Like [`run_script`], and returns what the script wrote to stdout.
pub(crate) fn run_capture(script: &str, payload: &[u8]) -> Result<String, BodyError> {
    let bun = bun_path().ok_or(BodyError::BunMissing)?;
    let dir = generator_dir();
    prepare_generator(&bun, &dir)?;
    let mut child = Command::new(&bun)
        .arg(script)
        .current_dir(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().map(capture);
    let stderr = child.stderr.take().map(capture);
    let wrote = child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(payload);
    let status = wait_or_kill(&mut child, SCRIPT_TIMEOUT);
    let joined = |reader: Option<thread::JoinHandle<String>>| {
        reader.and_then(|r| r.join().ok()).unwrap_or_default()
    };
    let (stdout, stderr) = (joined(stdout), joined(stderr));
    let command = format!("bun {}", dir.join(script).display());
    outcome(status, wrote.is_ok(), command, stderr).map(|()| stdout)
}

/// Maps how bun ended to the result of the script: killed, failed, or done.
fn outcome(
    status: Option<ExitStatus>,
    wrote: bool,
    command: String,
    stderr: String,
) -> Result<(), BodyError> {
    match status {
        None => Err(BodyError::GeneratorTimeout {
            command,
            seconds: SCRIPT_TIMEOUT.as_secs(),
        }),
        Some(status) if status.success() && wrote => Ok(()),
        Some(status) => Err(BodyError::GeneratorFailed {
            command,
            code: status.code(),
            stderr,
        }),
    }
}

/// Reads at most [`MAX_OUTPUT`] bytes of a child pipe on a thread, so the pipe never fills.
fn capture(pipe: impl Read + Send + 'static) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut text = Vec::new();
        let _ = pipe.take(MAX_OUTPUT).read_to_end(&mut text);
        String::from_utf8_lossy(&text).trim().to_string()
    })
}

/// Waits for `child` up to `timeout`; kills it and returns `None` when it is still running.
pub(super) fn wait_or_kill(child: &mut Child, timeout: Duration) -> Option<ExitStatus> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) if start.elapsed() < timeout => thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}
