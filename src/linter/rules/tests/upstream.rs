//! `upstream` rule tests, on errors and warnings from a real tscircuit build
//! of two overlapping resistors and a chip placed off a 10 mm board.

use super::lint;
use crate::linter::rules::upstream::kinds::KINDS;
use crate::linter::rules::UpstreamRule;
use crate::linter::{Linter, Severity};

const SAMPLE: &str = include_str!("upstream.json");

#[test]
fn each_real_error_blows_under_its_own_id() {
    let v = lint(UpstreamRule, SAMPLE);
    let errors = v.iter().filter(|d| d.severity == Severity::Error);
    let errors: Vec<&str> = errors.map(|d| d.rule.as_str()).collect();
    let expected = [
        "placement",
        "placement",
        "off_board",
        "footprint_overlap",
        "autorouting",
        "port_not_connected",
        "port_not_connected",
        "trace_missing",
        "trace_missing",
    ];
    assert_eq!(errors, expected);
    let overlap = v
        .iter()
        .find(|d| d.rule == "footprint_overlap")
        .expect("overlap");
    assert!(overlap
        .message
        .starts_with("pcb_footprint_overlap_error: R1 overlaps R2"));
    assert_eq!(
        overlap.elements[..2],
        ["pcb_component_0", "pcb_component_1"]
    );
    assert!(overlap.elements[2..]
        .iter()
        .all(|id| id.starts_with("pcb_smtpad_")));
}

#[test]
fn warnings_fold_by_kind_and_unnamed_traces_are_info() {
    let v = lint(UpstreamRule, SAMPLE);
    let unnamed = v
        .iter()
        .find(|d| d.rule == "unnamed_trace")
        .expect("unnamed");
    assert_eq!(unnamed.severity, Severity::Info);
    assert!(unnamed
        .message
        .ends_with("(and 1 more source_unnamed_trace_warning)"));
    let other: Vec<_> = v.iter().filter(|d| d.rule == "upstream_warning").collect();
    assert_eq!(other.len(), 6);
    assert!(other.iter().all(|d| d.severity == Severity::Warning));
}

#[test]
fn an_unknown_error_fails_closed_as_upstream_error() {
    let json = r#"[{"type":"source_trace_not_connected_error","message":"no thermalpad port"}]"#;
    let v = lint(UpstreamRule, json);
    assert_eq!(
        (v[0].rule.as_str(), v[0].severity),
        ("upstream_error", Severity::Error)
    );
    assert!(v[0]
        .message
        .contains("source_trace_not_connected_error: no thermalpad port"));
    assert!(v[0]
        .help
        .as_deref()
        .unwrap()
        .contains("pinLabel named thermalpad"));
}

#[test]
fn every_mapped_id_is_registered_and_errors_stay_errors() {
    let described = Linter::default().describe(None);
    for (kind, meta) in KINDS {
        assert!(
            described.iter().any(|r| r.id == meta.id),
            "{} is not described",
            meta.id
        );
        if kind.ends_with("_error") {
            assert_eq!(meta.default_severity, Severity::Error, "{kind}");
        }
    }
    for id in ["upstream_error", "upstream_warning", "unrouted"] {
        assert!(
            described.iter().any(|r| r.id == id),
            "{id} is not described"
        );
    }
}
