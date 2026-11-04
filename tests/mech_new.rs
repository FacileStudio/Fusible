//! Integration tests for `fusible body new` and the material check on `examples/mech`.

mod common;

use common::{mech_project as project, require_bun, run_in as run};

fn bun_or_skip() -> bool {
    let found = require_bun().is_some();
    if !found {
        eprintln!("skipped: bun is not on PATH");
    }
    found
}

#[test]
fn body_new_scaffolds_a_part_and_refuses_a_taken_name() {
    if !bun_or_skip() {
        return;
    }
    let dir = project("mech_new");
    let (code, _, err) = run(&dir.path, &["body", "new", "probe"]);
    assert_eq!(code, Some(0), "{err}");
    assert!(dir.path.join("mech/probe.ts").is_file());
    assert!(dir.path.join("mech/fusible.d.ts").is_file());
    let manifest = std::fs::read_to_string(dir.path.join("fusible.yml")).unwrap();
    assert!(
        manifest.contains("probe:\n    source: mech/probe.ts"),
        "{manifest}"
    );
    assert_eq!(run(&dir.path, &["body", "probe"]).0, Some(0));
    assert!(dir.path.join("dist/bodies/probe.step").is_file());
    for taken in ["probe", "main", "case"] {
        let (code, _, err) = run(&dir.path, &["body", "new", taken]);
        assert_eq!(code, Some(2), "{taken}: {err}");
    }
}

#[test]
fn an_unknown_material_is_a_manifest_error() {
    let dir = project("mech_material");
    let manifest = dir.path.join("fusible.yml");
    let text = std::fs::read_to_string(&manifest)
        .unwrap()
        .replace("petg", "unobtainium");
    std::fs::write(&manifest, text).unwrap();
    let (code, _, err) = run(&dir.path, &["check"]);
    assert_eq!(code, Some(2), "{err}");
    assert!(err.contains("unobtainium"), "{err}");
}
