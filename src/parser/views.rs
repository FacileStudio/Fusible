//! Per-type views over the typed elements of a [`Circuit`].

use super::{Circuit, CircuitElement, Component, Net, Pad, PcbPort, Port, Trace};

impl Circuit {
    /// Every `source_component`, in file order.
    pub fn components(&self) -> impl Iterator<Item = &Component> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::Component(c) => Some(c),
            _ => None,
        })
    }

    /// Every `source_net`, in file order.
    pub fn nets(&self) -> impl Iterator<Item = &Net> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::Net(n) => Some(n),
            _ => None,
        })
    }

    /// Every `source_port`, in file order.
    pub fn ports(&self) -> impl Iterator<Item = &Port> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::Port(p) => Some(p),
            _ => None,
        })
    }

    /// Every `source_pad` and `pcb_smtpad`, in file order.
    pub fn pads(&self) -> impl Iterator<Item = &Pad> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::Pad(p) => Some(p),
            _ => None,
        })
    }

    /// Every `pcb_port`, in file order.
    pub fn pcb_ports(&self) -> impl Iterator<Item = &PcbPort> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::PcbPort(p) => Some(p),
            _ => None,
        })
    }

    /// Every `source_trace`, in file order.
    pub fn traces(&self) -> impl Iterator<Item = &Trace> {
        self.elements.iter().filter_map(|e| match e {
            CircuitElement::Trace(t) => Some(t),
            _ => None,
        })
    }
}
