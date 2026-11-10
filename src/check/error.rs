//! Why a check could not finish: every variant except `FuseBlown` is a tool error (exit 2).

use miette::Diagnostic;
use thiserror::Error;

use crate::commands::assembly::AssemblyError;
use crate::commands::target::TargetError;
use crate::linter::FuseBlown;
use crate::parser;
use crate::tscircuit::RunnerError;

/// The input file could not be read.
#[derive(Debug, Error, Diagnostic)]
#[error("cannot read the Circuit JSON at \"{path}\"")]
#[diagnostic(help("check that the file exists and is readable"))]
pub struct ReadError {
    pub path: String,
    #[source]
    pub source: std::io::Error,
}

/// Unified error enum for the check pipeline.
#[derive(Debug, Error, Diagnostic)]
pub enum CheckError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Target(#[from] TargetError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Runner(#[from] RunnerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Read(#[from] ReadError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Parse(#[from] parser::JsonDiagnostic),

    #[error(transparent)]
    #[diagnostic(transparent)]
    FuseBlown(#[from] FuseBlown),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Assembly(Box<AssemblyError>),
}
