//! IPC-2221B Table 6-1 conductor spacing, loaded from `data/ipc2221b.yml`.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::config::ClearanceClass;

const TABLE: &str = include_str!("data/ipc2221b.yml");

#[derive(Debug, Deserialize)]
struct Row {
    max_v: f64,
    b1: f64,
    b2: f64,
    b4: f64,
}

#[derive(Debug, Deserialize)]
struct Slope {
    base: f64,
    per_volt: f64,
}

#[derive(Debug, Deserialize)]
struct Over500 {
    b1: Slope,
    b2: Slope,
    b4: Slope,
}

#[derive(Debug, Deserialize)]
struct Table {
    rows: Vec<Row>,
    over_500: Over500,
}

fn table() -> &'static Table {
    static LOADED: OnceLock<Table> = OnceLock::new();
    LOADED.get_or_init(|| serde_yaml::from_str(TABLE).expect("data/ipc2221b.yml is valid"))
}

/// Minimum conductor spacing in millimetres for `volts` (DC or AC peak) between two conductors.
pub(super) fn spacing(class: ClearanceClass, volts: f64) -> f64 {
    let table = table();
    let pick = |b1: f64, b2: f64, b4: f64| match class {
        ClearanceClass::B1 => b1,
        ClearanceClass::B2 => b2,
        ClearanceClass::B4 => b4,
    };
    if let Some(row) = table.rows.iter().find(|row| volts <= row.max_v) {
        return pick(row.b1, row.b2, row.b4);
    }
    let over = &table.over_500;
    let slope = pick_slope(class, over);
    slope.base + slope.per_volt * (volts - 500.0)
}

fn pick_slope(class: ClearanceClass, over: &Over500) -> &Slope {
    match class {
        ClearanceClass::B1 => &over.b1,
        ClearanceClass::B2 => &over.b2,
        ClearanceClass::B4 => &over.b4,
    }
}
