use super::physical::{board, lint_with};
use crate::linter::rules::TraceCurrentRule;
use crate::linter::{Linter, Severity};

const VBUS: &str = "nets:\n  VBUS: { current: 1 }\n";

fn run(layer: &str, width: f64, manifest: &str) -> Vec<crate::linter::Diagnostic> {
    let json = board(
        "",
        &["VBUS"],
        &[("VBUS", layer, width, (0.0, 0.0), (10.0, 0.0))],
    );
    lint_with(TraceCurrentRule, &json, manifest)
}

#[test]
fn external_one_ounce_matches_the_formula_by_hand() {
    let by_hand = 215.3 * 1.0_f64.powf(2.0) * 10.0_f64.powf(-1.15) * 1.378_f64.powf(-1.0);
    let v = run("top", 0.254, VBUS);
    assert_eq!(v.len(), 1, "{v:?}");
    let heated: f64 = v[0].message.split("about ").nth(1).unwrap()[..2]
        .parse()
        .unwrap();
    assert!((heated - by_hand).abs() < 1.0, "{heated} vs {by_hand}");
    assert!((by_hand - 11.06).abs() < 0.1);
    assert_eq!(v[0].standard.as_deref(), Some("IPC-2152"));
    assert_eq!(v[0].elements, vec!["pt0".to_string()]);
    assert!(v[0].help.as_deref().unwrap().contains("not certification"));
}

#[test]
fn rise_function_agrees_with_the_hand_computation_within_a_tenth() {
    use crate::linter::rules::ipc2152::rise;
    let by_hand = 215.3 * 10.0_f64.powf(-1.15) / 1.378;
    assert!((rise(1.0, 0.254, false, 1.0) - by_hand).abs() < 0.1);
    let internal = 200.0 * 1.0_f64.powf(1.9) * 10.0_f64.powf(-1.10) * 1.378_f64.powf(-1.52);
    assert!((rise(1.0, 0.254, true, 1.0) - internal).abs() < 0.1);
    let half = 120.0 * 2.0_f64.powf(2.0) * 20.0_f64.powf(-1.10) * (0.5 * 1.378_f64).powf(-1.52);
    assert!((rise(2.0, 0.508, true, 0.5) - half).abs() < 0.1);
}

#[test]
fn internal_layer_uses_the_inner_weight() {
    let manifest =
        "nets:\n  VBUS: { current: 1, max_rise: 5 }\ncopper: { outer_oz: 1, inner_oz: 1 }\n";
    let v = run("inner1", 0.254, manifest);
    assert_eq!(v.len(), 1);
    assert!(v[0].message.contains("inner1"));
}

#[test]
fn a_wide_trace_passes() {
    assert!(run("top", 1.0, VBUS).is_empty());
}

#[test]
fn the_fix_width_clears_the_limit() {
    use crate::linter::rules::ipc2152::{min_width, rise};
    let v = run("top", 0.254, VBUS);
    let fix = v[0].fix.as_deref().unwrap();
    assert!(fix.starts_with("widen VBUS to at least "), "{fix}");
    let width = min_width(1.0, 10.0, false, 1.0).unwrap();
    assert!(rise(1.0, width, false, 1.0) <= 10.0);
    assert!(rise(1.0, width - 0.02, false, 1.0) > 10.0);
    assert!(fix.contains(&format!("{width:.2}")));
}

#[test]
fn a_declared_net_without_traces_warns_and_no_manifest_is_silent() {
    let json = board("", &["VBUS"], &[]);
    let v = lint_with(TraceCurrentRule, &json, VBUS);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].severity, Severity::Warning);
    assert!(v[0]
        .message
        .contains("declares 1 A but has no routed traces to check"));
    let circuit = crate::parser::parse(&json, std::path::Path::new("c.json")).unwrap();
    let none = Linter::new().with_rule(TraceCurrentRule).run(&circuit);
    assert!(none.is_empty());
}

#[test]
fn an_unreadable_trace_is_reported() {
    let bad = r#"{"type":"pcb_trace","pcb_trace_id":"pt9","route":[{"x":0,"y":0,"layer":"top"}]}"#;
    let json = board(
        bad,
        &["VBUS"],
        &[("VBUS", "top", 1.0, (0.0, 0.0), (1.0, 0.0))],
    );
    let v = lint_with(TraceCurrentRule, &json, VBUS);
    assert_eq!(v.len(), 1);
    assert!(v[0].message.contains("pt9"));
}
