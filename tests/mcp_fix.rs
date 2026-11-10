//! MCP `apply_fix` and `waive`: fixes and waivers written through the edit safety rules.

mod common;
mod fake_support;

use std::ffi::OsString;

use common::TestDir;
use fake_support::rpc::{call, is_error, session, session_with, structured};
use fake_support::{git_commit_all, project};
use serde_json::json;

const NO_DECOUPLING: &str = "tests/fixtures/waivers/no-decoupling.json";

const BOARD: &str = "export default () => (\n  <board width=\"10mm\" height=\"10mm\">\n    \
<resistor name=\"R1\" resistance=\"1k\" footprint=\"0402\" />\n  </board>\n)\n";

fn commented_project() -> TestDir {
    let dir = project(NO_DECOUPLING);
    let manifest = dir.path.join("fusible.yml");
    let text = std::fs::read_to_string(&manifest).unwrap();
    std::fs::write(&manifest, format!("# keep this comment\n{text}")).unwrap();
    git_commit_all(&dir.path);
    dir
}

fn tsx_project() -> TestDir {
    let dir = TestDir::new("mcp_fix_tsx");
    std::fs::write(dir.path.join("board.tsx"), BOARD).unwrap();
    let manifest = "name: fix\nboards:\n  main:\n    entry: board.tsx\n";
    std::fs::write(dir.path.join("fusible.yml"), manifest).unwrap();
    git_commit_all(&dir.path);
    dir
}

#[test]
fn waive_appends_a_reasoned_waiver_and_moves_the_diagnostic() {
    let dir = commented_project();
    let args = json!({"rule": "decoupling_capacitor", "match": "U1.VDD",
                      "reason": "bulk capacitor on the carrier", "dry_run": false});

    let replies = session(
        &dir.path,
        &[call(1, "waive", args), call(2, "check", json!({}))],
    );

    assert_eq!(structured(&replies[0])["applied"], true, "{:?}", replies[0]);
    assert_eq!(structured(&replies[0])["fuse"], "intact");
    let text = std::fs::read_to_string(dir.path.join("fusible.yml")).unwrap();
    assert!(text.starts_with("# keep this comment\n"), "{text}");
    assert!(
        text.contains("reason: \"bulk capacitor on the carrier\""),
        "{text}"
    );
    let report = structured(&replies[1]);
    assert_eq!(report["fuse"], "intact");
    assert_eq!(
        report["waived"][0]["reason"],
        "bulk capacitor on the carrier"
    );
}

#[test]
fn waive_refuses_an_empty_reason_a_netlist_rule_and_a_bad_date() {
    let dir = commented_project();
    let before = std::fs::read(dir.path.join("fusible.yml")).unwrap();
    let base = json!({"rule": "decoupling_capacitor", "match": "U1.VDD", "dry_run": false});
    let mut empty = base.clone();
    empty["reason"] = json!("   ");
    let mut dated = base.clone();
    dated["reason"] = json!("ok");
    dated["until"] = json!("soon");
    let netlist = json!({"rule": "unconnected_net", "match": "n2", "reason": "x"});

    let replies = session(
        &dir.path,
        &[
            call(1, "waive", empty),
            call(2, "waive", dated),
            call(3, "waive", netlist),
        ],
    );

    assert!(replies.iter().all(is_error), "{replies:?}");
    assert_eq!(std::fs::read(dir.path.join("fusible.yml")).unwrap(), before);
}

#[test]
#[cfg(unix)]
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
