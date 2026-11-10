//! A read-only view of one board for the bring-up sections: ports by net, nets by port.

use crate::netlist::{NetNode, Netlist};
use crate::parser::{Circuit, Component, Port};

/// The parsed board, its netlist and the MCU picked for it.
pub struct View<'a> {
    pub circuit: &'a Circuit,
    pub netlist: &'a Netlist,
    pub mcu_id: Option<String>,
}

/// One pin of a component on a net.
pub struct Pin<'a> {
    pub component: &'a Component,
    pub port: &'a Port,
}

impl Pin<'_> {
    /// `R3.pin1`, from the component name and the port name.
    pub fn label(&self) -> String {
        let comp = self
            .component
            .name
            .as_deref()
            .or(self.component.component_id.as_deref());
        format!(
            "{}.{}",
            comp.unwrap_or("?"),
            self.port.name.as_deref().unwrap_or("?")
        )
    }

    pub fn kind(&self) -> &str {
        self.component.kind.as_deref().unwrap_or_default()
    }
}

impl<'a> View<'a> {
    /// The net a port sits on.
    pub fn net_of(&self, port: &Port) -> Option<&'a NetNode> {
        let index = *self.netlist.port_to_net.get(port.port_id.as_deref()?)?;
        self.netlist.nets.get(index)
    }

    /// Every component pin on the net named `net`.
    pub fn pins_on(&self, net: &str) -> Vec<Pin<'a>> {
        self.circuit
            .ports()
            .filter(|p| self.net_of(p).is_some_and(|n| n.display_name() == net))
            .filter_map(|port| {
                Some(Pin {
                    component: self.component(port.source_component_id.as_deref()?)?,
                    port,
                })
            })
            .collect()
    }

    /// The pins of the component with `id`.
    pub fn pins_of(&self, id: &str) -> Vec<&'a Port> {
        self.circuit
            .ports()
            .filter(|p| p.source_component_id.as_deref() == Some(id))
            .collect()
    }

    pub fn component(&self, id: &str) -> Option<&'a Component> {
        self.circuit
            .components()
            .find(|c| c.component_id.as_deref() == Some(id))
    }

    /// True when `pin` belongs to the MCU.
    pub fn is_mcu(&self, pin: &Pin) -> bool {
        self.mcu_id.is_some() && pin.component.component_id == self.mcu_id
    }
}

/// True for a ground net name.
pub fn is_ground(net: &str) -> bool {
    let upper = net.to_ascii_uppercase();
    upper.contains("GND") || upper == "VSS" || upper == "0V"
}
