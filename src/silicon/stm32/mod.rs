//! STM32 pin tables generated from `stm32-data-generated`
//! (<https://github.com/embassy-rs/stm32-data-generated>, MIT OR Apache-2.0,
//! notice in `src/silicon/data/LICENSE-stm32-data`) by `scripts/silicon-gen.ts`.
//!
//! Families G0, G4, F4, L4 (with L4+) and H7 are embedded. A query names a
//! chip at least down to its line (`stm32g431`) and may add the pin count,
//! flash, package and temperature letters (`stm32g431cbu6`). It resolves to
//! every package whose name starts with the query; the target lists the GPIOs
//! all of them bond out, with the functions all of them offer.
//!
//! Only signals `PeripheralSignal` expresses are kept: USART/UART TX, RX,
//! CTS, RTS, SPI SCK, MISO, MOSI, NSS, I2C SCL, SDA, ADC inputs, USB DM/DP,
//! and SWD on PA13/PA14 (AF0, absent from the upstream data). LPUART,
//! timers, I2S, SAI, FDCAN and instances above 9 are left out.

mod parse;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use parse::{Family, Package};

use crate::silicon::model::{PinSpec, SiliconTarget};

const SOURCES: [(&str, &str); 5] = [
    ("g0", include_str!("../data/stm32g0.txt")),
    ("g4", include_str!("../data/stm32g4.txt")),
    ("f4", include_str!("../data/stm32f4.txt")),
    ("l4", include_str!("../data/stm32l4.txt")),
    ("h7", include_str!("../data/stm32h7.txt")),
];

static FAMILIES: [OnceLock<Family>; 5] = [const { OnceLock::new() }; 5];

type Cache = Mutex<HashMap<String, Option<&'static SiliconTarget>>>;

static TARGETS: OnceLock<Cache> = OnceLock::new();

/// Resolves a normalised (lowercase, no separators) STM32 query.
pub fn find(clean: &str) -> Option<&'static SiliconTarget> {
    let cache = TARGETS.get_or_init(Cache::default);
    let mut cache = cache.lock().unwrap_or_else(|e| e.into_inner());
    *cache
        .entry(clean.to_string())
        .or_insert_with(|| build(clean))
}

fn family(clean: &str) -> Option<&'static Family> {
    let key = clean.strip_prefix("stm32")?.get(..2)?;
    let index = SOURCES.iter().position(|(k, _)| *k == key)?;
    Some(FAMILIES[index].get_or_init(|| parse::parse(SOURCES[index].1)))
}

fn matches(package: &Package, clean: &str) -> bool {
    let name = package.name.as_str();
    name.starts_with(clean)
        || (name.len() == clean.len()
            && name.ends_with('x')
            && clean.starts_with(&name[..name.len() - 1]))
}

fn build(clean: &str) -> Option<&'static SiliconTarget> {
    if clean.len() < 9 {
        return None;
    }
    let family = family(clean)?;
    let packages: Vec<&Package> = family
        .packages
        .iter()
        .filter(|p| matches(p, clean))
        .collect();
    let first = packages.first()?;
    let pins: Vec<PinSpec> = first
        .bonded
        .iter()
        .filter_map(|name| common_pin(family, &packages, name))
        .collect();
    Some(Box::leak(Box::new(SiliconTarget {
        name: Box::leak(clean.to_string().into_boxed_str()),
        family: "STM32",
        pins: Box::leak(pins.into_boxed_slice()),
    })))
}

/// The pin as every package offers it, or `None` when one does not bond it.
fn common_pin(family: &Family, packages: &[&Package], name: &str) -> Option<PinSpec> {
    let specs: Vec<&PinSpec> = packages
        .iter()
        .map(|p| {
            p.bonded.contains(&name).then_some(())?;
            family.tables[p.table].iter().find(|s| s.name == name)
        })
        .collect::<Option<_>>()?;
    let mut spec = specs[0].clone();
    if specs.iter().any(|s| s.functions != spec.functions) {
        let shared: Vec<_> = spec
            .functions
            .iter()
            .filter(|f| specs.iter().all(|s| s.functions.contains(f)))
            .cloned()
            .collect();
        spec.capabilities.is_analog = specs.iter().all(|s| s.capabilities.is_analog);
        spec.functions = Box::leak(shared.into_boxed_slice());
    }
    Some(spec)
}

#[cfg(test)]
mod tests;
