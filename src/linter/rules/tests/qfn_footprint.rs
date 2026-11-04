//! `qfn_footprint_span` rule tests.

use super::lint;
use crate::linter::rules::QfnFootprintSpanRule;

fn with(footprint: &str) -> String {
    format!(r#"[{{"type":"cad_component","footprinter_string":"{footprint}"}}]"#)
}

#[test]
fn small_bare_qfn_warns_once_per_footprint() {
    let json = r#"[{"type":"cad_component","footprinter_string":"qfn16_w3_h3_p0.5"},
        {"type":"cad_component","footprinter_string":"qfn16_w3_h3_p0.5"}]"#;
    let v = lint(QfnFootprintSpanRule, json);
    assert_eq!(v.len(), 1);
    assert!(v[0].message.contains("copper span"));
}

#[test]
fn sized_pads_or_large_spans_pass() {
    let sized = with("qfn16_w3.8_h3.8_p0.5_pw0.25_pl0.8_thermalpad1.23x1.23mm");
    assert!(lint(QfnFootprintSpanRule, &sized).is_empty());
    assert!(lint(QfnFootprintSpanRule, &with("qfn48_w7_h7_p0.5")).is_empty());
    assert!(lint(QfnFootprintSpanRule, &with("soic8")).is_empty());
}
