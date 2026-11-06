//! The thirteen MCP tools and their registry.

mod args;
mod body_new;
mod bom;
mod build_body;
mod check;
mod edit_board;
mod edit_body;
mod explain_rule;
mod export;
mod firmware_pins;
mod list_components;
mod list_nets;
pub mod outcome;
mod project_info;
mod render;

use serde_json::{json, Value};

use super::rpc::RpcError;
use super::session::Session;
use outcome::Outcome;

type Run = fn(&mut Session, &Value) -> Outcome;

struct Tool {
    name: &'static str,
    description: &'static str,
    schema: fn() -> Value,
    run: Run,
}

macro_rules! tool {
    ($module:ident) => {
        Tool {
            name: stringify!($module),
            description: $module::DESCRIPTION,
            schema: $module::schema,
            run: $module::run,
        }
    };
}

fn registry() -> [Tool; 13] {
    [
        tool!(project_info),
        tool!(check),
        tool!(explain_rule),
        tool!(list_components),
        tool!(list_nets),
        tool!(bom),
        tool!(firmware_pins),
        tool!(edit_board),
        tool!(body_new),
        tool!(edit_body),
        tool!(build_body),
        tool!(export),
        tool!(render),
    ]
}

/// The `tools/list` result.
pub fn list() -> Value {
    let tools: Vec<Value> = registry()
        .iter()
        .map(|t| json!({"name": t.name, "description": t.description, "inputSchema": (t.schema)()}))
        .collect();
    json!({"tools": tools})
}

/// Runs the tool named in `params` and returns the `tools/call` result.
pub fn call(session: &mut Session, params: &Value) -> Result<Value, RpcError> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| RpcError::params("name is required"))?;
    let tool = registry()
        .into_iter()
        .find(|t| t.name == name)
        .ok_or_else(|| RpcError::params(format!("unknown tool: {name}")))?;
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    Ok((tool.run)(session, &arguments).into_result())
}
