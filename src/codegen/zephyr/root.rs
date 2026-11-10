//! The root node: `zephyr,user`, `voltage-divider` inputs, `gpio-leds` and `gpio-keys`.
//!
//! Bindings: `dts/bindings/iio/afe/voltage-divider.yaml` (`io-channels`,
//! `output-ohms` the bottom leg, `full-ohms` both legs), `led/gpio-leds.yaml`
//! and `input/gpio-keys.yaml` (`gpios` per child). A button gets the internal
//! pull toward its idle level.

use crate::codegen::model::{BoardBindings, Indicator, IndicatorKind};

use super::analog::io_channel;
use super::{label, node_name, text, Out};

/// The `/ { ... };` block, when anything belongs in it.
pub(super) fn root(out: &mut Out) {
    let mut body = Out {
        b: out.b,
        soc: super::Soc::of(&out.b.mcu.chip),
        overrides: out.overrides,
        text: String::new(),
        todo: Vec::new(),
    };
    user(&mut body);
    dividers(&mut body);
    indicators(&mut body, IndicatorKind::Led);
    indicators(&mut body, IndicatorKind::Button);
    if !body.text.is_empty() {
        out.line(0, "\n/ {");
        out.text.push_str(&body.text);
        out.line(0, "};");
    }
    out.todo.extend(body.todo);
}

fn user(out: &mut Out) {
    let b: &BoardBindings = out.b;
    let channels: Vec<String> = b.adc.iter().filter_map(|a| io_channel(out, a)).collect();
    let mut lines = Vec::new();
    if !channels.is_empty() {
        lines.push(format!("io-channels = {};", channels.join(", ")));
    }
    let taken = |signal: &str| b.indicators.iter().any(|i| i.pin.signal == signal);
    for pin in b.gpio.iter().filter(|p| !taken(&p.signal)) {
        match out.soc.gpio(pin) {
            Some(r) => lines.push(format!(
                "{}-gpios = <{r} GPIO_ACTIVE_HIGH>;",
                node_name(&pin.signal)
            )),
            None => out.todo(
                1,
                &pin.label,
                "no Zephyr GPIO controller is known for this SoC",
            ),
        }
    }
    if lines.is_empty() {
        return;
    }
    out.line(1, "zephyr,user {");
    lines.iter().for_each(|l| out.line(2, l));
    out.line(1, "};");
}

fn dividers(out: &mut Out) {
    let b: &BoardBindings = out.b;
    for input in &b.adc {
        let (Some(divider), Some(channel)) = (input.divider, io_channel(out, input)) else {
            continue;
        };
        let signal = &input.pin.signal;
        gap(out);
        out.line(1, &format!("{}: {} {{", label(signal), node_name(signal)));
        out.line(2, "compatible = \"voltage-divider\";");
        out.line(2, &format!("io-channels = {channel};"));
        out.line(
            2,
            &format!("output-ohms = <{}>;", divider.bottom.round() as u64),
        );
        out.line(
            2,
            &format!(
                "full-ohms = <{}>;",
                (divider.top + divider.bottom).round() as u64
            ),
        );
        out.line(1, "};");
    }
}

fn indicators(out: &mut Out, kind: IndicatorKind) {
    let b: &BoardBindings = out.b;
    let found: Vec<(&Indicator, String)> = b
        .indicators
        .iter()
        .filter(|i| i.kind == kind)
        .filter_map(|i| Some((i, out.soc.gpio(&i.pin)?)))
        .collect();
    if found.is_empty() {
        return;
    }
    let (node, compatible) = match kind {
        IndicatorKind::Led => ("fusible-leds", "gpio-leds"),
        IndicatorKind::Button => ("fusible-keys", "gpio-keys"),
    };
    gap(out);
    out.line(1, &format!("{node} {{"));
    out.line(2, &format!("compatible = \"{compatible}\";"));
    for (indicator, gpio) in found {
        let device = &indicator.device;
        out.line(2, &format!("{}: {} {{", label(device), node_name(device)));
        out.line(3, &format!("gpios = <{gpio} {}>;", flags(indicator)));
        out.line(3, &format!("label = \"{}\";", text(&indicator.pin.signal)));
        out.line(2, "};");
    }
    out.line(1, "};");
}

/// A blank line between two nodes of the root block.
fn gap(out: &mut Out) {
    if !out.text.is_empty() {
        out.text.push('\n');
    }
}

fn flags(indicator: &Indicator) -> &'static str {
    match (indicator.kind, indicator.active_low) {
        (IndicatorKind::Led, false) => "GPIO_ACTIVE_HIGH",
        (IndicatorKind::Led, true) => "GPIO_ACTIVE_LOW",
        (IndicatorKind::Button, false) => "(GPIO_ACTIVE_HIGH | GPIO_PULL_DOWN)",
        (IndicatorKind::Button, true) => "(GPIO_ACTIVE_LOW | GPIO_PULL_UP)",
    }
}
