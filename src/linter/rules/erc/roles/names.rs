//! Name heuristics: what a pin or net name suggests when no attribute declares it.
//!
//! These are the only name rules of the linter and the code generator. A rule uses them
//! only as a fallback, at warning level.

/// Tokens of a supply pin or rail name.
const SUPPLY: &[&str] = &[
    "VDD", "VCC", "VIN", "VOUT", "VBAT", "BAT", "VBUS", "VSYS", "VSUP", "AVDD", "DVDD", "3V3", "5V",
];

/// Exact net names the code generator treats as rails, never as GPIO.
const RAIL_NETS: &[&str] = &[
    "GND", "VSS", "VCC", "VDD", "+3V3", "3V3", "+5V", "5V", "VBUS",
];

/// Words that name a rail on their own.
const RAIL_WORDS: &[&str] = &["BAT", "3V3", "+3V3", "5V", "+5V", "1V8", "+1V8"];

/// Words that make a name a signal, whatever voltage it carries (`MODEM_RX_1V8`, `VBAT_SENSE`).
const SIGNAL_WORDS: &[&str] = &[
    "RX", "TX", "RXD", "TXD", "CTS", "RTS", "SDA", "SCL", "SCK", "CLK", "MOSI", "MISO", "CS", "EN",
    "RST", "RESET", "INT", "IRQ", "PWM", "SENSE", "ADC", "KEY", "DATA",
];

/// Whether a pin is a supply pin by its own name (`VDD`, `AVDD`, `VBAT`, `3V3`, ...).
pub fn is_supply_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    SUPPLY.iter().any(|token| upper.contains(token))
}

/// Whether a name is a ground (`GND`, `AGND`, `VSS`, ...).
pub fn is_ground_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper.contains("GND") || upper.contains("VSS")
}

/// Whether a pin name says the pin supplies a rail (`VOUT`, `VO`, `VREG`, `3V3_OUT`).
pub fn is_power_out_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    let supply_out = upper.contains("OUT") && is_supply_name(&upper.replace("OUT", ""));
    upper.contains("VOUT") || upper == "VO" || upper.contains("VREG") || supply_out
}

/// Whether a pin name marks a pin that must stay open (`NC`, `NC3`, `DNC`, `N/C`).
pub fn is_no_connect_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    let numbered = upper
        .strip_prefix("NC")
        .is_some_and(|rest| rest.chars().all(|c| c.is_ascii_digit() || c == '_'));
    numbered || matches!(upper.as_str(), "DNC" | "N/C" | "N.C.")
}

/// Whether a pin name says nothing of its function: a bare number or `pinN`.
pub fn is_generic_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let digits = lower.strip_prefix("pin").unwrap_or(&lower);
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

/// Whether a net name names a supply rail: a word of it starts with `V` and holds a supply
/// token (`VDD3V3`, `VSYS`), is `V` and a digit (`V3_3`), or is a rail word (`BAT`, `3V3`).
/// Grounds, names with a signal word (`MODEM_RX_1V8`, `VBAT_SENSE`) and the names tscircuit
/// makes up for unnamed nets (`.R1 > .pin2 to .U1 > .VDD`) never are.
pub fn is_rail_name(name: &str) -> bool {
    if name.starts_with('.') || is_ground_name(name) {
        return false;
    }
    let upper = name.to_ascii_uppercase();
    let words: Vec<&str> = upper
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '+')
        .collect();
    if words.iter().any(|w| SIGNAL_WORDS.contains(w)) {
        return false;
    }
    let volts = |w: &str| {
        w.strip_prefix('V')
            .is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))
    };
    let supply =
        |w: &str| (w.starts_with('V') && is_supply_name(w)) || volts(w) || RAIL_WORDS.contains(&w);
    words.into_iter().any(supply)
}

/// Whether a net or pin label is one of the usual rail names, matched exactly.
pub fn is_rail_net(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    RAIL_NETS.contains(&upper.as_str())
}
