use super::*;

#[test]
fn a_code_part_declares_a_known_material_and_a_process_it_supports() {
    let ok = "boards: {a: {}}\nbodies:\n  bracket: {source: mech/bracket.ts, material: al6061, process: cnc}\n";
    let cfg = FusibleConfig::from_str(ok).unwrap();
    assert!(cfg.bodies["bracket"].is_code());
    assert_eq!(cfg.body_board(&cfg.bodies["bracket"]), Some("a"));
    let unknown = ok.replace("al6061", "unobtainium");
    assert!(FusibleConfig::from_str(&unknown).is_err());
    let wrong = ok.replace("cnc", "fdm");
    assert!(FusibleConfig::from_str(&wrong).is_err());
    let step = "bodies:\n  knob: {source: knob.step}\n";
    assert!(!FusibleConfig::from_str(step).unwrap().bodies["knob"].is_code());
}
