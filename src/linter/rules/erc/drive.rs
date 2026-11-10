//! Which nets receive supply: a driver on the net, an external source declared in `nets:`,
//! or a series part (resistor, inductor, diode, switch, fuse) from a net that has it.

use std::collections::{BTreeMap, BTreeSet};

use super::{Board, Pin};
use crate::config::NetSource;

/// The two nets of every two-pin part that DC supply passes through.
fn bridges(board: &Board<'_>) -> Vec<(usize, usize)> {
    let mut parts: BTreeMap<&str, Vec<Option<usize>>> = BTreeMap::new();
    for pin in board.pins.iter().filter(|p| p.kind.passes_power()) {
        parts.entry(pin.part_id).or_default().push(pin.net);
    }
    parts
        .values()
        .filter_map(|nets| match nets.as_slice() {
            [Some(a), Some(b)] if a != b => Some((*a, *b)),
            _ => None,
        })
        .collect()
}

/// Every net that receives supply, given which pins drive a rail.
pub(super) fn supplied(board: &Board<'_>, drives: impl Fn(&Pin<'_>) -> bool) -> BTreeSet<usize> {
    let mut nets: BTreeSet<usize> = board
        .pins
        .iter()
        .filter(|pin| drives(pin))
        .filter_map(|pin| pin.net)
        .collect();
    let external = board.nets().map(|(net, _)| net).filter(|&net| {
        board
            .spec(net)
            .is_some_and(|s| s.source == Some(NetSource::External))
    });
    nets.extend(external.collect::<Vec<_>>());
    let bridges = bridges(board);
    let mut grew = true;
    while grew {
        grew = false;
        for &(a, b) in &bridges {
            if nets.contains(&a) != nets.contains(&b) {
                nets.extend([a, b]);
                grew = true;
            }
        }
    }
    nets
}
