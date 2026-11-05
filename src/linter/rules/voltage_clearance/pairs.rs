//! Finds the worst-spaced copper of each pair of nets with known voltages.

use std::collections::BTreeMap;

use crate::config::{ClearanceClass, DesignRules};
use crate::linter::rules::copper::{neighbours, Copper};
use crate::linter::rules::ipc2221b::spacing;
use crate::linter::rules::physics::is_external;

const GROUNDS: &[&str] = &["GND", "AGND", "DGND", "PGND", "VSS"];
const CLASSES: [ClearanceClass; 3] = [ClearanceClass::B1, ClearanceClass::B2, ClearanceClass::B4];

pub(super) struct Worst {
    pub gap: f64,
    pub layer: String,
    pub required: f64,
    pub class: ClearanceClass,
    pub high: (String, f64),
    pub low: (String, f64),
    pub elements: Vec<String>,
}

/// The net's declared voltage, or 0 V for a ground name. `None` for any other net.
pub(super) fn voltage(design: &DesignRules, net: &str) -> Option<f64> {
    let declared = design.nets.get(net).and_then(|spec| spec.voltage);
    let ground = GROUNDS.iter().any(|g| g.eq_ignore_ascii_case(net));
    declared.or_else(|| ground.then_some(0.0))
}

fn class_for(design: &DesignRules, layer: &str) -> ClearanceClass {
    match is_external(layer) {
        true => design.clearance_class,
        false => ClearanceClass::B1,
    }
}

fn candidate(design: &DesignRules, a: &Copper, b: &Copper) -> Option<Worst> {
    let (va, vb) = (voltage(design, &a.net)?, voltage(design, &b.net)?);
    let class = class_for(design, &a.layer);
    let required = spacing(class, (va - vb).abs());
    let gap = a.shape.gap(&b.shape);
    if va == vb || gap >= required {
        return None;
    }
    let ((high, hi), (low, lo)) = match va >= vb {
        true => ((a, va), (b, vb)),
        false => ((b, vb), (a, va)),
    };
    Some(Worst {
        gap,
        layer: a.layer.clone(),
        required,
        class,
        high: (high.net.clone(), hi),
        low: (low.net.clone(), lo),
        elements: vec![high.id.clone(), low.id.clone()],
    })
}

/// One entry per net pair: the violating spot with the smallest gap.
pub(super) fn worst_pairs(design: &DesignRules, items: &[Copper]) -> Vec<Worst> {
    let top = items
        .iter()
        .filter_map(|i| voltage(design, &i.net))
        .fold(0.0, f64::max);
    let margin = CLASSES.iter().map(|c| spacing(*c, top)).fold(0.0, f64::max);
    let mut worst: BTreeMap<(String, String), Worst> = BTreeMap::new();
    for (i, j) in neighbours(items, margin) {
        let Some(found) = candidate(design, &items[i], &items[j]) else {
            continue;
        };
        let key = (found.high.0.clone(), found.low.0.clone());
        if worst.get(&key).is_none_or(|w| found.gap < w.gap) {
            worst.insert(key, found);
        }
    }
    worst.into_values().collect()
}
