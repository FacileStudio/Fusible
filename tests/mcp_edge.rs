//! `fusible mcp` startup, argument and error edge cases.

mod common;
mod fake_support;

use fake_support::rpc::{call, is_error, request, session, structured};
use fake_support::{git_commit_all, project, INTACT};
use serde_json::{json, Value};

fn initialize() -> Value {
    let info = json!({"name": "t", "version": "0"});
    json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
           "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": info}})
}

fn committed_project() -> common::TestDir {
    let dir = project(INTACT);
    git_commit_all(&dir.path);
    dir
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

#[test]
fn exactly_one_of_patch_or_content_is_required() {
    let dir = committed_project();
    let both = json!({"board": "main", "content": "x", "patch": "y"});
    let neither = json!({"board": "main"});

    let replies = session(
        &dir.path,
        &[call(1, "edit_board", both), call(2, "edit_board", neither)],
    );

    assert!(is_error(&replies[0]) && is_error(&replies[1]));
}

#[test]
fn edit_body_needs_a_code_part() {
    let dir = committed_project();

    let replies = session(
        &dir.path,
        &[call(
            1,
            "edit_body",
            json!({"name": "nothing", "content": "x"}),
        )],
    );

    assert!(is_error(&replies[0]));
}
