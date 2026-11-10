//! The starter designs of `fusible new`: a board, its pre-built Circuit JSON and its firmware.
//!
//! Each `circuit.json` was built from the `index.tsx` beside it with the tscircuit version
//! pinned in [`package_json`], so `fusible check` passes offline before any install. Each
//! MCU template carries an embedded crate (`templates/<t>/firmware/`) that blinks the LED
//! through the bindings Fusible generates.

use crate::cli::Template;

/// One starter design.
pub struct Starter {
    /// The board source, `hardware/index.tsx`.
    pub tsx: &'static str,
    /// The Circuit JSON tscircuit builds from `tsx`.
    pub circuit: &'static str,
    /// The MCU and its firmware, or `None` for a board without an MCU.
    pub mcu: Option<Mcu>,
}

/// The MCU of a starter and the firmware crate that runs on it.
pub struct Mcu {
    /// The board `mcu:` value.
    pub chip: &'static str,
    /// The firmware `framework:` value.
    pub framework: &'static str,
    /// Files of the crate, relative to the project root; `@name@` stands for the project name.
    pub files: &'static [(&'static str, &'static str)],
}

const STM32G4: &[(&str, &str)] = &[
    (
        "firmware/Cargo.toml",
        include_str!("templates/stm32g4/firmware/Cargo.toml.in"),
    ),
    (
        "firmware/.cargo/config.toml",
        include_str!("templates/stm32g4/firmware/config.toml"),
    ),
    (
        "firmware/rust-toolchain.toml",
        include_str!("templates/stm32g4/firmware/rust-toolchain.toml"),
    ),
    (
        "firmware/src/main.rs",
        include_str!("templates/stm32g4/firmware/main.rs"),
    ),
];

const RP2040: &[(&str, &str)] = &[
    (
        "firmware/Cargo.toml",
        include_str!("templates/rp2040/firmware/Cargo.toml.in"),
    ),
    (
        "firmware/.cargo/config.toml",
        include_str!("templates/rp2040/firmware/config.toml"),
    ),
    (
        "firmware/rust-toolchain.toml",
        include_str!("templates/rp2040/firmware/rust-toolchain.toml"),
    ),
    (
        "firmware/build.rs",
        include_str!("templates/rp2040/firmware/build.rs"),
    ),
    (
        "firmware/memory.x",
        include_str!("templates/rp2040/firmware/memory.x"),
    ),
    (
        "firmware/src/main.rs",
        include_str!("templates/rp2040/firmware/main.rs"),
    ),
];

const ESP32S3: &[(&str, &str)] = &[
    (
        "firmware/Cargo.toml",
        include_str!("templates/esp32-s3/firmware/Cargo.toml.in"),
    ),
    (
        "firmware/.cargo/config.toml",
        include_str!("templates/esp32-s3/firmware/config.toml"),
    ),
    (
        "firmware/rust-toolchain.toml",
        include_str!("templates/esp32-s3/firmware/rust-toolchain.toml"),
    ),
    (
        "firmware/src/main.rs",
        include_str!("templates/esp32-s3/firmware/main.rs"),
    ),
];

/// The files of `template`.
pub fn starter(template: Template) -> Starter {
    let mcu = |chip, framework, files| {
        Some(Mcu {
            chip,
            framework,
            files,
        })
    };
    match template {
        Template::Esp32S3 => Starter {
            tsx: include_str!("templates/esp32-s3/index.tsx"),
            circuit: include_str!("templates/esp32-s3/circuit.json"),
            mcu: mcu("esp32-s3", "esp-hal", ESP32S3),
        },
        Template::Stm32G4 => Starter {
            tsx: include_str!("templates/stm32g4/index.tsx"),
            circuit: include_str!("templates/stm32g4/circuit.json"),
            mcu: mcu("stm32g431kb", "embassy", STM32G4),
        },
        Template::Rp2040 => Starter {
            tsx: include_str!("templates/rp2040/index.tsx"),
            circuit: include_str!("templates/rp2040/circuit.json"),
            mcu: mcu("rp2040", "embassy", RP2040),
        },
        Template::Blank => Starter {
            tsx: include_str!("templates/blank/index.tsx"),
            circuit: include_str!("templates/blank/circuit.json"),
            mcu: None,
        },
    }
}

/// `.gitignore` of a new project: build output stays out, the pre-built circuit stays in.
pub const GITIGNORE: &str =
    "node_modules/\ndist/\n!hardware/dist/\n.tscircuit/\n.fusible/\ntarget/\n*.kicad_prl\n*.bak\n";

/// The `fusible.yml` of a project named `name`.
///
/// The firmware builds and flashes through cargo: the target and the `probe-rs` or
/// `espflash` runner live in `firmware/.cargo/config.toml`.
pub fn manifest(name: &str, mcu: Option<&Mcu>) -> String {
    let name = serde_json::to_string(name).unwrap_or_default();
    let board_mcu = mcu.map_or_else(String::new, |m| format!("    mcu: {}\n", m.chip));
    let firmware = mcu.map_or_else(String::new, |m| {
        format!(
            "\nfirmware:\n  app:\n    board: main\n    path: firmware\n    framework: {}\n    \
             pins_rs: firmware/src/pins.rs\n    outputs: [rust]\n    build_cmd: cargo build --release\n    \
             flash_cmd: cargo run --release\n",
            m.framework
        )
    });
    format!(
        "name: {name}\n\nboards:\n  main:\n    entry: hardware/index.tsx\n    out_dir: dist\n\
         {board_mcu}{firmware}\nbodies:\n  enclosure:\n    generate: enclosure\n    board: main\n\n\
         # rules:\n#   decoupling_capacitor: warning\n# waivers:\n#   - rule: decoupling_capacitor\n\
         #     match: U1.VDD\n#     reason: why this case is safe\n"
    )
}

/// `hardware/package.json`, pinned to the tscircuit release the starters were built with.
pub fn package_json(name: &str) -> String {
    let package = serde_json::to_string(&format!("{name}-hardware")).unwrap_or_default();
    format!(
        "{{\n  \"name\": {package},\n  \"version\": \"0.1.0\",\n  \"private\": true,\n  \
         \"dependencies\": {{\n    \"react\": \"19.3.1\",\n    \"tscircuit\": \"0.0.3139\"\n  }}\n}}\n"
    )
}
