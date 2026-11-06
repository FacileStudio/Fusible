//! Newline-delimited JSON-RPC over stdin and stdout.

use std::io::{BufRead, Write};

use serde_json::Value;

use super::rpc;
use super::session::Session;

#[cfg(unix)]
fn protocol_out() -> Box<dyn Write> {
    use std::os::fd::AsFd;

    extern "C" {
        fn dup2(old: i32, new: i32) -> i32;
    }
    match std::io::stdout().as_fd().try_clone_to_owned() {
        Ok(saved) => {
            unsafe { dup2(2, 1) };
            Box::new(std::fs::File::from(saved))
        }
        Err(_) => Box::new(std::io::stdout()),
    }
}

#[cfg(windows)]
fn protocol_out() -> Box<dyn Write> {
    use std::os::windows::io::{AsHandle, AsRawHandle};

    extern "system" {
        fn SetStdHandle(id: u32, handle: *mut std::ffi::c_void) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = 0xFFFF_FFF5;
    match std::io::stdout().as_handle().try_clone_to_owned() {
        Ok(saved) => {
            unsafe { SetStdHandle(STD_OUTPUT_HANDLE, std::io::stderr().as_raw_handle()) };
            Box::new(std::fs::File::from(saved))
        }
        Err(_) => Box::new(std::io::stdout()),
    }
}

#[cfg(not(any(unix, windows)))]
fn protocol_out() -> Box<dyn Write> {
    Box::new(std::io::stdout())
}

fn send(out: &mut dyn Write, message: &Value) -> std::io::Result<()> {
    writeln!(out, "{message}")?;
    out.flush()
}

/// Reads requests until stdin closes; stray prints from reused commands go to stderr.
pub fn serve(mut session: Session) -> std::io::Result<()> {
    let mut out = protocol_out();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = rpc::handle_line(&mut session, &line) {
            send(&mut *out, &response)?;
        }
    }
    Ok(())
}
