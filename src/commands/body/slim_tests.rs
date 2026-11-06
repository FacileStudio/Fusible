use super::{convert_to_glb, generate_step, preview};
use crate::config::EnclosureConfig;
use crate::tscircuit::bun_path;

const BOARD: &str = r#"[{"type":"pcb_board","width":30,"height":20,"thickness":1.4},
  {"type":"pcb_hole","x":-12,"y":7,"hole_diameter":3.2}]"#;

fn primitives(glb: &[u8]) -> usize {
    let length = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
    let json: serde_json::Value = serde_json::from_slice(&glb[20..20 + length]).unwrap();
    json["meshes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["primitives"].as_array().unwrap().len())
        .sum()
}

#[test]
fn a_converted_glb_has_fewer_primitives_than_the_raw_export() {
    if bun_path().is_none() {
        eprintln!("skipped: bun is not available");
        return;
    }
    let raw = preview(BOARD, None).unwrap().glb;
    let dir = std::env::temp_dir().join(format!("fusible-slim-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let (step, glb) = (dir.join("body.step"), dir.join("body.glb"));
    generate_step(BOARD, EnclosureConfig::default(), step.clone()).unwrap();
    convert_to_glb(&step, "step", &glb).unwrap();
    let slim = std::fs::read(&glb).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        primitives(&slim) < primitives(&raw),
        "{} vs {}",
        primitives(&slim),
        primitives(&raw)
    );
    assert_eq!(primitives(&slim), 1);
}
