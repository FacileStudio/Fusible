use super::new::add_entry;
use crate::config::FusibleConfig;

fn fields() -> Vec<String> {
    vec!["source: mech/x.ts".into(), "material: petg".into()]
}

#[test]
fn a_manifest_without_bodies_gets_a_section() {
    let out = add_entry("boards:\n  main: {}\n", "x", &fields()).unwrap();
    let cfg = FusibleConfig::from_str(&out).unwrap();
    assert_eq!(cfg.bodies["x"].material.as_deref(), Some("petg"));
}

#[test]
fn a_new_entry_goes_after_the_last_body_and_keeps_comments() {
    let old = "boards:\n  main: {}\nbodies:\n  # the lid\n  lid:\n    generate: enclosure\n\nassembly:\n  parts: {main: {}}\n";
    let out = add_entry(old, "x", &fields()).unwrap();
    assert!(out.contains("# the lid"));
    let cfg = FusibleConfig::from_str(&out).unwrap();
    assert!(cfg.bodies.contains_key("lid") && cfg.bodies.contains_key("x"));
    assert!(cfg.assembly.is_some());
}

#[test]
fn an_inline_bodies_mapping_is_refused() {
    assert!(add_entry("bodies: {a: {source: a.step}}\n", "x", &fields()).is_err());
    assert!(add_entry("mechanical: {}\n", "x", &fields()).is_err());
}
