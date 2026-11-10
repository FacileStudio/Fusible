//! The one place that classifies a pin: its electrical [`Role`].
//!
//! A role comes from the pin attributes tscircuit writes on every `source_port`
//! ([`Role::declared`]), or, when the design declares nothing, from the pin's name, its
//! net's name and its part ([`Role::guessed`]). A rule that acts on a guessed role reports
//! at warning level, never at error level.

mod names;

pub use names::{
    is_generic_name, is_ground_name, is_no_connect_name, is_power_out_name, is_rail_name,
    is_rail_net, is_supply_name,
};

use super::parts::PartKind;
use crate::parser::PinAttributes;

/// What a pin does on its net. The order is the row order of the pin conflict matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    PowerIn,
    PowerOut,
    GroundIn,
    GroundOut,
    Output,
    OpenDrain,
    Input,
    Bidirectional,
    Passive,
    NoConnect,
    Unknown,
}

impl Role {
    /// The role the pin attributes declare, first match wins; `None` when none is set.
    pub fn declared(a: &PinAttributes) -> Option<Self> {
        [
            (a.do_not_connect, Self::NoConnect),
            (a.provides_power, Self::PowerOut),
            (a.requires_power, Self::PowerIn),
            (a.provides_ground, Self::GroundOut),
            (a.requires_ground, Self::GroundIn),
            (a.is_using_open_drain, Self::OpenDrain),
            (a.is_using_open_collector, Self::OpenDrain),
            (a.is_using_push_pull, Self::Output),
            (a.is_output, Self::Output),
            (a.is_bidirectional, Self::Bidirectional),
            (a.is_input, Self::Input),
            (a.is_passive, Self::Passive),
        ]
        .into_iter()
        .find(|(set, _)| *set == Some(true))
        .map(|(_, role)| role)
    }

    /// The role a pin's name suggests. A pin named only by number takes its net's name.
    pub fn guessed(pin: &str, net: &str, part: PartKind) -> Self {
        let generic = is_generic_name(pin);
        let name = match generic {
            true => net,
            false => pin,
        };
        match part {
            PartKind::Chip | PartKind::Other if generic => by_net(net),
            PartKind::Chip | PartKind::Other => by_name(name),
            PartKind::Battery | PartKind::Source if is_ground_name(name) => Self::GroundOut,
            PartKind::Battery | PartKind::Source => Self::PowerOut,
            _ => Self::Passive,
        }
    }

    /// How a message names the role.
    pub fn label(self) -> &'static str {
        match self {
            Self::PowerIn => "power input",
            Self::PowerOut => "power output",
            Self::GroundIn => "ground input",
            Self::GroundOut => "ground output",
            Self::Output => "push-pull output",
            Self::OpenDrain => "open-drain output",
            Self::Input => "input",
            Self::Bidirectional => "bidirectional",
            Self::Passive => "passive",
            Self::NoConnect => "no-connect",
            Self::Unknown => "unspecified",
        }
    }
}

fn by_name(name: &str) -> Role {
    if is_no_connect_name(name) {
        Role::NoConnect
    } else if is_ground_name(name) {
        Role::GroundIn
    } else if is_power_out_name(name) {
        Role::PowerOut
    } else if is_supply_name(name) {
        Role::PowerIn
    } else {
        Role::Unknown
    }
}

fn by_net(net: &str) -> Role {
    if is_ground_name(net) {
        Role::GroundIn
    } else if is_rail_name(net) {
        Role::PowerIn
    } else {
        Role::Unknown
    }
}

/// Whether the attributes declare any role at all, `false` values included.
pub fn declares_role(a: &PinAttributes) -> bool {
    [
        a.do_not_connect,
        a.provides_power,
        a.requires_power,
        a.provides_ground,
        a.requires_ground,
        a.is_using_open_drain,
        a.is_using_open_collector,
        a.is_using_push_pull,
        a.is_output,
        a.is_bidirectional,
        a.is_input,
        a.is_passive,
    ]
    .iter()
    .any(Option::is_some)
}
