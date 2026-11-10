//! MCP `apply_fix`: a fix written to the TSX through the edit safety rules.
#![cfg(unix)]

mod common;
mod fake_support;

use std::ffi::OsString;

use common::TestDir;
use fake_support::git_commit_all;
use fake_support::rpc::{call, is_error, session_with, structured};
use serde_json::json;

const BOARD: &str = "export default () => (\n  <board width=\"10mm\" height=\"10mm\">\n    \
<resistor name=\"R1\" resistance=\"1k\" footprint=\"0402\" />\n  </board>\n)\n";

fn tsx_project() -> TestDir {
    let dir = TestDir::new("mcp_fix_tsx");
    std::fs::write(dir.path.join("board.tsx"), BOARD).unwrap();
    let manifest = "name: fix\nboards:\n  main:\n    entry: board.tsx\n";
    std::fs::write(dir.path.join("fusible.yml"), manifest).unwrap();
    git_commit_all(&dir.path);
    dir
}

#[test]
fn apply_fix_sets_the_prop_on_the_tsx_tag() {
    let dir = tsx_project();
    let fix = json!({"set_prop": {"selector": ".R1", "prop": "pcbX", "value": 2.5}});
    let offline = OsString::from("/usr/bin:/bin");

    let calls = [
        call(1, "apply_fix", json!({"board": "main", "fix": fix.clone()})),
        call(
            2,
            "apply_fix",
            json!({"board": "main", "fix": fix, "dry_run": false}),
        ),
        call(
            3,
            "apply_fix",
            json!({"board": "main", "fix": {"note": "x"}}),
        ),
    ];
    let replies = session_with(&dir.path, &calls, Some(&offline));

    let preview = structured(&replies[0]);
    assert_eq!(preview["applied"], false);
    assert!(
        preview["diff"].as_str().unwrap().contains("pcbX={2.5}"),
        "{preview}"
    );
    assert_eq!(structured(&replies[1])["applied"], true);
    let source = std::fs::read_to_string(dir.path.join("board.tsx")).unwrap();
    assert!(
        source.contains("footprint=\"0402\" pcbX={2.5} />"),
        "{source}"
    );
    assert!(is_error(&replies[2]));
}
