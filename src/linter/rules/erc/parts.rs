//! What a component is, read from its `ftype`, or from its designator when it has none.

use crate::parser::Component;

/// The kinds of part the electrical rules tell apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartKind {
    Chip,
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Led,
    Connector,
    Battery,
    Source,
    Switch,
    Fuse,
    TestPoint,
    Other,
}

/// Words of a circuit-json `ftype` (`simple_resistor`, `simple_pin_header`, ...), checked in order.
const FTYPES: &[(&str, PartKind)] = &[
    ("led", PartKind::Led),
    ("diode", PartKind::Diode),
    ("chip", PartKind::Chip),
    ("resistor", PartKind::Resistor),
    ("potentiometer", PartKind::Resistor),
    ("capacitor", PartKind::Capacitor),
    ("inductor", PartKind::Inductor),
    ("ferrite", PartKind::Inductor),
    ("connector", PartKind::Connector),
    ("pin_header", PartKind::Connector),
    ("pinheader", PartKind::Connector),
    ("battery", PartKind::Battery),
    ("power_source", PartKind::Source),
    ("voltage_source", PartKind::Source),
    ("switch", PartKind::Switch),
    ("push_button", PartKind::Switch),
    ("fuse", PartKind::Fuse),
    ("test_point", PartKind::TestPoint),
    ("testpoint", PartKind::TestPoint),
];

/// Designator prefixes, longest first, for a component without an `ftype`.
const PREFIXES: &[(&str, PartKind)] = &[
    ("LED", PartKind::Led),
    ("TP", PartKind::TestPoint),
    ("BT", PartKind::Battery),
    ("SW", PartKind::Switch),
    ("FB", PartKind::Inductor),
    ("CN", PartKind::Connector),
    ("IC", PartKind::Chip),
    ("U", PartKind::Chip),
    ("R", PartKind::Resistor),
    ("C", PartKind::Capacitor),
    ("L", PartKind::Inductor),
    ("D", PartKind::Diode),
    ("J", PartKind::Connector),
    ("F", PartKind::Fuse),
];

impl PartKind {
    /// The kind of `component` (id `id`): its `ftype` first, else its designator.
    pub fn of(component: Option<&Component>, id: &str) -> Self {
        if let Some(kind) = component.and_then(|c| c.kind.as_deref()) {
            let kind = kind.to_ascii_lowercase();
            let found = FTYPES.iter().find(|(word, _)| kind.contains(word));
            return found.map_or(Self::Other, |(_, part)| *part);
        }
        let name = component.and_then(|c| c.name.as_deref()).unwrap_or(id);
        let upper = name.to_ascii_uppercase();
        let found = PREFIXES
            .iter()
            .find(|(prefix, _)| upper.starts_with(prefix));
        found.map_or(Self::Other, |(_, part)| *part)
    }

    /// Whether DC supply passes through the part from one pin to the other.
    pub fn passes_power(self) -> bool {
        matches!(
            self,
            Self::Resistor | Self::Inductor | Self::Diode | Self::Switch | Self::Fuse
        )
    }

    /// Whether the part brings power from outside the board or stores it.
    pub fn brings_power(self) -> bool {
        matches!(self, Self::Connector | Self::Battery | Self::Source)
    }
}
