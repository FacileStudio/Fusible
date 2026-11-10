use super::levels::{known_rule_ids, Levels};
use crate::config::{Date, FusibleConfig, Waiver};

fn waiver(rule: &str) -> Waiver {
    Waiver {
        rule: rule.into(),
        matches: "J1".into(),
        reason: "checked on rev A".into(),
        until: None,
    }
}

#[test]
fn an_assembly_waiver_is_unused_only_once_the_assembly_ran() {
    let config = FusibleConfig {
        waivers: vec![waiver("mech.boss"), waiver("harness_mismatch")],
        ..Default::default()
    };
    let levels = Levels::new(&config, Date::parse("2026-11-10").unwrap()).unwrap();
    assert_eq!(levels.unused(false).len(), 1);
    assert_eq!(levels.unused(true).len(), 2);
    assert!(known_rule_ids().contains(&"connector_blocked"));
}
