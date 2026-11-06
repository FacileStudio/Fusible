use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args, PartialEq)]
pub struct NewArgs {
    /// Name of the new body; unique across boards, firmware and bodies
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Board the part is modeled on [default: the only board]
    #[arg(long)]
    pub board: Option<String>,

    /// Material key from the material library
    #[arg(long, default_value = "petg")]
    pub material: String,

    /// Manufacturing process: fdm, sla, mjf, injection or cnc
    #[arg(long, default_value = "fdm")]
    pub process: String,
}

#[derive(Debug, Args, PartialEq)]
pub struct EjectArgs {
    /// Name of the generated body to eject
    #[arg(value_name = "BODY")]
    pub name: String,

    /// Output file; its extension (step, stl, glb, brep) picks the format
    /// [default: bodies/<BODY>.<first configured format, step when unset>]
    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Skip electrical rule verification before generating the model
    #[arg(long)]
    pub skip_lint: bool,
}

#[derive(Debug, Args, PartialEq)]
pub struct KitArgs {
    /// Board name or circuit file [default: the only board]
    #[arg(value_name = "BOARD")]
    pub board: Option<PathBuf>,

    /// Output directory [default: bodies/kit/<BOARD>, or the file name of a bare circuit file]
    #[arg(short = 'o', long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Skip electrical rule verification before writing the kit
    #[arg(long)]
    pub skip_lint: bool,
}
