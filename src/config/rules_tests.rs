use super::{ConfigError, Date, FusibleConfig, RuleLevel};

fn invalid(yaml: &str) -> String {
    match FusibleConfig::from_str(yaml) {
        Err(ConfigError::Invalid(message)) => message,
        other => panic!("expected an invalid manifest, got {other:?}"),
    }
}

const WAIVED: &str = "rules:\n  test_point_coverage: off\n  decoupling_capacitor: warning\n\
waivers:\n  - rule: decoupling_capacitor\n    match: \"U3.VDD\"\n    reason: shared bulk cap\n    until: 2027-03-01\n";

#[test]
fn rules_and_waivers_parse() {
    let cfg = FusibleConfig::from_str(WAIVED).unwrap();
    assert_eq!(cfg.rules["test_point_coverage"], RuleLevel::Off);
    assert_eq!(cfg.rules["decoupling_capacitor"], RuleLevel::Warning);
    let waiver = &cfg.waivers[0];
    assert_eq!(waiver.matches, "U3.VDD");
    assert_eq!(waiver.until, Date::parse("2027-03-01"));
    assert!(waiver.active(Date::parse("2027-03-01").unwrap()));
    assert!(!waiver.active(Date::parse("2027-03-02").unwrap()));
}

#[test]
fn a_waiver_needs_a_reason() {
    let missing = "waivers:\n  - rule: decoupling_capacitor\n    match: U3.VDD\n";
    assert!(invalid(missing).contains("needs a `reason`"));
    let blank = "waivers:\n  - {rule: decoupling_capacitor, match: U3.VDD, reason: \"  \"}\n";
    assert!(invalid(blank).contains("needs a `reason`"));
}

#[test]
fn netlist_rules_cannot_be_lowered_or_waived() {
    for rule in ["shorted_nets", "ghost_net", "unconnected_net"] {
        let lowered = format!("rules:\n  {rule}: warning\n");
        assert!(invalid(&lowered).contains("guards the netlist"), "{rule}");
        let waived = format!("waivers:\n  - {{rule: {rule}, match: N1, reason: why}}\n");
        assert!(invalid(&waived).contains("cannot be waived"), "{rule}");
    }
    assert!(FusibleConfig::from_str("rules:\n  shorted_nets: error\n").is_ok());
}

#[test]
fn bad_levels_and_dates_are_parse_errors() {
    let level = FusibleConfig::from_str("rules:\n  decoupling_capacitor: loud\n");
    assert!(matches!(level, Err(ConfigError::Parse { .. })));
    let date = "waivers:\n  - {rule: a, match: b, reason: c, until: 2027-02-30}\n";
    assert!(matches!(
        FusibleConfig::from_str(date),
        Err(ConfigError::Parse { .. })
    ));
}

#[test]
fn rule_ids_are_checked_against_the_registry() {
    let cfg = FusibleConfig::from_str(WAIVED).unwrap();
    let known = ["test_point_coverage", "decoupling_capacitor"];
    assert!(cfg.validate_rule_ids(&known).is_ok());
    let err = cfg
        .validate_rule_ids(&["decoupling_capacitor"])
        .unwrap_err();
    assert!(err.to_string().contains("test_point_coverage"));
}

#[test]
fn dates_parse_strictly() {
    assert_eq!(
        Date::parse("2024-02-29").map(|d| d.to_string()).as_deref(),
        Some("2024-02-29")
    );
    for bad in [
        "2023-02-29",
        "2027-13-01",
        "2027-00-10",
        "2027-3-01",
        "27-03-01",
        "2027/03/01",
    ] {
        assert_eq!(Date::parse(bad), None, "{bad}");
    }
    assert!(Date::parse("2027-03-01") < Date::parse("2027-03-02"));
    assert!(Date::parse("2026-12-31") < Date::parse("2027-01-01"));
}

#[test]
fn civil_dates_come_from_unix_days() {
    let day = |text: &str| Date::parse(text).unwrap();
    assert_eq!(Date::from_unix_days(0), day("1970-01-01"));
    assert_eq!(Date::from_unix_days(-1), day("1969-12-31"));
    assert_eq!(Date::from_unix_days(11_016), day("2000-02-29"));
    assert_eq!(Date::from_unix_days(20_402), day("2025-11-10"));
    assert!(Date::today() >= day("2025-01-01"));
}
