use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Clone, Default, Args, PartialEq)]
pub struct StudioArgs {
    /// Entry file (.tsx, .json) or directory
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Port to run dev server on (default: 3020)
    #[arg(short = 'p', long)]
    pub port: Option<u16>,

    /// Use legacy tsci dev server instead of native Fusible Studio
    #[arg(long)]
    pub legacy: bool,

    /// Run under the desktop shell: print a ready line, exit when stdin closes
    #[arg(long)]
    pub desktop: bool,

    /// Also listen on the local network so a paired phone can view the project
    #[arg(long)]
    pub share: bool,
}
