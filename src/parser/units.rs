//! Tolerant readers for physical quantities.
//!
//! circuit-json normalises values to plain numbers in SI base units, but
//! hand-written files and older generators send strings such as `"3.3V"`,
//! `"100nF"` or `"10kΩ"`. Both spellings read to the same `f64`.

use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

const PREFIXES: [(char, f64); 12] = [
    ('f', 1e-15),
    ('p', 1e-12),
    ('n', 1e-9),
    ('u', 1e-6),
    ('µ', 1e-6),
    ('μ', 1e-6),
    ('m', 1e-3),
    ('k', 1e3),
    ('K', 1e3),
    ('M', 1e6),
    ('G', 1e9),
    ('T', 1e12),
];

fn split_number(text: &str) -> Option<(f64, &str)> {
    let ends = text.char_indices().map(|(i, c)| i + c.len_utf8()).rev();
    ends.filter_map(|end| Some((text[..end].parse::<f64>().ok()?, &text[end..])))
        .find(|(value, _)| value.is_finite())
}

/// Reads `"3.3V"`, `"100 nF"`, `"10k"`, `"1e-7"` or `"4.7kΩ"` as a number in base units.
///
/// The unit letters after the optional SI prefix are not checked; a range,
/// prose or an empty string is `None`.
pub fn si_value(text: &str) -> Option<f64> {
    let (number, rest) = split_number(text.trim())?;
    let rest = rest.trim_start();
    let mut chars = rest.chars();
    let prefix = chars
        .next()
        .and_then(|first| PREFIXES.iter().find(|(c, _)| *c == first));
    let (scale, unit) = prefix.map_or((1.0, rest), |(_, scale)| (*scale, chars.as_str()));
    let unit_ok = unit.chars().all(|c| c.is_alphabetic() || c == 'Ω');
    unit_ok.then_some(number * scale)
}

/// Serde reader for an optional quantity sent as a number or a string.
///
/// A value that is neither, or a string [`si_value`] cannot read, is an
/// error: the caller decides whether that rejects the element.
pub fn quantity<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_f64()
            .filter(|v| v.is_finite())
            .map(Some)
            .ok_or_else(|| D::Error::custom("quantity is not a finite number")),
        Some(Value::String(s)) => si_value(&s)
            .map(Some)
            .ok_or_else(|| D::Error::custom(format!("cannot read quantity `{s}`"))),
        Some(other) => Err(D::Error::custom(format!(
            "quantity `{other}` is not a number or a string"
        ))),
    }
}
