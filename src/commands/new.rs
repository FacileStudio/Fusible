//! Subcommand: `fusible new <name> --template <t>` — a complete project from a starter design.
//!
//! The project holds the board TSX with `pinAttributes` on its MCU and regulator, the Circuit
//! JSON tscircuit builds from it, a firmware crate bound to the board, a generated enclosure
//! and the manifest tying them together. The pre-built Circuit JSON lets `fusible check` pass
//! before any dependency is installed.

use std::path::{Path, PathBuf};

use crate::cli::ProjectArgs;
use crate::commands::init::starter::{self, Starter};
use crate::commands::init::{install_hardware_deps, InitError, TSCONFIG};

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
        ("fusible.yml", starter::manifest(name, starter.mcu)),
        (".gitignore", starter::GITIGNORE.to_string()),
        ("hardware/package.json", starter::package_json(name)),
        ("hardware/tsconfig.json", TSCONFIG.to_string()),
        ("hardware/index.tsx", starter.tsx.to_string()),
    ];
    if let Some((_, framework)) = starter.mcu {
        files.push(("firmware/Cargo.toml", starter::cargo_toml(name)));
        files.push(("firmware/src/main.rs", starter::main_rs(framework)));
        files.push(("firmware/src/pins.rs", starter::PINS_RS.to_string()));
    }
    files.push((CIRCUIT, starter.circuit.to_string()));
    files
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
    if starter.mcu.is_some() {
        println!("  ├── firmware/           a crate that reads the generated pins.rs");
    }
    println!("  └── hardware/dist/      the pre-built Circuit JSON");
    println!();
    println!("Next steps:");
    println!("  cd {name}");
    println!("  fusible check     run the rules (exit 0 means the fuse is intact)");
    println!("  fusible studio    open the board in the studio");
    println!("  fusible build     write pins.rs and build the firmware");
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
