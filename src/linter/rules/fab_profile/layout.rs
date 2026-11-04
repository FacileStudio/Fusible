//! Via-to-via spacing and board size limits of a fab profile.

use super::{violation, EPSILON};
use crate::config::FabProfile;
use crate::linter::rules::physics::mm;
use crate::linter::Diagnostic;
use crate::parser::geometry::{Board, Geometry, Via};

fn via_gap(a: &Via, b: &Via) -> f64 {
    (a.x - b.x).hypot(a.y - b.y) - (a.outer + b.outer) / 2.0
}

pub(super) fn close_vias(profile: &FabProfile, geometry: &Geometry) -> Vec<Diagnostic> {
    let mut vias: Vec<&Via> = geometry.vias.iter().collect();
    vias.sort_by(|a, b| a.x.total_cmp(&b.x));
    let widest = vias.iter().map(|v| v.outer).fold(0.0, f64::max);
    let reach = widest + profile.min_via_to_via;
    let mut found = Vec::new();
    for (i, a) in vias.iter().enumerate() {
        for b in vias[i + 1..].iter().take_while(|b| b.x - a.x <= reach) {
            let gap = via_gap(a, b);
            if gap < profile.min_via_to_via - EPSILON {
                let message = format!(
                    "via to via {} mm is below {} minimum {} mm",
                    mm(gap.max(0.0)),
                    profile.name,
                    mm(profile.min_via_to_via)
                );
                found.push(violation(
                    profile,
                    message,
                    vec![a.id.clone(), b.id.clone()],
                ));
            }
        }
    }
    found
}

fn fits(board: &Board, profile: &FabProfile) -> bool {
    let within = |w: f64, h: f64| {
        (profile.board_min[0]..=profile.board_max[0]).contains(&w)
            && (profile.board_min[1]..=profile.board_max[1]).contains(&h)
    };
    within(board.width, board.height) || within(board.height, board.width)
}

pub(super) fn board_size(profile: &FabProfile, geometry: &Geometry) -> Option<Diagnostic> {
    let board = geometry.board.as_ref().filter(|b| !fits(b, profile))?;
    let [min_w, min_h] = profile.board_min;
    let [max_w, max_h] = profile.board_max;
    let message = format!(
        "board {} x {} mm is outside {} limits {}x{} to {}x{} mm",
        mm(board.width),
        mm(board.height),
        profile.name,
        mm(min_w),
        mm(min_h),
        mm(max_w),
        mm(max_h)
    );
    Some(violation(profile, message, Vec::new()))
}
