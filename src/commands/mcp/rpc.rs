//! JSON-RPC 2.0 envelope and method routing.

use serde_json::{json, Value};

use super::session::Session;
use super::{resources, tools};

const VERSIONS: [&str; 3] = ["2025-11-25", "2025-06-18", "2025-03-26"];
const DEFAULT_VERSION: &str = "2025-06-18";

const INSTRUCTIONS: &str = "Fusible checks hardware designs. Call check after every edit. \
A blown fuse means the design breaks a rule: read each violation's message, standard and fix, \
change the board or part, and check again. export refuses to run while the fuse is blown. \
Edits are file changes in a git repository; nothing is committed for you.";

/// A JSON-RPC error: a code from the specification and a message.
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

impl RpcError {
    /// Builds an error with the given code.
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// The method does not exist (-32601).
    pub fn method(name: &str) -> Self {
        Self::new(-32601, format!("method not found: {name}"))
    }

    /// The parameters are invalid (-32602).
    pub fn params(message: impl Into<String>) -> Self {
        Self::new(-32602, message)
    }
}

fn initialize(params: &Value) -> Value {
    let asked = params.get("protocolVersion").and_then(Value::as_str);
    let version = asked
        .filter(|v| VERSIONS.contains(v))
        .unwrap_or(DEFAULT_VERSION);
    json!({
        "protocolVersion": version,
        "capabilities": {"tools": {"listChanged": false}, "resources": {"listChanged": false}},
        "serverInfo": {"name": "fusible", "version": env!("CARGO_PKG_VERSION")},
        "instructions": INSTRUCTIONS,
    })
}

fn route(session: &mut Session, method: &str, params: &Value) -> Result<Value, RpcError> {
    match method {
        "initialize" => Ok(initialize(params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(tools::list()),
        "tools/call" => tools::call(session, params),
        "resources/list" => resources::list(session),
        "resources/read" => resources::read(session, params),
        other => Err(RpcError::method(other)),
    }
}

fn failure(id: Value, error: RpcError) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": error.code, "message": error.message}})
}

fn handle(session: &mut Session, message: &Value) -> Option<Value> {
    let id = message.get("id")?.clone();
    let Some(method) = message.get("method").and_then(Value::as_str) else {
        return Some(failure(
            id,
            RpcError::new(-32600, "invalid request: no method"),
        ));
    };
    let params = message.get("params").cloned().unwrap_or(Value::Null);
    Some(match route(session, method, &params) {
        Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
        Err(error) => failure(id, error),
    })
}

/// Answers one line of input; notifications and blank messages yield no response.
pub fn handle_line(session: &mut Session, line: &str) -> Option<Value> {
    match serde_json::from_str::<Value>(line) {
        Ok(message) => handle(session, &message),
        Err(err) => Some(failure(
            Value::Null,
            RpcError::new(-32700, format!("parse error: {err}")),
        )),
    }
}
