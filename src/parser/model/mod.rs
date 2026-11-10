//! Typed model of the Circuit JSON elements the netlist AST does not cover.
//!
//! [`Model::build`] reads the board, placement, copper, drill, silkscreen
//! and courtyard elements, the part data of every `source_component`, and
//! every upstream `*_error` / `*_warning`. An element of a modelled type that
//! does not deserialize is never dropped: it is listed in
//! [`Model::unreadable`], which a rule reports to keep the fuse fail-closed.

mod board;
mod issues;
mod pcb;
mod silk;

pub use super::elements::PadPoint as Point;
pub use super::props::ComponentProps;
pub use board::{InternalConnection, PcbBoard, PcbComponent};
pub use issues::UpstreamIssue;
pub use pcb::{
    BrepShape, PcbCopperPour, PcbHole, PcbPlatedHole, PcbTrace, PcbVia, Ring, RoutePoint,
};
pub use silk::{CourtyardOutline, CourtyardRect, SilkscreenPath, SilkscreenText};

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Circuit;

/// Every typed element of one circuit, grouped by type, in file order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub boards: Vec<PcbBoard>,
    pub components: Vec<PcbComponent>,
    /// Part data of each `source_component`.
    pub parts: Vec<ComponentProps>,
    pub internal_connections: Vec<InternalConnection>,
    pub traces: Vec<PcbTrace>,
    pub vias: Vec<PcbVia>,
    pub plated_holes: Vec<PcbPlatedHole>,
    pub holes: Vec<PcbHole>,
    pub copper_pours: Vec<PcbCopperPour>,
    pub silkscreen_texts: Vec<SilkscreenText>,
    pub silkscreen_paths: Vec<SilkscreenPath>,
    pub courtyard_rects: Vec<CourtyardRect>,
    pub courtyard_outlines: Vec<CourtyardOutline>,
    pub issues: Vec<UpstreamIssue>,
    /// `<id>: <type>: <why>` for each element that could not be read.
    pub unreadable: Vec<String>,
}

pub(super) trait Kind: DeserializeOwned {
    const TYPE: &'static str;
    fn slot(model: &mut Model) -> &mut Vec<Self>;
}

type Reader = fn(&mut Model, &Value) -> Result<(), String>;

fn read<T: Kind>(model: &mut Model, element: &Value) -> Result<(), String> {
    let item = T::deserialize(element).map_err(|err| err.to_string())?;
    T::slot(model).push(item);
    Ok(())
}

const READERS: [(&str, Reader); 13] = [
    (PcbBoard::TYPE, read::<PcbBoard>),
    (PcbComponent::TYPE, read::<PcbComponent>),
    (ComponentProps::TYPE, read::<ComponentProps>),
    (InternalConnection::TYPE, read::<InternalConnection>),
    (PcbTrace::TYPE, read::<PcbTrace>),
    (PcbVia::TYPE, read::<PcbVia>),
    (PcbPlatedHole::TYPE, read::<PcbPlatedHole>),
    (PcbHole::TYPE, read::<PcbHole>),
    (PcbCopperPour::TYPE, read::<PcbCopperPour>),
    (SilkscreenText::TYPE, read::<SilkscreenText>),
    (SilkscreenPath::TYPE, read::<SilkscreenPath>),
    (CourtyardRect::TYPE, read::<CourtyardRect>),
    (CourtyardOutline::TYPE, read::<CourtyardOutline>),
];

impl Model {
    /// Reads every element of `circuit` once.
    ///
    /// Ports whose pin attributes failed to read are listed in `unreadable`
    /// too, under their `source_port_id`.
    pub fn build(circuit: &Circuit) -> Self {
        let mut model = Self::default();
        for element in &circuit.raw_elements {
            model.add(element);
        }
        let ports = circuit.ports().filter_map(|port| {
            let why = port.attributes.unreadable.as_ref()?;
            let id = port.port_id.as_deref().unwrap_or("source_port");
            Some(format!("{id}: source_port: pin attributes: {why}"))
        });
        model.unreadable.extend(ports);
        model
    }

    fn add(&mut self, element: &Value) {
        if let Some(issue) = UpstreamIssue::from_element(element) {
            self.issues.push(issue);
            return;
        }
        let kind = element.get("type").and_then(Value::as_str).unwrap_or("");
        let Some((_, reader)) = READERS.iter().find(|(name, _)| *name == kind) else {
            return;
        };
        if let Err(why) = reader(self, element) {
            let own = element.get(format!("{kind}_id")).and_then(Value::as_str);
            let id = own.unwrap_or(kind);
            self.unreadable.push(format!("{id}: {kind}: {why}"));
        }
    }
}
