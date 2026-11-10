//! Subcommand: `fusible new <name> --template <t>` — a complete project from a starter design.
//!
//! The project holds the board TSX with `pinAttributes` on its MCU and regulator, the Circuit
//! JSON tscircuit builds from it, an embedded firmware crate bound to the board, a generated
//! enclosure and the manifest tying them together. The pre-built Circuit JSON lets
//! `fusible check` pass before any dependency is installed.

use std::path::{Path, PathBuf};

use crate::cli::ProjectArgs;
use crate::codegen::{generate_pins_rs, rust};
use crate::commands::bindings;
use crate::commands::init::starter::{self, Starter};
use crate::commands::init::{install_hardware_deps, InitError, TSCONFIG};
use crate::parser;

/// Where the pre-built Circuit JSON goes: where `tsci build` writes it for `hardware/index.tsx`.
const CIRCUIT: &str = "hardware/dist/index/circuit.json";

fn check_name(name: &str) -> Result<(), InitError> {
    let valid = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let starts_well = name
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphanumeric());
    match valid && starts_well {
        true => Ok(()),
        false => Err(InitError::new(format!(
            "\"{name}\" is not a valid project name; use letters, digits, `-` and `_`"
        ))),
    }
}

fn empty_target(root: &Path) -> Result<(), InitError> {
    let used = std::fs::read_dir(root).is_ok_and(|mut entries| entries.next().is_some());
    match used {
        true => Err(InitError::new(format!(
            "{} already exists and is not empty",
            root.display()
        ))),
        false => Ok(()),
    }
}

/// Every file of the project, relative to its root; the Circuit JSON comes last so it is
/// newer than the TSX it was built from.
fn files(name: &str, starter: &Starter) -> Vec<(&'static str, String)> {
    let mut files = vec![
        ("fusible.yml", starter::manifest(name, starter.mcu.as_ref())),
        (".gitignore", starter::GITIGNORE.to_string()),
        ("hardware/package.json", starter::package_json(name)),
        ("hardware/tsconfig.json", TSCONFIG.to_string()),
        ("hardware/index.tsx", starter.tsx.to_string()),
    ];
    if let Some(mcu) = &starter.mcu {
        let crate_files = mcu.files.iter();
        files.extend(crate_files.map(|(path, text)| (*path, text.replace("@name@", name))));
        files.extend(bindings(name, starter.circuit, mcu.chip));
    }
    files.push((CIRCUIT, starter.circuit.to_string()));
    files
}

/// `pins.rs`, and `board.rs` for an Embassy chip, as `fusible build` writes them from the
/// pre-built circuit, so the firmware compiles before the first build.
fn bindings(name: &str, circuit: &str, chip: &str) -> Vec<(&'static str, String)> {
    let parsed = parser::parse(circuit, Path::new(CIRCUIT)).ok();
    let Some(b) = parsed.and_then(|c| bindings::model(&c, Some(chip), name)) else {
        return Vec::new();
    };
    let board = rust::board_rs(&b).map(|code| ("firmware/src/board.rs", code));
    std::iter::once(("firmware/src/pins.rs", generate_pins_rs(&b.mcu, &b.board)))
        .chain(board)
        .collect()
}

fn write(root: &Path, relative: &str, content: &str) -> Result<(), InitError> {
    let path = root.join(relative);
    let fail = |e: std::io::Error| InitError::new(format!("cannot write {}: {e}", path.display()));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(fail)?;
    }
    std::fs::write(&path, content).map_err(fail)
}

fn print_summary(name: &str, root: &Path, starter: &Starter) {
    println!("✨ Created project \"{name}\" in {}", root.display());
    println!("  ├── fusible.yml         boards, firmware, the enclosure body");
    println!("  ├── hardware/index.tsx  the board, with pinAttributes on its parts");
    if let Some(mcu) = &starter.mcu {
        println!(
            "  ├── firmware/           {} firmware that blinks the LED",
            mcu.framework
        );
    }
    println!("  └── hardware/dist/      the pre-built Circuit JSON");
    println!();
    println!("Next steps:");
    println!("  cd {name}");
    println!("  fusible check     run the rules (exit 0 means the fuse is intact)");
    println!("  fusible studio    open the board in the studio");
    println!("  fusible build     write the pin bindings and build the firmware");
    if starter.mcu.is_some() {
        println!("  fusible flash     flash the board (probe-rs or espflash)");
    }
}

/// Writes the project `args.name` from its template.
pub fn run(args: ProjectArgs) -> Result<(), InitError> {
    check_name(&args.name)?;
    let root = args
        .dir
        .unwrap_or_else(|| PathBuf::from("."))
        .join(&args.name);
    empty_target(&root)?;
    let starter = starter::starter(args.template);
    for (relative, content) in files(&args.name, &starter) {
        write(&root, relative, &content)?;
    }
    if !args.no_install {
        install_hardware_deps(&root.join("hardware"));
    }
    print_summary(&args.name, &root, &starter);
    Ok(())
}
