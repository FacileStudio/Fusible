use super::{ClearanceClass, DesignRules, FusibleConfig};

fn load(yaml: &str) -> Result<FusibleConfig, String> {
    FusibleConfig::from_str(yaml).map_err(|e| format!("{e:?}"))
}

#[test]
fn design_keys_parse_and_default() {
    let cfg =
        load("nets:\n  VBUS: { voltage: 5, current: 2, max_rise: 10 }\nfab: jlcpcb-2l\n").unwrap();
    let rules = DesignRules::from_config(&cfg);
    assert_eq!(rules.nets["VBUS"].current, Some(2.0));
    assert_eq!(rules.fab.as_ref().map(|f| f.min_trace), Some(0.127));
    assert_eq!(
        rules.fab.as_ref().map(|f| f.name.as_str()),
        Some("jlcpcb-2l")
    );
    assert_eq!(rules.clearance_class, ClearanceClass::B2);
    assert_eq!((rules.copper.outer_oz, rules.copper.inner_oz), (1.0, 0.5));
}

#[test]
fn custom_fab_is_accepted_inline() {
    let yaml = "fab: { min_trace: 0.1, min_space: 0.1, min_drill: 0.2, min_via_pad: 0.5, min_annular: 0.1, \
                min_via_to_via: 0.2, drill_step: 0.05, board_min: [5, 5], board_max: [100, 100] }\n";
    let rules = DesignRules::from_config(&load(yaml).unwrap());
    assert_eq!(rules.fab.map(|f| f.name), Some("custom".to_string()));
}

#[test]
fn unsupported_class_is_rejected_with_the_supported_list() {
    let err = load("clearance_class: B3\n").unwrap_err();
    assert!(
        err.contains("B3, A5, A6, A7 are not supported yet; use B1, B2 or B4"),
        "{err}"
    );
}

#[test]
fn unknown_fab_and_bad_numbers_are_rejected() {
    assert!(load("fab: pcbway-std\n")
        .unwrap_err()
        .contains("not a built-in profile"));
    assert!(load("nets:\n  X: { current: -1 }\n")
        .unwrap_err()
        .contains("nets.X.current"));
    assert!(load("nets:\n  X: { voltage: 0 }\n")
        .unwrap_err()
        .contains("greater than 0"));
}
