//! `firmware.<name>.outputs`, `framework` and `flash_cmd`.

use std::path::PathBuf;

use super::*;

const MANIFEST: &str = r#"
boards:
  main: { entry: index.tsx, mcu: stm32g431cbu }
firmware:
  legacy: { board: main, framework: esp-idf, pins_rs: fw/legacy/pins.rs }
  kinds: { board: main, framework: embassy, path: fw/kinds, outputs: [rust, c, zephyr] }
  paths:
    board: main
    framework: zephyr
    flash_cmd: west flash
    outputs: { c: out/pins.h, zephyr: out/board.overlay }
"#;

fn firmware(name: &str) -> FirmwareConfig {
    FusibleConfig::from_str(MANIFEST).unwrap().firmware[name].clone()
}

#[test]
fn a_1x_firmware_keeps_writing_pins_rs_only() {
    let fw = firmware("legacy");
    let rust = (OutputKind::Rust, PathBuf::from("fw/legacy/pins.rs"));
    assert_eq!(fw.output_paths(), [rust]);
    assert!(!fw.wants_board_rs());
    assert_eq!(fw.framework_kind(), Some(Framework::EspIdf));
}

#[test]
fn a_kind_list_takes_paths_under_the_firmware_directory() {
    let fw = firmware("kinds");
    let paths: Vec<_> = fw.output_paths().into_iter().map(|(_, p)| p).collect();
    let root = PathBuf::from("fw/kinds");
    assert_eq!(
        paths,
        [
            root.join("src/pins.rs"),
            root.join("include/pins.h"),
            root.join("app.overlay")
        ]
    );
    assert!(fw.wants_board_rs());
}

#[test]
fn a_path_map_names_each_file_and_flash_cmd_parses() {
    let fw = firmware("paths");
    let kinds: Vec<_> = fw.output_paths().into_iter().map(|(k, _)| k).collect();
    assert_eq!(kinds, [OutputKind::C, OutputKind::Zephyr]);
    assert_eq!(fw.flash_cmd.as_deref(), Some("west flash"));
    assert_eq!(fw.framework_kind(), Some(Framework::Zephyr));
    assert!(FusibleConfig::from_str("firmware: { a: { outputs: [cobol] } }").is_err());
}
