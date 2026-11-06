//! MCP resources: the manifest and the Circuit JSON of each board.

use serde_json::{json, Value};

use super::report;
use super::rpc::RpcError;
use super::session::Session;
use crate::commands::check::load_circuit_source;

const PROJECT_URI: &str = "fusible://project";

fn board_uri(name: &str) -> String {
    format!("fusible://board/{name}/circuit.json")
}

/// Lists `fusible://project` and one circuit per board.
pub fn list(session: &Session) -> Result<Value, RpcError> {
    let mut items = Vec::new();
    if session.manifest.is_some() {
        items.push(json!({"uri": PROJECT_URI, "name": "project", "mimeType": "application/yaml"}));
    }
    for board in report::select(None).unwrap_or_default() {
        let name = report::board_name(&board);
        let item = json!({"uri": board_uri(&name), "name": name, "mimeType": "application/json"});
        items.push(item);
    }
    Ok(json!({"resources": items}))
}

fn read_text(session: &Session, uri: &str) -> Result<(&'static str, String), String> {
    if uri == PROJECT_URI {
        let path = session
            .manifest
            .as_ref()
            .ok_or("the project has no manifest")?;
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        return Ok(("application/yaml", text));
    }
    let name = uri
        .strip_prefix("fusible://board/")
        .and_then(|rest| rest.strip_suffix("/circuit.json"))
        .ok_or_else(|| format!("unknown resource: {uri}"))?;
    let boards = report::select(Some(name))?;
    let (_, text) = load_circuit_source(&boards[0].entry).map_err(|e| e.to_string())?;
    Ok(("application/json", text))
}

/// Returns the text behind one resource URI.
pub fn read(session: &Session, params: &Value) -> Result<Value, RpcError> {
    let uri = params
        .get("uri")
        .and_then(Value::as_str)
        .ok_or_else(|| RpcError::params("uri is required"))?;
    let (mime, text) = read_text(session, uri).map_err(RpcError::params)?;
    Ok(json!({"contents": [{"uri": uri, "mimeType": mime, "text": text}]}))
}
