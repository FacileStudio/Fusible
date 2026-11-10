//! `fusible migrate` renames rule ids only where the manifest names a rule.

mod common;

use common::{run_in, TestDir};

const OLD: &str = concat!(
    "# E_SCHEMATIC_SCALE_EXCESSIVE was noisy on rev A\n",
    "boards:\n  main: { circuit_json: circuit.json }\n",
    "rules:\n  E_SCHEMATIC_SCALE_EXCESSIVE: warning   # E_SCHEMATIC_SCALE_EXCESSIVE\n",
    "waivers:\n  - rule: E_SCHEMATIC_SCALE_EXCESSIVE\n",
    "    match: E_SCHEMATIC_SCALE_EXCESSIVE\n    reason: sheet drawn by hand\n",
);

const NEW: &str = concat!(
    "# E_SCHEMATIC_SCALE_EXCESSIVE was noisy on rev A\n",
    "boards:\n  main: { circuit_json: circuit.json }\n",
    "rules:\n  schematic_scale: warning   # E_SCHEMATIC_SCALE_EXCESSIVE\n",
    "waivers:\n  - rule: schematic_scale\n",
    "    match: E_SCHEMATIC_SCALE_EXCESSIVE\n    reason: sheet drawn by hand\n",
);

#[test]
fn rule_ids_are_renamed_in_rules_keys_and_waiver_rules_only() {
    let dir = TestDir::new("migrate_rule_ids");
    let path = dir.path.join("fusible.yml");
    std::fs::write(&path, OLD).unwrap();
    let (code, stdout, stderr) = run_in(&dir.path, &["migrate"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), NEW);
}
