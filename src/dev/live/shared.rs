//! The state every request handler shares: the board slots, the event hub, and the runs.

use std::sync::{Arc, Mutex};

use super::hub::ClientHub;
use super::state::Slot;
use crate::commands::target::BoardTarget;
use crate::dev::kicad::status::KicadStates;
use crate::dev::pair::PairState;
use crate::dev::run::runs::Runs;

#[derive(Clone)]
pub(in crate::dev) struct State {
    pub hub: ClientHub,
    pub bundle: Arc<Mutex<Option<String>>>,
    pub slots: Arc<Vec<Slot>>,
    pub runs: Runs,
    pub desktop: bool,
    pub kicad: KicadStates,
    pub pair: Option<Arc<PairState>>,
    pub lan_host: Option<String>,
}

impl State {
    pub(in crate::dev) fn new(boards: &[BoardTarget]) -> Self {
        Self {
            hub: ClientHub::default(),
            bundle: Arc::default(),
            slots: Arc::new(boards.iter().map(Slot::new).collect()),
            runs: Runs::default(),
            desktop: false,
            kicad: KicadStates::default(),
            pair: None,
            lan_host: None,
        }
    }

    /// Finds the slot for `board`; without a name, the first one.
    pub(in crate::dev) fn slot(&self, board: Option<&str>) -> Option<&Slot> {
        match board {
            Some(name) => self.slots.iter().find(|s| s.name.as_deref() == Some(name)),
            None => self.slots.first(),
        }
    }
}
