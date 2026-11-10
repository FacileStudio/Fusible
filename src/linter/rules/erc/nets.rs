//! What a net is to the electrical rules: a supply rail, a ground, and what resistors join.

use std::collections::BTreeMap;

use super::parts::PartKind;
use super::roles::{is_ground_name, is_rail_name, Role};
use super::Board;

/// Whether net `net` is a supply rail (`up`) or a ground (`!up`): by a pin's role, by a
/// declared `nets:` voltage (rails only), or by the net's name.
pub(super) fn is_rail(board: &Board<'_>, net: usize, up: bool) -> bool {
    let wanted: &[Role] = match up {
        true => &[Role::PowerIn, Role::PowerOut],
        false => &[Role::GroundIn, Role::GroundOut],
    };
    let name = board.on_net(net).next().map_or("", |p| p.net_name);
    let named = match up {
        true => is_rail_name(name),
        false => is_ground_name(name),
    };
    let declared = up
        && board
            .spec(net)
            .is_some_and(|s| s.voltage.is_some_and(|v| v > 0.0));
    named || declared || board.on_net(net).any(|p| wanted.contains(&p.role().0))
}

/// For each net, the nets a two-pin part of kind `kind` joins it to.
pub(super) fn joined_by(board: &Board<'_>, kind: PartKind) -> BTreeMap<usize, Vec<usize>> {
    let mut parts: BTreeMap<&str, Vec<Option<usize>>> = BTreeMap::new();
    for pin in board.pins.iter().filter(|p| p.kind == kind) {
        parts.entry(pin.part_id).or_default().push(pin.net);
    }
    let mut ends: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for nets in parts.values() {
        if let [Some(a), Some(b)] = nets.as_slice() {
            ends.entry(*a).or_default().push(*b);
            ends.entry(*b).or_default().push(*a);
        }
    }
    ends
}
