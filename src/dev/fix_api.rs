//! `POST /api/fix`: applies a diagnostic's `set_prop` fix to the TSX source of a served board.
//!
//! Same-origin JSON only, like `/api/run`. The body names the board and the fix as the report
//! prints it: `{"board": "main", "fix": {"set_prop": {"selector", "prop", "value"}}}`. The
//! board's file watcher rebuilds and pushes the new checks.

use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;
use tiny_http::Request;

use super::api::{error_body, respond_json};
use super::run_api::forbidden;
use super::state::State;
use crate::commands::mcp::tsx_prop::set_prop;
use crate::commands::target::is_tsx_target;

const MAX_REQUEST: u64 = 16 * 1024;

#[derive(Deserialize)]
struct SetProp {
    selector: String,
    prop: String,
    value: Value,
}

#[derive(Deserialize)]
struct FixBody {
    set_prop: SetProp,
}

#[derive(Deserialize)]
struct FixRequest {
    board: Option<String>,
    fix: FixBody,
}

/// Writes the fix into `entry`; the error is the status and message to answer with.
fn apply(entry: &Path, fix: &FixBody) -> Result<(), (u16, String)> {
    if !is_tsx_target(entry) {
        return Err((422, "fixes apply to TSX boards only".into()));
    }
    let source = std::fs::read_to_string(entry).map_err(|e| (500, e.to_string()))?;
    let SetProp {
        selector,
        prop,
        value,
    } = &fix.set_prop;
    let text = set_prop(&source, selector, prop, value).map_err(|e| (422, e))?;
    std::fs::write(entry, text).map_err(|e| (500, e.to_string()))
}

fn parse(request: &mut Request) -> Result<FixRequest, String> {
    let mut body = String::new();
    request
        .as_reader()
        .take(MAX_REQUEST)
        .read_to_string(&mut body)
        .map_err(|_| "unreadable request body".to_string())?;
    serde_json::from_str(&body).map_err(|e| format!("expected a board and a set_prop fix: {e}"))
}

/// Applies the requested fix after the origin, method and content-type checks.
pub(super) fn serve(mut request: Request, state: &State, port: u16) {
    if forbidden(&request, port) {
        return respond_json(request, 403, error_body("forbidden"));
    }
    let wanted = match parse(&mut request) {
        Ok(wanted) => wanted,
        Err(message) => return respond_json(request, 400, error_body(&message)),
    };
    let Some(slot) = state.slot(wanted.board.as_deref()) else {
        return respond_json(request, 404, error_body("unknown board"));
    };
    match apply(&slot.entry, &wanted.fix) {
        Ok(()) => respond_json(
            request,
            200,
            serde_json::json!({"applied": true}).to_string(),
        ),
        Err((status, message)) => respond_json(request, status, error_body(&message)),
    }
}

#[cfg(test)]
mod tests {
    use super::{apply, FixBody};

    fn fix(selector: &str) -> FixBody {
        let text = format!(r#"{{"set_prop":{{"selector":"{selector}","prop":"pcbX","value":3}}}}"#);
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn a_set_prop_fix_rewrites_the_tsx_tag() {
        let dir = std::env::temp_dir().join(format!("fusible_fix_api_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let entry = dir.join("board.tsx");
        std::fs::write(&entry, "<board><resistor name=\"R1\" /></board>\n").unwrap();

        apply(&entry, &fix("R1")).unwrap();
        let missing = apply(&entry, &fix("R9"));
        let json = apply(&dir.join("circuit.json"), &fix("R1"));

        let text = std::fs::read_to_string(&entry).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(text, "<board><resistor name=\"R1\" pcbX={3} /></board>\n");
        assert_eq!(missing.unwrap_err().0, 422);
        assert_eq!(json.unwrap_err().0, 422);
    }
}
