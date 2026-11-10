//! Step 1 — command-line definition.
//!
//! clap handles CLI arguments and subcommands (`studio`, `export`, `build`, `check`, `init`).

use std::path::PathBuf;

use clap::builder::ValueHint;
use clap::{Parser, Subcommand};

mod args;
mod args_body;
mod args_new;
mod args_studio;
mod mcp_args;

pub use args::{
    AssemblyArgs, BodyAction, BodyArgs, BomArgs, BomFormat, CheckArgs, ExportArgs, InitArgs,
    ReleaseArgs, RouteArgs, SyncArgs,
};
pub use args_body::{EjectArgs, KitArgs, NewArgs};
pub use args_new::{ExplainArgs, ProjectArgs, Template};
pub use args_studio::StudioArgs;
pub use mcp_args::{McpArgs, McpClient, NuageArgs, OpusArgs, ShareArgs, ShareTarget};

/// Fusible — electronic linter & hardware-firmware bridge for tscircuit.
#[derive(Debug, Parser)]
#[command(
    name = "fusible",
    version,
    arg_required_else_help = true,
    about = "Electronic linter: the fuse blows to block firmware code generation",
    after_help = "Examples:\n  fusible --input circuit.json\n  fusible check\n  fusible studio"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the Circuit JSON or TSX file (shorthand for check)
    #[arg(short = 'i', long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub input: Option<PathBuf>,

    /// Silicon target, e.g. `stm32g431` (consumed by pin-muxing in phase 2)
    #[arg(long, value_name = "MCU")]
    pub mcu: Option<String>,
}

/// Available subcommands for managing tscircuit monorepo projects.
#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// Open the studio: the interactive viewer with hot reload
    #[command(alias = "dev", alias = "view", alias = "webview")]
    Studio(StudioArgs),

    /// Run the electronic linter on a circuit file or monorepo project
    #[command(alias = "lint")]
    Check(CheckArgs),

    /// Compile circuit artifacts and build firmware
    Build {
        /// Path to entry TSX file
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
    },

    /// Export tscircuit code to various formats (gerbers, svg, glb, etc.)
    Export(ExportArgs),

    /// Check every board, then write the fabrication bundle with a checksum manifest
    Release(ReleaseArgs),

    /// Build the physical bodies: authored CAD files and generated enclosures
    #[command(
        alias = "bodies",
        alias = "mechanical",
        alias = "case",
        alias = "housing"
    )]
    Body(BodyArgs),

    /// Place the boards and bodies of `assembly:` in one frame and export one model
    #[command(alias = "asm")]
    Assembly(AssemblyArgs),

    /// List the parts of the project with quantities and prices (bill of materials)
    #[command(alias = "parts")]
    Bom(BomArgs),

    /// Serve the project to AI agents over MCP (stdio)
    Mcp(McpArgs),

    /// Send the fuse-gated export to a Facile app
    Share(ShareArgs),

    /// Run the autorouter to solve circuit traces
    Route(RouteArgs),

    /// Compare PCB footprint placement between KiCad and tscircuit
    Diff {
        /// Path to KiCad PCB file (.kicad_pcb)
        #[arg(value_name = "KICAD_PCB", required = true)]
        kicad_pcb: PathBuf,

        /// Path to Circuit JSON or TSX entry file
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
    },

    /// Synchronize layout adjustments from KiCad back into tscircuit
    Sync(SyncArgs),

    /// Initialize a new Fusible project and tscircuit hardware workspace
    Init(InitArgs),

    /// Create a project from a template: board, firmware crate, enclosure and manifest
    New(ProjectArgs),

    /// Describe one rule: what it checks, its standard, its level here
    Explain(ExplainArgs),

    /// List every rule with its category and level
    Rules {
        /// Print one JSON array
        #[arg(long)]
        json: bool,
    },

    /// Rewrite a 1.x fusible.yml into the 2.0 form and print the diff
    Migrate {
        /// Write nothing; exit 1 when the manifest needs a rewrite
        #[arg(long)]
        check: bool,
    },

    /// Add a tscircuit component or registry package to the hardware project
    Add {
        /// Packages to install (e.g. @tsci/seveibar.smd-resistor or @tscircuit/core)
        #[arg(value_name = "PACKAGE", required = true)]
        packages: Vec<String>,
    },

    /// Install all hardware dependencies in the project
    Install {
        /// Target directory (defaults to hardware/ or current directory)
        #[arg(value_name = "DIR")]
        dir: Option<PathBuf>,
    },

    /// Search the tscircuit registry for components, footprints, and CAD models
    Search {
        /// Search terms (e.g. "esp32", "usb-c", "resistor 0603")
        #[arg(value_name = "QUERY", required = true)]
        query: Vec<String>,
    },

    /// Inspect toolchain health and available tscircuit runners
    #[command(alias = "setup")]
    Doctor {
        /// Automatically install tscircuit runner if missing
        #[arg(long)]
        install: bool,

        /// Print the toolchain as JSON and exit 0 even when tools are missing
        #[arg(long)]
        json: bool,
    },
}
