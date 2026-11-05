use super::physical::{board, lint_with};
use crate::linter::rules::VoltageClearanceRule;

const MAINS: &str = "nets:\n  MAINS_L: { voltage: 230 }\n";

fn pair(layer: &str, gap: f64, manifest: &str) -> Vec<crate::linter::Diagnostic> {
    let y = 0.2 + gap;
    let json = board(
        "",
        &["MAINS_L", "GND"],
        &[
            ("MAINS_L", layer, 0.2, (0.0, 0.0), (10.0, 0.0)),
            ("GND", layer, 0.2, (0.0, y), (10.0, y)),
        ],
    );
    lint_with(VoltageClearanceRule, &json, manifest)
}

#[test]
fn mains_against_ground_blows_at_0_8_mm_and_passes_at_1_3() {
    let v = pair("top", 0.8, MAINS);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(
        v[0].message,
        "MAINS_L (230 V) and GND are 0.80 mm apart on top; IPC-2221B B2 asks at least 1.25 mm"
    );
    assert_eq!(v[0].standard.as_deref(), Some("IPC-2221B"));
    assert!(v[0].help.as_deref().unwrap().contains("IEC 62368-1"));
    assert!(pair("top", 1.3, MAINS).is_empty());
}

#[test]
fn class_b4_passes_at_half_a_millimetre() {
    let manifest = "nets:\n  MAINS_L: { voltage: 230 }\nclearance_class: B4\n";
    assert!(pair("top", 0.5, manifest).is_empty());
    assert_eq!(pair("top", 0.3, manifest).len(), 1);
}

#[test]
fn inner_layers_always_use_b1() {
    assert!(pair("inner1", 0.3, MAINS).is_empty());
    let v = pair("inner1", 0.1, MAINS);
    assert_eq!(v.len(), 1);
    assert!(
        v[0].message.contains("B1 asks at least 0.20 mm"),
        "{}",
        v[0].message
    );
}

#[test]
fn above_500_volts_the_formula_applies() {
    let manifest = "nets:\n  MAINS_L: { voltage: 600 }\n";
    let v = pair("top", 2.9, manifest);
    assert_eq!(v.len(), 1);
    assert!(
        v[0].message.contains("asks at least 3.00 mm"),
        "{}",
        v[0].message
    );
    assert!(pair("top", 3.1, manifest).is_empty());
    use crate::config::ClearanceClass::{B1, B2, B4};
    use crate::linter::rules::ipc2221b::spacing;
    assert!((spacing(B1, 600.0) - 0.5).abs() < 1e-9);
    assert!((spacing(B2, 600.0) - 3.0).abs() < 1e-9);
    assert!((spacing(B4, 600.0) - 1.1050).abs() < 1e-9);
}

#[test]
fn undeclared_nets_are_skipped_and_table_rows_hold() {
    let json = board(
        "",
        &["MAINS_L", "SIG"],
        &[
            ("MAINS_L", "top", 0.2, (0.0, 0.0), (10.0, 0.0)),
            ("SIG", "top", 0.2, (0.0, 0.5), (10.0, 0.5)),
        ],
    );
    assert!(lint_with(VoltageClearanceRule, &json, MAINS).is_empty());
    use crate::config::ClearanceClass::{B2, B4};
    use crate::linter::rules::ipc2221b::spacing;
    assert_eq!(spacing(B2, 15.0), 0.10);
    assert_eq!(spacing(B2, 16.0), 0.10);
    assert_eq!(spacing(B2, 31.0), 0.60);
    assert_eq!(spacing(B4, 101.0), 0.40);
    assert_eq!(spacing(B2, 500.0), 2.50);
}
