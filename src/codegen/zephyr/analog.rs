//! ADC controller nodes with one channel node per analog input.
//!
//! Channel properties follow `dts/bindings/adc/adc-controller.yaml`, which
//! requires `zephyr,gain`, `zephyr,reference` and `zephyr,acquisition-time`:
//! unity gain, the internal reference and the default acquisition time, the
//! values every supported SoC driver accepts. Review them against the input
//! range before use.

use crate::codegen::model::AdcInput;
use crate::codegen::view::Family;

use super::{text, Out};

/// `<&adc1 1>`: the `io-channels` entry of an input, where Zephyr has an ADC node for it.
pub(super) fn io_channel(out: &Out, input: &AdcInput) -> Option<String> {
    let channel = input.channel?;
    Some(format!("<&{} {}>", out.soc.adc(channel)?, channel.1))
}

/// Every ADC node the inputs use, and a TODO per input without a known channel.
pub(super) fn adc_nodes(out: &mut Out) {
    let b = out.b;
    let mut nodes: Vec<(String, Vec<&AdcInput>)> = Vec::new();
    for input in &b.adc {
        let Some(adc) = input.channel.and_then(|c| out.soc.adc(c)) else {
            let what = format!("ADC input {} ({})", input.pin.label, input.pin.signal);
            out.line(0, "");
            out.todo(
                0,
                &what,
                "the silicon table gives no ADC channel Zephyr can name for this pin",
            );
            continue;
        };
        match nodes.iter_mut().find(|(n, _)| *n == adc) {
            Some((_, inputs)) => inputs.push(input),
            None => nodes.push((adc, vec![input])),
        }
    }
    for (adc, inputs) in nodes {
        node(out, &adc, &inputs);
    }
}

fn node(out: &mut Out, adc: &str, inputs: &[&AdcInput]) {
    out.line(0, &format!("\n&{adc} {{"));
    let pinctrl: Vec<String> = match out.soc.family {
        Family::Stm32 => inputs
            .iter()
            .filter_map(|a| Some(format!("&{}", out.soc.adc_pin(a.channel?, &a.pin))))
            .collect(),
        Family::Rp => vec!["&adc_fusible".to_string()],
        _ => Vec::new(),
    };
    if !pinctrl.is_empty() {
        out.line(1, &format!("pinctrl-0 = <{}>;", pinctrl.join(" ")));
        out.line(1, "pinctrl-names = \"default\";");
    }
    out.line(1, "#address-cells = <1>;");
    out.line(1, "#size-cells = <0>;");
    out.line(1, "status = \"okay\";");
    let mut channels: Vec<(u8, &str)> = inputs
        .iter()
        .filter_map(|a| Some((a.channel?.1, a.pin.signal.as_str())))
        .collect();
    channels.sort();
    channels.dedup_by_key(|(ch, _)| *ch);
    for (ch, signal) in channels {
        channel(out, ch, signal);
    }
    out.line(0, "};");
}

fn channel(out: &mut Out, ch: u8, signal: &str) {
    let resolution = out.soc.resolution();
    out.line(0, "");
    out.line(1, &format!("channel@{ch:x} {{"));
    out.line(2, &format!("/* {} */", text(signal)));
    out.line(2, &format!("reg = <{ch}>;"));
    out.line(2, "zephyr,gain = \"ADC_GAIN_1\";");
    out.line(2, "zephyr,reference = \"ADC_REF_INTERNAL\";");
    out.line(2, "zephyr,acquisition-time = <ADC_ACQ_TIME_DEFAULT>;");
    out.line(2, &format!("zephyr,resolution = <{resolution}>;"));
    out.line(1, "};");
}
