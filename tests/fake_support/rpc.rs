//! A scripted MCP session: send requests, close stdin, read every reply.

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

use serde_json::{json, Value};

use crate::common::fusible;

/// A `tools/call` request.
pub fn call(id: u64, name: &str, arguments: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "method": "tools/call", "params": {"name": name, "arguments": arguments}})
}

/// A request without parameters.
pub fn request(id: u64, method: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "method": method})
}

/// Runs `fusible mcp --root root`, feeds `requests`, and returns the replies in order.
///
/// Fails when any line on stdout is not a JSON-RPC message.
pub fn session(root: &Path, requests: &[Value]) -> Vec<Value> {
    session_with(root, requests, None)
}

/// [`session`] with `PATH` replaced by `path` when given.
pub fn session_with(root: &Path, requests: &[Value], path: Option<&OsStr>) -> Vec<Value> {
    let mut command = fusible();
    if let Some(path) = path {
        command.env("PATH", path).env_remove("FUSIBLE_BUN");
    }
    let mut child = command
        .args(["mcp", "--root"])
        .arg(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("server should start");
    let mut stdin = child.stdin.take().expect("stdin");
    for request in requests {
        writeln!(stdin, "{request}").expect("write request");
    }
    drop(stdin);
    let output = child
        .wait_with_output()
        .expect("server should exit when stdin closes");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .unwrap_or_else(|e| panic!("stdout must be protocol only: {line}: {e}"))
        })
        .collect()
}

/// The `structuredContent` of a tool reply.
pub fn structured(reply: &Value) -> &Value {
    &reply["result"]["structuredContent"]
}

/// Whether a tool reply is flagged as an error.
pub fn is_error(reply: &Value) -> bool {
    reply["result"]["isError"] == json!(true)
}
