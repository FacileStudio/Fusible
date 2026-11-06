//! LAN phone pairing: one-time tokens, paired device store, and the read-only remote gate.

mod claim;
mod clock;
mod dirs;
mod lan;
mod remote;
mod routes;
mod store;
mod summary;
mod token;

#[cfg(test)]
mod claim_limit_tests;
#[cfg(test)]
mod claim_tests;
#[cfg(test)]
mod remote_tests;
#[cfg(test)]
mod summary_tests;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use store::Devices;

use super::state::State;

pub(in crate::dev) use lan::lan_ip;
pub(in crate::dev) use remote::{authorize_remote, is_lan_host, is_remote};
pub(in crate::dev) use routes::{owns, serve};

/// A pairing token waiting to be claimed: its hash and the moment it stops working.
pub(super) struct Pending {
    pub(super) hash: String,
    pub(super) expires: Instant,
}

/// Everything pairing needs while the studio shares on the network.
pub(super) struct PairState {
    pub(in crate::dev::pair) pending: Mutex<Option<Pending>>,
    pub(in crate::dev::pair) devices: Mutex<Devices>,
    pub(in crate::dev::pair) failures: Mutex<HashMap<IpAddr, Vec<Instant>>>,
}

/// Turns on phone pairing for `state` and tells the user where the studio is shared.
pub(in crate::dev) fn enable_sharing(state: &mut State, ip: Option<Ipv4Addr>, port: u16) {
    let Some(ip) = ip else {
        return;
    };
    state.pair = Some(Arc::new(PairState::open()));
    state.lan_host = Some(format!("{ip}:{port}"));
    println!("sharing on http://{ip}:{port} for paired phones (read-only)");
    println!("pair a phone: open the studio and press \"Pair phone\"");
}

impl PairState {
    /// Opens the store at `path`; without one, devices live in memory only.
    pub(super) fn new(path: Option<PathBuf>) -> Self {
        Self {
            pending: Mutex::new(None),
            devices: Mutex::new(Devices::load(path)),
            failures: Mutex::new(HashMap::new()),
        }
    }

    /// Opens the store in the user's state directory.
    pub(super) fn open() -> Self {
        let path = dirs::state_dir().map(|dir| dir.join("devices.json"));
        if path.is_none() {
            eprintln!("⚠ no state directory: paired phones are forgotten when the studio stops");
        }
        Self::new(path)
    }
}
