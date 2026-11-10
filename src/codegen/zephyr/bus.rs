//! One bus controller node with its pinctrl, chip selects and device children.

use crate::codegen::view::{Bus, Family, Kind};

use super::device::{facts, Facts};
use super::{label, node_name, pinctrl, text, Out};

/// The `&i2c1 { ... }` node of `bus`, or a TODO when no controller instance is known.
pub(super) fn node(out: &mut Out, bus: &Bus) {
    let controller = bus.name.to_ascii_lowercase();
    let Some(pinctrl) = pinctrl(out, bus, &controller) else {
        let pins: Vec<String> = bus
            .lines
            .iter()
            .map(|(role, p)| format!("{}={}", role.to_ascii_uppercase(), p.label))
            .collect();
        out.line(0, "");
        let why = "the silicon table gives no controller instance for these pins; describe the bus by hand";
        out.todo(0, &format!("{} ({})", bus.name, pins.join(", ")), why);
        return;
    };
    out.line(0, &format!("\n&{controller} {{"));
    out.line(1, &format!("pinctrl-0 = <{pinctrl}>;"));
    out.line(1, "pinctrl-names = \"default\";");
    out.line(1, "status = \"okay\";");
    cs_gpios(out, bus);
    children(out, bus, &controller);
    out.line(0, "};");
}

fn pinctrl(out: &Out, bus: &Bus, controller: &str) -> Option<String> {
    bus.instance?;
    match out.soc.family {
        Family::Stm32 => {
            let labels: Vec<String> = bus
                .lines
                .iter()
                .map(|(role, p)| format!("&{controller}_{role}_{}", p.label.to_ascii_lowercase()))
                .collect();
            Some(labels.join(" "))
        }
        _ => out
            .soc
            .pinctrl_header()
            .map(|_| format!("&{}", pinctrl::state_name(bus))),
    }
}

fn cs_gpios(out: &mut Out, bus: &Bus) {
    let refs: Vec<String> = bus
        .chip_selects
        .iter()
        .filter_map(|(pin, _)| out.soc.gpio(pin))
        .map(|r| format!("<{r} GPIO_ACTIVE_LOW>"))
        .collect();
    if !refs.is_empty() {
        out.line(1, &format!("cs-gpios = {};", refs.join(", ")));
    }
}

fn children(out: &mut Out, bus: &Bus, controller: &str) {
    match bus.kind {
        Kind::Spi => {
            let selected: Vec<(usize, String)> = bus
                .chip_selects
                .iter()
                .enumerate()
                .filter_map(|(i, (_, dev))| Some((i, (*dev)?.to_string())))
                .collect();
            for (cs, name) in selected {
                child(out, &name, bus.kind, controller, Some(cs));
            }
        }
        _ => {
            for name in bus.devices.clone() {
                child(out, &name, bus.kind, controller, None);
            }
        }
    }
}

fn child(out: &mut Out, name: &str, kind: Kind, controller: &str, cs: Option<usize>) {
    let part = out.b.parts.get(name).cloned();
    let mpn = part
        .as_ref()
        .and_then(|p| p.mpn.clone())
        .unwrap_or_default();
    let what = format!("{name} ({mpn}) on {controller}");
    if !part.as_ref().is_some_and(|p| p.is_chip()) {
        let note = format!(
            "/* {}: not a chip; describe what it connects to by hand */",
            text(&what)
        );
        return out.line(1, &note);
    }
    match facts(name, part.as_ref(), kind, out.overrides) {
        Ok(facts) => known(out, name, &facts, cs),
        Err(_) if kind == Kind::Uart => {
            let note = format!("/* {}: no Zephyr UART-device binding known */", text(&what));
            out.line(1, &note);
        }
        Err(why) => {
            let why = format!("{why} in `firmware.<name>.zephyr.{name}` of fusible.yml");
            out.todo(1, &what, &why);
            missing(out, name, kind, cs);
        }
    }
}

fn known(out: &mut Out, name: &str, f: &Facts, cs: Option<usize>) {
    let reg = cs.map(|i| i as u16).or(f.reg);
    let unit = reg.map(|r| format!("@{r:x}")).unwrap_or_default();
    out.line(1, &format!("{}: {}{unit} {{", label(name), f.node));
    out.line(2, &format!("compatible = \"{}\";", f.compatible));
    if let Some(r) = reg {
        out.line(2, &format!("reg = <{r:#x}>;"));
    }
    if let Some(hz) = f.spi_hz {
        out.line(2, &format!("spi-max-frequency = <{hz}>;"));
    }
    out.line(2, "status = \"okay\";");
    out.line(1, "};");
}

fn missing(out: &mut Out, name: &str, kind: Kind, cs: Option<usize>) {
    let unit = cs.map(|i| format!("@{i:x}")).unwrap_or_default();
    out.line(1, &format!("{}: {}{unit} {{", label(name), node_name(name)));
    if let (Kind::Spi, Some(i)) = (kind, cs) {
        out.line(2, &format!("reg = <{i}>;"));
    }
    out.line(2, "status = \"disabled\";");
    out.line(1, "};");
}

/// Warns about each `zephyr:` entry of the manifest that names no device on a bus.
pub(super) fn unused_overrides(out: &mut Out, buses: &[Bus]) {
    let on_bus: Vec<&String> = buses.iter().flat_map(|b| &b.devices).collect();
    let unused: Vec<String> = out
        .overrides
        .keys()
        .filter(|name| !on_bus.contains(name))
        .cloned()
        .collect();
    for name in unused {
        let why = "names no device on an I2C, SPI or UART bus of this board, so it is unused";
        out.todo
            .push(format!("`firmware.<name>.zephyr.{name}` {why}"));
    }
}
