use serde_json::Value;

use super::rules_api::rules_json;
use crate::config::{DesignRules, FusibleConfig};

fn rules(manifest: Option<&str>) -> Vec<Value> {
    let design = manifest.map(|m| DesignRules::from_config(&FusibleConfig::from_str(m).unwrap()));
    serde_json::from_str(&rules_json(design.as_ref())).unwrap()
}

fn enabled(list: &[Value], id: &str) -> bool {
    list.iter().find(|r| r["id"] == id).unwrap()["enabled"]
        .as_bool()
        .unwrap()
}

#[test]
fn every_registered_rule_is_listed_with_a_summary() {
    let list = rules(None);
    assert_eq!(list.len(), 55);
    for rule in &list {
        assert!(!rule["summary"].as_str().unwrap().is_empty(), "{rule}");
        assert!(["error", "warning", "info"].contains(&rule["severity"].as_str().unwrap()));
    }
    let current = list.iter().find(|r| r["id"] == "trace_current").unwrap();
    assert_eq!(current["standard"], "IPC-2152");
    assert!(current["source_url"]
        .as_str()
        .unwrap()
        .starts_with("https://docs.kicad.org/"));
}

#[test]
fn physics_rules_are_enabled_only_when_their_data_is_declared() {
    let off = rules(None);
    for id in ["trace_current", "voltage_clearance", "fab_profile"] {
        assert!(!enabled(&off, id), "{id}");
    }
    assert!(enabled(&off, "unconnected_net"));
    let on = rules(Some(
        "nets:\n  VBUS: { voltage: 5, current: 2 }\nfab: jlcpcb-4l\n",
    ));
    for id in ["trace_current", "voltage_clearance", "fab_profile"] {
        assert!(enabled(&on, id), "{id}");
    }
    let voltage_only = rules(Some("nets:\n  VBUS: { voltage: 5 }\n"));
    assert!(!enabled(&voltage_only, "trace_current"));
}
