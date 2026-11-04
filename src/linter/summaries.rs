//! One-sentence summaries of the registered rules, served at `/api/rules`.

/// The summary of the rule called `name`, or an empty string for an unknown name.
pub(super) fn summary_of(name: &str) -> &'static str {
    match name {
        "unconnected_net" => "A net with fewer than two endpoints is floating.",
        "malformed_element" => "A recognized element without its identifier cannot be verified.",
        "ghost_net" => "A port or pad refers to a net that no source net defines.",
        "dangling_reference" => "A trace or port points at an element that does not exist.",
        "shorted_nets" => "Two differently named nets must not be tied together by a trace.",
        "pin_mux" => "Pin assignments respect the alternate functions of the target MCU.",
        "decoupling_capacitor" => {
            "Each IC power pin needs a bypass capacitor on its rail, placed within 3 mm of the pin."
        }
        "trace_clearance" => {
            "Copper of distinct nets on one layer keeps 0.15 mm, or the fab minimum space when a fab is set."
        }
        "E_SCHEMATIC_SCALE_EXCESSIVE" => "Schematic symbols stay inside a readable sheet.",
        "trace_current" => {
            "A net that declares a current must not heat its narrowest trace past its allowed rise."
        }
        "voltage_clearance" => {
            "Copper at different declared voltages keeps the IPC-2221B Table 6-1 spacing."
        }
        "fab_profile" => {
            "Trace width, spacing, drills, vias and board size stay within the declared fab limits."
        }
        _ => "",
    }
}
