//! Integration tests for `fusible release`.

mod common;

use common::{fusible, TestDir};

#[test]
fn blown_board_exits_1_and_writes_nothing() {
    let dir = TestDir::new("release_blown");
    let out = dir.path.join("rel");
    let run = fusible()
        .args([
            "release",
            "examples/circuit-floating.json",
            "-o",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("binary should run");
    assert_eq!(run.status.code(), Some(1));
    assert!(!out.exists());
}

#[test]
fn intact_board_writes_the_bundle_and_manifest() {
    let dir = TestDir::new("release_intact");
    let out = dir.path.join("rel");
    let run = fusible()
        .args([
            "release",
            "examples/circuit-pad-shapes.json",
            "-o",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("binary should run");
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let manifest = std::fs::read_to_string(out.join("RELEASE.txt")).unwrap();
    assert!(manifest.contains("fuse intact"));
    assert!(manifest.contains("gerbers.zip"));
    assert!(out.join("bom.csv").exists() && out.join("pnp.csv").exists());
}

#[test]
fn warnings_block_unless_allowed() {
    let dir = TestDir::new("release_warn");
    let json = r#"[{"type":"pcb_board","width":10,"height":10,"center":{"x":0,"y":0}},
        {"type":"cad_component","footprinter_string":"qfn16_w3_h3_p0.5"}]"#;
    let file = dir.path.join("c.json");
    std::fs::write(&file, json).unwrap();
    let out = dir.path.join("rel");
    let args = |extra: &[&str]| {
        let mut cmd = fusible();
        cmd.args([
            "release",
            file.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ])
        .args(extra);
        cmd.output().expect("binary should run").status.code()
    };
    assert_eq!(args(&[]), Some(1));
    assert!(!out.exists());
    assert_eq!(args(&["--allow-warnings"]), Some(0));
}

#[test]
fn pour_on_a_layer_without_a_gerber_exits_2() {
    let dir = TestDir::new("release_pour");
    let json = r#"[{"type":"pcb_board","width":10,"height":10,"center":{"x":0,"y":0},"num_layers":2},
        {"type":"pcb_copper_pour","pcb_copper_pour_id":"pour_0","shape":"rect","layer":"inner1",
         "center":{"x":0,"y":0},"width":4,"height":4}]"#;
    let file = dir.path.join("c.json");
    std::fs::write(&file, json).unwrap();
    let out = dir.path.join("rel");
    let run = fusible()
        .args([
            "release",
            file.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("binary should run");
    assert_eq!(run.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&run.stderr).contains("inner1"));
    assert!(!out.exists());
}

#[test]
fn physics_warnings_block_a_release_unless_allowed() {
    let dir = TestDir::new("release_physics");
    std::fs::write(
        dir.path.join("fusible.yml"),
        "nets:\n  GND: { voltage: 5, current: 2 }\n",
    )
    .unwrap();
    std::fs::copy("examples/circuit.json", dir.path.join("c.json")).unwrap();
    let args = |extra: &[&str]| {
        let mut cmd = fusible();
        cmd.current_dir(&dir.path)
            .args(["release", "c.json", "-o", "rel"])
            .args(extra);
        cmd.output().expect("binary should run").status.code()
    };
    assert_eq!(args(&[]), Some(1));
    assert!(!dir.path.join("rel").exists());
    assert_ne!(args(&["--allow-warnings"]), Some(1));
}
