//! One pin of the board as the electrical rules see it: its port, part and net.

use super::parts::PartKind;
use super::roles::Role;
use crate::parser::{Component, PinAttributes, Port};

/// A `source_port` with its part and net resolved.
#[derive(Debug, Clone)]
pub struct Pin<'a> {
    pub port: &'a Port,
    /// The port's `source_port_id`.
    pub id: &'a str,
    pub part: Option<&'a Component>,
    /// The part's `source_component_id`, else the `U1` of a `U1.1` port id, else the port id.
    pub part_id: &'a str,
    pub kind: PartKind,
    /// Index of the pin's net in the netlist, `None` for a pin on no net.
    pub net: Option<usize>,
    /// The net's display name, empty for a pin on no net.
    pub net_name: &'a str,
}

impl<'a> Pin<'a> {
    /// The part's designator, e.g. `U1`.
    pub fn refdes(&self) -> &'a str {
        self.part
            .and_then(|c| c.name.as_deref())
            .unwrap_or(self.part_id)
    }

    /// `REFDES.PIN`, e.g. `U1.VDD`, the form waivers match.
    pub fn label(&self) -> String {
        let pin = self.port.name.clone();
        let pin = pin.or_else(|| self.port.pin_number.map(|n| n.to_string()));
        format!(
            "{}.{}",
            self.refdes(),
            pin.unwrap_or_else(|| self.id.into())
        )
    }

    /// The pin's declared attributes.
    pub fn attributes(&self) -> &'a PinAttributes {
        &self.port.attributes
    }

    /// The role the pin's name suggests.
    pub fn guessed(&self) -> Role {
        let name = self.port.name.as_deref().unwrap_or("");
        Role::guessed(name, self.net_name, self.kind)
    }

    /// The declared role, else the guessed one, with whether it was declared.
    pub fn role(&self) -> (Role, bool) {
        match Role::declared(self.attributes()) {
            Some(role) => (role, true),
            None => (self.guessed(), false),
        }
    }
}
