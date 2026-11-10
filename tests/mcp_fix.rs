//! MCP `waive`: waivers written through the edit safety rules.

mod common;
mod fake_support;

use common::TestDir;
use fake_support::rpc::{call, is_error, session, structured};
use fake_support::{git_commit_all, project};
use serde_json::json;

const NO_DECOUPLING: &str = "tests/fixtures/waivers/no-decoupling.json";

fn commented_project() -> TestDir {
    let dir = project(NO_DECOUPLING);
    let manifest = dir.path.join("fusible.yml");
    let text = std::fs::read_to_string(&manifest).unwrap();
    std::fs::write(&manifest, format!("# keep this comment\n{text}")).unwrap();
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
