//! Integration tests for code parts: `fusible body`, `body new`, `check` and `bom` on `examples/mech`.

mod common;

use std::path::Path;

use common::{fusible, require_bun, TestDir};

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let target = to.join(entry.file_name());
        match entry.file_type().unwrap().is_dir() {
            true => copy_dir(&entry.path(), &target),
            false => drop(std::fs::copy(entry.path(), target).unwrap()),
        }
    }
}

fn project(name: &str) -> TestDir {
    let dir = TestDir::new(name);
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/mech");
    for entry in ["fusible.yml", "circuit.json"] {
        std::fs::copy(example.join(entry), dir.path.join(entry)).unwrap();
    }
    copy_dir(&example.join("mech"), &dir.path.join("mech"));
    dir
}

fn run(dir: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let out = fusible()
        .current_dir(dir)
        .args(args)
        .output()
        .expect("binary should run");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn bun_or_skip() -> bool {
    let found = require_bun().is_some();
    if !found {
        eprintln!("skipped: bun is not on PATH");
    }
    found
}

#[test]
fn a_code_part_builds_every_format_and_passes_the_mechanical_rules() {
    if !bun_or_skip() {
        return;
    }
    let dir = project("mech_case");
    let (code, out, err) = run(&dir.path, &["body", "case"]);
    assert_eq!(code, Some(0), "{out}\n{err}");
    for ext in ["step", "stl", "glb", "brep", "features.json"] {
        let file = dir.path.join(format!("dist/bodies/case.{ext}"));
        assert!(file.is_file(), "missing {}", file.display());
    }
    let (code, _, err) = run(&dir.path, &["check"]);
    assert_eq!(code, Some(0), "{err}");
    assert!(err.contains("bracket was not built yet"), "{err}");
}

#[test]
fn building_a_part_twice_gives_identical_step_bytes() {
    if !bun_or_skip() {
        return;
    }
    let dir = project("mech_repro");
    let step = dir.path.join("dist/bodies/case.step");
    assert_eq!(run(&dir.path, &["body", "case"]).0, Some(0));
    let first = std::fs::read(&step).unwrap();
    std::fs::remove_file(&step).unwrap();
    assert_eq!(run(&dir.path, &["body", "case"]).0, Some(0));
    assert_eq!(first, std::fs::read(&step).unwrap());
}

#[test]
fn a_part_with_thin_walls_and_narrow_vents_blows_the_fuse() {
    if !bun_or_skip() {
        return;
    }
    let dir = project("mech_bad");
    let manifest = dir.path.join("fusible.yml");
    let mut text = std::fs::read_to_string(&manifest).unwrap();
    text.push_str("  bad:\n    source: mech/bad.ts\n    material: petg\n    process: fdm\n");
    std::fs::write(&manifest, text).unwrap();
    assert_eq!(run(&dir.path, &["body", "bad"]).0, Some(0));
    let (code, _, err) = run(&dir.path, &["check"]);
    assert_eq!(code, Some(1), "{err}");
    for rule in [
        "mech.wall_thickness",
        "mech.vent_width",
        "mech.component_clearance",
    ] {
        assert!(err.contains(rule), "{rule} missing in {err}");
    }
    assert!(err.contains("Design guideline"), "{err}");
}

#[test]
fn mass_and_cost_reach_the_bom() {
    if !bun_or_skip() {
        return;
    }
    let dir = project("mech_bom");
    assert_eq!(run(&dir.path, &["body", "bracket"]).0, Some(0));
    let (code, out, err) = run(&dir.path, &["bom"]);
    assert_eq!(code, Some(0), "{err}");
    assert!(out.contains("3D print: bracket (al6061 cnc, "), "{out}");
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
