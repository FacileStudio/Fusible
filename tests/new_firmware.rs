//! The embedded firmware of the `fusible new` MCU templates.
//!
//! The normal suite checks the scaffolded files offline. The ignored test compiles each
//! crate for its real target; it needs network access, rustup, the thumb targets and the
//! `esp` toolchain: `cargo test --test new_firmware -- --ignored`.

mod common;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use common::scaffold::{create, offline};
use common::{fusible, TestDir};

/// Template, Rust target, HAL crate, and whether Fusible writes a `board.rs` for it.
const FIRMWARE: [(&str, &str, &str, bool); 3] = [
    ("stm32g4", "thumbv7em-none-eabihf", "embassy-stm32", true),
    ("rp2040", "thumbv6m-none-eabi", "embassy-rp", true),
    ("esp32-s3", "xtensa-esp32s3-none-elf", "esp-hal", false),
];

fn read(root: &Path, file: &str) -> String {
    std::fs::read_to_string(root.join(file)).unwrap_or_else(|e| panic!("{file}: {e}"))
}

#[test]
fn mcu_templates_scaffold_an_embedded_crate_bound_to_the_board() {
    let parent = TestDir::new("new_firmware");
    for (template, target, hal, board_rs) in FIRMWARE {
        let root = create(&parent, template);
        let crate_name = format!("name = \"demo-{template}-firmware\"");
        let expected = [
            ("firmware/Cargo.toml", crate_name),
            ("firmware/Cargo.toml", format!("{hal} = {{ version = \"=")),
            (
                "firmware/.cargo/config.toml",
                format!("target = \"{target}\""),
            ),
            ("firmware/rust-toolchain.toml", "[toolchain]".into()),
            ("firmware/src/pins.rs", "pub const LED: u8".into()),
            ("fusible.yml", "outputs: [rust]".into()),
            ("fusible.yml", "build_cmd: cargo build --release".into()),
            ("fusible.yml", "flash_cmd: cargo run --release".into()),
        ];
        for (file, needle) in expected {
            assert!(
                read(&root, file).contains(&needle),
                "{template} {file}: {needle}"
            );
        }
        let has_board_rs = root.join("firmware/src/board.rs").is_file();
        assert_eq!(has_board_rs, board_rs, "{template}");
    }
}

#[test]
fn check_writes_the_same_bindings_new_wrote() {
    let parent = TestDir::new("new_bindings");
    for (template, _, _, _) in FIRMWARE {
        let root = create(&parent, template);
        let before =
            ["pins.rs", "board.rs"].map(|f| std::fs::read(root.join("firmware/src").join(f)).ok());

        let out = offline(&root, &["check", "--write-bindings"]);

        assert_eq!(out.status.code(), Some(0), "{template}: {out:?}");
        let after =
            ["pins.rs", "board.rs"].map(|f| std::fs::read(root.join("firmware/src").join(f)).ok());
        assert_eq!(before, after, "{template}");
        assert!(
            read(&root, "firmware/src/main.rs").contains("LED"),
            "{template}"
        );
    }
}

/// The rustup proxies first, so each crate's `rust-toolchain.toml` picks its toolchain.
fn rustup_path() -> OsString {
    let home = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cargo")))
        .unwrap_or_default();
    let dirs = [
        home.join("bin"),
        PathBuf::from("/usr/bin"),
        PathBuf::from("/bin"),
    ];
    std::env::join_paths(dirs).expect("PATH should join")
}

#[test]
#[ignore = "downloads crates and needs the embedded Rust targets and the esp toolchain"]
fn every_template_firmware_builds_for_its_target() {
    let parent = TestDir::new("new_firmware_build");
    for (template, target, _, _) in FIRMWARE {
        let root = create(&parent, template);

        let out = fusible()
            .current_dir(&root)
            .env("PATH", rustup_path())
            .env_remove("RUSTUP_TOOLCHAIN")
            .env_remove("CARGO_TARGET_DIR")
            .env_remove("FUSIBLE_BUN")
            .arg("build")
            .output()
            .expect("binary should run");

        assert_eq!(out.status.code(), Some(0), "{template}: {out:?}");
        let elf = format!("firmware/target/{target}/release/demo-{template}-firmware");
        assert!(root.join(elf).is_file(), "{template}");
    }
}
