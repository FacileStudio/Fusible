//! The table label of a printed body: its description, material, process and weight.

use super::report::Line;

pub(super) fn print_label(line: &Line) -> String {
    let made = [line.material.clone(), line.process.clone()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let grams = line.grams.map(|g| format!("{g} g"));
    let detail = [Some(made).filter(|m| !m.is_empty()), grams]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ");
    match detail.is_empty() {
        true => line.description.clone(),
        false => format!("{} ({detail})", line.description),
    }
}
