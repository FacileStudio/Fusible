//! `fusible mcp`: protocol, tools and printed configuration.

mod common;
mod fake_support;

use fake_support::rpc::{call, is_error, request, session, structured};
use fake_support::{git_commit_all, project, BLOWN, INTACT};
use serde_json::{json, Value};

const TOOLS: [&str; 13] = [
    "project_info",
    "check",
    "explain_rule",
    "list_components",
    "list_nets",
    "bom",
    "firmware_pins",
    "edit_board",
    "body_new",
    "edit_body",
    "build_body",
    "export",
    "render",
];

fn initialize() -> Value {
    json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
           "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "t", "version": "0"}}})
}

#[test]
fn initialize_and_tools_list_name_the_thirteen_tools() {
    let dir = project(INTACT);
    let replies = session(&dir.path, &[initialize(), request(2, "tools/list")]);

    assert_eq!(replies[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(replies[0]["result"]["serverInfo"]["name"], "fusible");
    let names: Vec<&str> = replies[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, TOOLS);
    for tool in replies[1]["result"]["tools"].as_array().unwrap() {
        assert_eq!(tool["inputSchema"]["type"], "object");
        assert_eq!(tool["inputSchema"]["additionalProperties"], false);
    }
}

#[test]
fn an_unknown_protocol_version_falls_back_and_notifications_get_no_reply() {
    let dir = project(INTACT);
    let mut init = initialize();
    init["params"]["protocolVersion"] = json!("1999-01-01");
    let note = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});

    let replies = session(&dir.path, &[init, note, request(2, "ping")]);

    assert_eq!(replies.len(), 2);
    assert_eq!(replies[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(replies[1]["result"], json!({}));
}

#[test]
fn an_unknown_method_is_32601_and_an_unknown_tool_is_32602() {
    let dir = project(INTACT);
    let replies = session(
        &dir.path,
        &[
            request(1, "nope/nothing"),
            call(2, "no_such_tool", json!({})),
        ],
    );

    assert_eq!(replies[0]["error"]["code"], -32601);
    assert_eq!(replies[1]["error"]["code"], -32602);
}

#[test]
fn check_reports_the_fuse_for_both_verdicts() {
    let intact = project(INTACT);
    let blown = project(BLOWN);

    let ok = session(&intact.path, &[call(1, "check", json!({}))]);
    let bad = session(&blown.path, &[call(1, "check", json!({"board": "main"}))]);

    assert_eq!(structured(&ok[0])["fuse"], "intact");
    assert_eq!(structured(&bad[0])["fuse"], "blown");
    let first = &structured(&bad[0])["boards"][0]["diagnostics"][0];
    assert_eq!(first["rule"], "unconnected_net");
    assert_eq!(first["severity"], "error");
}

#[test]
fn reading_tools_describe_the_circuit() {
    let dir = project(INTACT);
    let replies = session(
        &dir.path,
        &[
            call(1, "list_components", json!({"board": "main"})),
            call(2, "list_nets", json!({"board": "main"})),
            call(3, "explain_rule", json!({"rule": "unconnected_net"})),
            call(4, "explain_rule", json!({"rule": "made_up"})),
            call(5, "bom", json!({})),
            call(6, "check", json!({"board": "ghost"})),
        ],
    );

    assert_eq!(
        structured(&replies[0])["components"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(structured(&replies[1])["nets"].as_array().unwrap().len(), 2);
    assert_eq!(structured(&replies[2])["id"], "unconnected_net");
    assert!(is_error(&replies[3]));
    assert!(structured(&replies[4])["lines"].as_array().unwrap().len() >= 2);
    assert!(is_error(&replies[5]));
}

#[test]
fn project_info_and_resources_see_the_manifest() {
    let dir = project(INTACT);
    git_commit_all(&dir.path);
    let read = json!({"jsonrpc": "2.0", "id": 3, "method": "resources/read",
                      "params": {"uri": "fusible://board/main/circuit.json"}});

    let replies = session(
        &dir.path,
        &[
            call(1, "project_info", json!({})),
            request(2, "resources/list"),
            read,
        ],
    );

    let info = structured(&replies[0]);
    assert_eq!(info["name"], "mcp-fixture");
    assert_eq!(info["boards"][0]["entry"], "circuit.json");
    assert_eq!(info["git"]["dirty"], false);
    let uris: Vec<&str> = replies[1]["result"]["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["uri"].as_str().unwrap())
        .collect();
    assert_eq!(
        uris,
        ["fusible://project", "fusible://board/main/circuit.json"]
    );
    assert!(replies[2]["result"]["contents"][0]["text"]
        .as_str()
        .unwrap()
        .contains("source_component"));
}

#[test]
fn export_obeys_the_fuse_and_stdout_stays_protocol_only() {
    let intact = project(INTACT);
    let blown = project(BLOWN);
    let docs = json!({"preset": "docs"});

    let ok = session(&intact.path, &[call(1, "export", docs.clone())]);
    let bad = session(&blown.path, &[call(1, "export", docs)]);

    let files = structured(&ok[0])["files"].as_array().unwrap().clone();
    assert!(files.iter().any(|f| f == "dist/docs/pcb.svg"), "{files:?}");
    assert!(is_error(&bad[0]));
    assert_eq!(structured(&bad[0])["fuse"], "blown");
    assert!(!blown.path.join("dist").exists());
}

#[test]
fn render_returns_svg_and_the_stubbed_body_tool_says_so() {
    let dir = project(INTACT);
    let replies = session(
        &dir.path,
        &[
            call(1, "render", json!({"board": "main", "view": "schematic"})),
            call(2, "body_new", json!({"name": "x"})),
        ],
    );

    assert!(structured(&replies[0])["svg"]
        .as_str()
        .unwrap()
        .contains("<svg"));
    assert!(is_error(&replies[1]));
    assert!(structured(&replies[1])["error"]
        .as_str()
        .unwrap()
        .contains("needs Fusible 1.0 bodies"));
}

#[test]
fn print_config_matches_the_documented_snippets() {
    let run = |client: &str| {
        let out = common::fusible()
            .args(["mcp", "--print-config", client])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(0));
        String::from_utf8(out.stdout).unwrap()
    };

    assert_eq!(
        run("bulle"),
        "sources:\n  mcp:\n    fusible:\n      command: fusible\n      args: [mcp]\n"
    );
    assert_eq!(run("claude"), "claude mcp add fusible -- fusible mcp\n");
    assert_eq!(
        run("codex"),
        "[mcp_servers.fusible]\ncommand = \"fusible\"\nargs = [\"mcp\"]\n"
    );
}

#[test]
fn a_root_without_a_manifest_still_starts() {
    let empty = common::TestDir::new("no_manifest");

    let replies = session(&empty.path, &[call(1, "project_info", json!({}))]);

    assert!(structured(&replies[0])["manifest"].is_null());
}
