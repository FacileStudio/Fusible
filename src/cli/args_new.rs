//! Arguments of `fusible new`, `fusible explain` and `fusible rules`.

use std::path::PathBuf;

use clap::{Args, ValueEnum};

/// A starter project `fusible new` can write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Template {
    /// ESP32-S3 module, 3.3 V LDO, USB-C power, ESP-IDF firmware
    #[value(name = "esp32-s3")]
    Esp32S3,
    /// STM32G431 in LQFP-32, 3.3 V LDO, Embassy firmware
    #[value(name = "stm32g4")]
    Stm32G4,
    /// RP2040 with its 1.1 V core rail, 3.3 V LDO, Embassy firmware
    #[value(name = "rp2040")]
    Rp2040,
    /// A board with a connector and a resistor, no MCU
    #[value(name = "blank")]
    Blank,
}

#[derive(Debug, Args, PartialEq)]
pub struct ProjectArgs {
    /// Project name; also the directory to create
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Starter design
    #[arg(long, value_enum, default_value = "blank")]
    pub template: Template,

    /// Parent directory of the new project [default: current directory]
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Skip installing the hardware dependencies with bun or npm
    #[arg(long)]
    pub no_install: bool,
}

#[derive(Debug, Args, PartialEq)]
pub struct ExplainArgs {
    /// Rule id, such as `trace_current`
    #[arg(value_name = "RULE")]
    pub rule: String,

    /// Print one JSON object
    #[arg(long)]
    pub json: bool,
}
