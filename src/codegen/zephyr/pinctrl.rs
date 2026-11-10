//! Pinctrl states of the RP2040, RP2350 and ESP32 buses and of the RP ADC.
//!
//! Macro names are those of Zephyr's `include/zephyr/dt-bindings/pinctrl/`:
//! `rpi-pico-*-pinctrl.h` (`I2C0_SDA_P4`, `UART0_TX_P0`, `SPI0_SCK_P2`,
//! `SPI0_TX_P3` for MOSI, `SPI0_RX_P0` for MISO, `ADC_CH0_P26`) and
//! `esp32*-pinctrl.h` (`I2C0_SDA_GPIO1`, `UART0_TX_GPIO43`, `SPIM2_SCLK_GPIO12`).
//! Group properties follow the Zephyr `rpi_pico` and `esp32s3_devkitc` boards.

use crate::codegen::model::BusPin;
use crate::codegen::view::{Bus, Family, Kind};

use super::{Out, Soc};

type State = (String, Vec<(String, &'static str)>);

/// The `&pinctrl` block, for families that use pinctrl groups.
pub(super) fn block(out: &mut Out, buses: &[Bus]) {
    if out.soc.pinctrl_header().is_none() {
        return;
    }
    let states: Vec<State> = buses
        .iter()
        .filter_map(|bus| Some((state_name(bus), lines(&out.soc, bus)?)))
        .chain(adc_state(out))
        .collect();
    if states.is_empty() {
        return;
    }
    out.line(0, "\n&pinctrl {");
    for (name, pins) in &states {
        state(out, name, pins);
    }
    out.line(0, "};");
}

/// `i2c0_fusible`: the state a bus node points `pinctrl-0` at.
pub(super) fn state_name(bus: &Bus) -> String {
    format!("{}_fusible", bus.name.to_ascii_lowercase())
}

fn lines(soc: &Soc, bus: &Bus) -> Option<Vec<(String, &'static str)>> {
    let n = bus.instance?;
    let lines = bus.lines.iter().map(|(role, pin)| {
        let mux = raw_uart(soc, bus, pin).unwrap_or_else(|| mux(soc.family, bus, n, role, pin));
        (mux, props(soc.family, bus.kind, role))
    });
    Some(lines.collect())
}

fn mux(family: Family, bus: &Bus, n: u8, role: &str, pin: &BusPin) -> String {
    let (prefix, role) = match (family, bus.kind, role) {
        (Family::Rp, Kind::Spi, "mosi") => (format!("SPI{n}"), "TX".to_string()),
        (Family::Rp, Kind::Spi, "miso") => (format!("SPI{n}"), "RX".to_string()),
        (Family::Esp, Kind::Spi, "sck") => (format!("SPIM{n}"), "SCLK".to_string()),
        (Family::Esp, Kind::Spi, _) => (format!("SPIM{n}"), role.to_ascii_uppercase()),
        _ => (bus.name.clone(), role.to_ascii_uppercase()),
    };
    let pin = match family {
        Family::Rp => format!("P{}", pin.number),
        _ => format!("GPIO{}", pin.number),
    };
    format!("{prefix}_{role}_{pin}")
}

/// Zephyr's `rpi-pico-rp2350b-pinctrl.h` names the F2 UART functions of
/// GPIO30-47 after the GPIO0-29 pattern (`UART0_TX_P36` where the RP2350
/// datasheet and pico-sdk have UART1 TX), so those pins get the raw
/// `RP2XXX_PINMUX` with the function number the silicon table gives.
fn raw_uart(soc: &Soc, bus: &Bus, pin: &BusPin) -> Option<String> {
    let function = match pin.af? {
        2 => "UART",
        11 => "UART_ALT",
        _ => return None,
    };
    let affected = soc.is("rp2350b") && bus.kind == Kind::Uart && pin.number >= 30;
    affected.then(|| {
        format!(
            "RP2XXX_PINMUX({}, RP2_PINCTRL_GPIO_FUNC_{function})",
            pin.number
        )
    })
}

fn props(family: Family, kind: Kind, role: &str) -> &'static str {
    match (family, kind, role) {
        (Family::Rp, Kind::I2c, _) => "input-enable;\ninput-schmitt-enable;",
        (Family::Rp, Kind::Uart, "rx") | (Family::Rp, Kind::Spi, "miso") => "input-enable;",
        (Family::Esp, Kind::I2c, _) => "bias-pull-up;\ndrive-open-drain;\noutput-high;",
        (Family::Esp, Kind::Uart, "tx") => "output-high;",
        (Family::Esp, Kind::Uart, "rx") => "bias-pull-up;",
        (Family::Esp, Kind::Spi, "mosi") => "output-low;",
        _ => "",
    }
}

/// `adc_fusible` with every RP ADC input the silicon table maps to a channel.
fn adc_state(out: &Out) -> Option<State> {
    let pins: Vec<(String, &'static str)> = out
        .b
        .adc
        .iter()
        .filter(|_| out.soc.family == Family::Rp)
        .filter_map(|a| {
            Some((
                format!("ADC_CH{}_P{}", a.channel?.1, a.pin.number),
                "input-enable;",
            ))
        })
        .collect();
    (!pins.is_empty()).then(|| ("adc_fusible".to_string(), pins))
}

/// One state, its pins grouped by shared properties in first-seen order.
fn state(out: &mut Out, name: &str, pins: &[(String, &'static str)]) {
    let mut groups: Vec<(&'static str, Vec<String>)> = Vec::new();
    for (mux, props) in pins {
        match groups.iter_mut().find(|(p, _)| p == props) {
            Some((_, muxes)) => muxes.push(format!("<{mux}>")),
            None => groups.push((props, vec![format!("<{mux}>")])),
        }
    }
    out.line(1, &format!("{name}: {name} {{"));
    for (i, (props, muxes)) in groups.iter().enumerate() {
        out.line(2, &format!("group{} {{", i + 1));
        out.line(3, &format!("pinmux = {};", muxes.join(", ")));
        props.lines().for_each(|p| out.line(3, p));
        out.line(2, "};");
    }
    out.line(1, "};");
}
