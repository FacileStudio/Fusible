//! Reader for the generated STM32 family files in `src/silicon/data/`.
//!
//! A file holds `table` blocks. Each block lists one pin per line
//! (`PA9 I2C2_SCL:4 UART1_TX:7`, a token without `:af` is an ADC input),
//! then the `pkg <name> <bonded pins...>` lines of the packages sharing it.

use crate::silicon::model::{MuxFunction, PeripheralSignal, PinCapabilities, PinSpec};

/// One package of a family file and the pins it bonds out.
pub(super) struct Package {
    pub name: String,
    pub table: usize,
    pub bonded: Vec<&'static str>,
}

/// A parsed family file: its pin tables and the packages pointing at them.
#[derive(Default)]
pub(super) struct Family {
    pub tables: Vec<Vec<PinSpec>>,
    pub packages: Vec<Package>,
}

const AF_NAMES: [&str; 16] = [
    "AF0", "AF1", "AF2", "AF3", "AF4", "AF5", "AF6", "AF7", "AF8", "AF9", "AF10", "AF11", "AF12",
    "AF13", "AF14", "AF15",
];

/// Parses a family file. The pin function slices are leaked once per family.
pub(super) fn parse(text: &'static str) -> Family {
    let mut family = Family::default();
    for line in text
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let mut words = line.split(' ');
        match (words.next(), family.tables.len().checked_sub(1)) {
            (Some("table"), _) => family.tables.push(Vec::new()),
            (Some("pkg"), Some(table)) => family.packages.push(Package {
                name: words.next().unwrap_or_default().to_ascii_lowercase(),
                table,
                bonded: words.collect(),
            }),
            (Some(pin), Some(table)) => family.tables[table].push(pin_spec(pin, words)),
            _ => {}
        }
    }
    family
}

fn pin_spec<'a>(name: &'static str, tokens: impl Iterator<Item = &'a str>) -> PinSpec {
    let functions: Vec<MuxFunction> = tokens.filter_map(function).collect();
    let is_analog = functions.iter().any(|f| f.af_name == "ADC");
    PinSpec {
        gpio: None,
        name,
        capabilities: PinCapabilities {
            is_gpio: true,
            is_analog,
            is_input_only: false,
            is_reserved_flash: false,
        },
        strapping: None,
        functions: Box::leak(functions.into_boxed_slice()),
    }
}

/// Reads one `SIGNAL[:af]` token. `None` when the signal or AF is unknown.
pub(super) fn function(token: &str) -> Option<MuxFunction> {
    let (name, af) = match token.split_once(':') {
        Some((name, af)) => (name, Some(af.parse::<usize>().ok()?)),
        None => (token, None),
    };
    let signal = match name {
        "SWD_DIO" => PeripheralSignal::SwdDio,
        "SWD_CLK" => PeripheralSignal::SwdClk,
        _ => PeripheralSignal::parse(name)?,
    };
    let (af_number, af_name) = match (af, &signal) {
        (Some(af), _) => (af as u8, *AF_NAMES.get(af)?),
        (None, PeripheralSignal::UsbDm | PeripheralSignal::UsbDp) => (0, "USB"),
        (None, _) => (0, "ADC"),
    };
    Some(MuxFunction {
        signal,
        af_number,
        af_name,
    })
}
