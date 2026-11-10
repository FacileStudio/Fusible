use std::path::Path;

use super::levels::Levels;
use super::waive::matches;
use crate::config::{Date, FusibleConfig, RuleLevel, Waiver};
use crate::linter::{Diagnostic, Severity};
use crate::parser::{self, Circuit};

fn circuit() -> Circuit {
    let source = r#"[
      {"type": "source_component", "source_component_id": "sc0", "name": "U3", "ftype": "chip"},
      {"type": "source_port", "source_port_id": "sp0", "name": "VDD", "pin_number": 4,
       "source_component_id": "sc0"},
      {"type": "source_port", "source_port_id": "sp1", "name": "GND", "pin_number": 5,
       "source_component_id": "sc0"}
    ]"#;
    parser::parse(source, Path::new("circuit.json")).unwrap()
}

fn waiver(rule: &str, at: &str, until: Option<&str>) -> Waiver {
    Waiver {
        rule: rule.into(),
        matches: at.into(),
        reason: "checked on rev A".into(),
        until: until.and_then(Date::parse),
    }
}

fn diagnostic(rule: &str, elements: &[&str], message: &str) -> Diagnostic {
    Diagnostic {
        rule: rule.into(),
        message: message.into(),
        elements: elements.iter().map(|e| e.to_string()).collect(),
        ..Default::default()
    }
}

#[test]
fn a_waiver_matches_an_element_its_name_or_its_refdes_pin() {
    let c = circuit();
    let on_port = diagnostic("decoupling_capacitor", &["sp0"], "no cap");
    for at in ["sp0", "U3.VDD", "U3.4"] {
        assert!(
            matches(
                &waiver("decoupling_capacitor", at, None),
                &on_port,
                Some(&c)
            ),
            "{at}"
        );
    }
    let on_chip = diagnostic("decoupling_capacitor", &["sc0"], "no cap");
    assert!(matches(
        &waiver("decoupling_capacitor", "U3", None),
        &on_chip,
        Some(&c)
    ));
    assert!(!matches(
        &waiver("trace_current", "sp0", None),
        &on_port,
        Some(&c)
    ));
    assert!(!matches(
        &waiver("decoupling_capacitor", "U3.GND", None),
        &on_port,
        Some(&c)
    ));
}

#[test]
fn a_pin_named_in_the_message_tells_two_pins_of_one_chip_apart() {
    let c = circuit();
    let found = diagnostic(
        "decoupling_capacitor",
        &["C1", "U3"],
        "C1 decouples U3 pin 4 (VDD)",
    );
    assert!(matches(
        &waiver("decoupling_capacitor", "U3.VDD", None),
        &found,
        Some(&c)
    ));
    assert!(!matches(
        &waiver("decoupling_capacitor", "U3.GND", None),
        &found,
        Some(&c)
    ));
    assert!(!matches(
        &waiver("decoupling_capacitor", "U3.4", None),
        &found,
        Some(&c)
    ));
}

fn config() -> FusibleConfig {
    let mut config = FusibleConfig::default();
    config.rules.insert("trace_current".into(), RuleLevel::Off);
    config
        .rules
        .insert("decoupling_capacitor".into(), RuleLevel::Info);
    config.waivers = vec![
        waiver("fab_profile", "sp1", None),
        waiver("pin_mux", "sp0", Some("2000-01-01")),
        waiver("qfn_footprint_span", "U9", None),
    ];
    config
}

#[test]
fn levels_drop_relevel_and_waive_then_report_unused_and_expired() {
    let config = config();
    let mut levels = Levels::new(&config, Date::parse("2026-11-10").unwrap()).unwrap();
    let found = vec![
        diagnostic("trace_current", &["sp0"], "thin"),
        diagnostic("decoupling_capacitor", &["sp0"], "far"),
        diagnostic("fab_profile", &["sp1"], "small drill"),
        diagnostic("pin_mux", &["sp0"], "conflict"),
    ];
    let mut waived = Vec::new();
    let kept = levels.apply(found, Some(&circuit()), Some("main"), &mut waived);
    let rules: Vec<(&str, Severity)> = kept.iter().map(|d| (d.rule.as_str(), d.severity)).collect();
    assert_eq!(
        rules,
        [
            ("decoupling_capacitor", Severity::Info),
            ("pin_mux", Severity::Error)
        ]
    );
    assert_eq!(waived.len(), 1);
    assert_eq!(waived[0].diagnostic.rule, "fab_profile");
    assert_eq!(waived[0].board.as_deref(), Some("main"));
    let unused = levels.unused(true);
    assert_eq!(unused.len(), 1);
    assert_eq!(unused[0].elements, ["U9"]);
    assert_eq!(levels.expired()[0].until.as_deref(), Some("2000-01-01"));
}

#[test]
fn a_diagnostic_without_elements_matches_through_its_message() {
    let c = circuit();
    let found = diagnostic(
        "decoupling_capacitor",
        &[],
        "IC \"U3\" power pin \"VDD\" has no decoupling capacitor",
    );
    assert!(matches(
        &waiver("decoupling_capacitor", "U3.VDD", None),
        &found,
        Some(&c)
    ));
    assert!(!matches(
        &waiver("decoupling_capacitor", "U3.GND", None),
        &found,
        Some(&c)
    ));
}
