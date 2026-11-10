//! Whether a waiver's `match` names a diagnostic.
//!
//! A waiver matches when its rule id equals the diagnostic's and its `match` is one of:
//! * an element id of the diagnostic, exactly;
//! * the name of a component the diagnostic lists by id;
//! * `REFDES.PIN` or `REFDES.NUMBER` of a port the diagnostic lists by id;
//! * `REFDES.PIN` where `REFDES` is a component the diagnostic lists, `PIN` is the name of
//!   one of its ports, and the message carries `PIN` as a whole word. Rules such as
//!   `decoupling_capacitor` list the parts, not the port, so this is the only way to tell
//!   two pins of one chip apart;
//! * the same `REFDES.PIN` on a diagnostic that lists no element at all, when the message
//!   carries both `REFDES` and `PIN` as whole words. This message fallback is the last
//!   resort, for rules that report no element ids.
//!
//! Pin numbers never match through the message, only through a listed port.

use crate::config::Waiver;
use crate::linter::Diagnostic;
use crate::parser::{Circuit, Component, Port};

/// Whether `waiver` names `diagnostic`; `circuit` resolves names and pins when known.
pub(super) fn matches(waiver: &Waiver, diagnostic: &Diagnostic, circuit: Option<&Circuit>) -> bool {
    if waiver.rule != diagnostic.rule {
        return false;
    }
    let wanted = waiver.matches.trim();
    let named = diagnostic
        .elements
        .iter()
        .any(|element| aliases(element, circuit).iter().any(|a| a == wanted));
    named || circuit.is_some_and(|c| pin_in_message(wanted, diagnostic, c))
}

fn component<'a>(circuit: &'a Circuit, key: &str) -> Option<&'a Component> {
    circuit
        .components()
        .find(|c| c.component_id.as_deref() == Some(key) || c.name.as_deref() == Some(key))
}

fn port_aliases(circuit: &Circuit, port: &Port) -> Vec<String> {
    let owner = port.source_component_id.as_deref();
    let Some(refdes) = owner.and_then(|id| component(circuit, id)?.name.clone()) else {
        return Vec::new();
    };
    let pins = [port.name.clone(), port.pin_number.map(|n| n.to_string())];
    pins.into_iter()
        .flatten()
        .map(|pin| format!("{refdes}.{pin}"))
        .collect()
}

fn aliases(element: &str, circuit: Option<&Circuit>) -> Vec<String> {
    let mut found = vec![element.to_string()];
    let Some(circuit) = circuit else {
        return found;
    };
    found.extend(component(circuit, element).and_then(|c| c.name.clone()));
    let port = circuit
        .ports()
        .find(|p| p.port_id.as_deref() == Some(element));
    found.extend(port.map(|p| port_aliases(circuit, p)).unwrap_or_default());
    found
}

fn said(message: &str, word: &str) -> bool {
    message
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .any(|w| w == word)
}

fn pin_in_message(wanted: &str, diagnostic: &Diagnostic, circuit: &Circuit) -> bool {
    let Some((refdes, pin)) = wanted.split_once('.') else {
        return false;
    };
    let Some(chip) = component(circuit, refdes) else {
        return false;
    };
    let message = diagnostic.message.as_str();
    let owner = match diagnostic.elements.is_empty() {
        true => said(message, refdes),
        false => diagnostic
            .elements
            .iter()
            .any(|e| component(circuit, e).is_some_and(|c| std::ptr::eq(c, chip))),
    };
    let has_pin = circuit
        .ports()
        .any(|p| p.source_component_id == chip.component_id && p.name.as_deref() == Some(pin));
    owner && has_pin && pin.parse::<u32>().is_err() && said(message, pin)
}
