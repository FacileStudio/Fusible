//! The designator of the part a pad or PCB component belongs to.

use std::collections::HashMap;

use crate::linter::LintContext;
use crate::parser::Pad;

/// Lookups from PCB ids to part designators such as `U1`.
pub(in crate::linter::rules) struct Owners<'a> {
    pcb_port: HashMap<&'a str, &'a str>,
    port_part: HashMap<&'a str, &'a str>,
    pcb_part: HashMap<&'a str, &'a str>,
    names: HashMap<&'a str, &'a str>,
}

impl<'a> Owners<'a> {
    /// Indexes the ports, PCB components and source components of `ctx`.
    pub fn new(ctx: &LintContext<'a>) -> Self {
        let circuit = ctx.circuit;
        let pcb_port = circuit
            .pcb_ports()
            .filter_map(|p| Some((p.pcb_port_id.as_deref()?, p.source_port_id.as_deref()?)));
        let port_part = circuit
            .ports()
            .filter_map(|p| Some((p.port_id.as_deref()?, p.source_component_id.as_deref()?)));
        let model = circuit.model();
        let pcb_part = model
            .components
            .iter()
            .map(|c| (c.pcb_component_id.as_str(), c.source_component_id.as_str()));
        let names = circuit.components().filter_map(|c| {
            let id = c.component_id.as_deref()?;
            Some((id, c.name.as_deref().unwrap_or(id)))
        });
        Self {
            pcb_port: pcb_port.collect(),
            port_part: port_part.collect(),
            pcb_part: pcb_part.collect(),
            names: names.collect(),
        }
    }

    /// The designator of the part whose port `pad` realizes.
    pub fn of_pad(&self, pad: &Pad) -> Option<&'a str> {
        let port = pad.port_id.as_deref().or_else(|| {
            let pcb = pad.pcb_port_id.as_deref()?;
            self.pcb_port.get(pcb).copied()
        })?;
        let part = self.port_part.get(port)?;
        self.names.get(part).copied()
    }

    /// The designator of the source component behind `pcb_component_id`.
    pub fn of_component(&self, pcb_component_id: &str) -> Option<&'a str> {
        let part = self.pcb_part.get(pcb_component_id)?;
        self.names.get(part).copied().or(Some(part))
    }
}
