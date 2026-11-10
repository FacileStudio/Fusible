//! Arguments of `fusible explain`.

use clap::Args;

#[derive(Debug, Args, PartialEq)]
pub struct ExplainArgs {
    /// Rule id, such as `trace_current`
    #[arg(value_name = "RULE")]
    pub rule: String,

    /// Print one JSON object
    #[arg(long)]
    pub json: bool,
}
