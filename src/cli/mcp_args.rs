//! Arguments of `fusible mcp`, `fusible share` and the shared `--opus` flags.

use std::path::PathBuf;

use clap::{Args, Subcommand, ValueEnum};

/// Flags that mirror blown fuses and unbought parts into Opus tasks.
#[derive(Debug, Args, PartialEq, Clone, Default)]
pub struct OpusArgs {
    /// Create Opus tasks for what needs attention (best effort, never changes the exit code)
    #[arg(long)]
    pub opus: bool,

    /// Opus project that receives the tasks
    #[arg(long, value_name = "NAME", requires = "opus")]
    pub opus_project: Option<String>,
}

/// Agent clients `fusible mcp --print-config` knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum McpClient {
    Bulle,
    Claude,
    Codex,
}

#[derive(Debug, Args, PartialEq)]
pub struct McpArgs {
    /// Project root [default: the nearest ancestor of the working directory with a fusible.yml]
    #[arg(long, value_name = "DIR")]
    pub root: Option<PathBuf>,

    /// Print the configuration snippet for a client and exit
    #[arg(long, value_enum, value_name = "CLIENT")]
    pub print_config: Option<McpClient>,
}

#[derive(Debug, Args, PartialEq)]
pub struct ShareArgs {
    #[command(subcommand)]
    pub target: ShareTarget,
}

/// Where `fusible share` sends the export.
#[derive(Debug, Subcommand, PartialEq)]
pub enum ShareTarget {
    /// Copy the fuse-gated export into a synced Nuage space
    Nuage(NuageArgs),
}

#[derive(Debug, Args, PartialEq)]
pub struct NuageArgs {
    /// Name of the Nuage space
    #[arg(long, value_name = "NAME")]
    pub space: String,

    /// Export preset: fab, cad, docs, or all
    #[arg(long, value_name = "PRESET", default_value = "fab")]
    pub preset: String,

    /// Create a share link for the uploaded folder
    #[arg(long)]
    pub link: bool,

    /// Lifetime of the share link
    #[arg(long, value_name = "DURATION", default_value = "30d")]
    pub expires: String,

    /// Overwrite a non-empty destination folder
    #[arg(long)]
    pub force: bool,
}
