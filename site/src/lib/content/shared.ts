import type { Link } from "./types";

export const BULLE_SNIPPET = `sources:
  mcp:
    fusible:
      command: fusible
      args: [mcp]`;

export const CODEX_SNIPPET = `[mcp_servers.fusible]
command = "fusible"
args = ["mcp"]`;

export const ARXIV: Link[] = [
  { label: "arxiv.org/html/2607.05915", href: "https://arxiv.org/html/2607.05915" },
  { label: "arxiv.org/html/2602.00510v1", href: "https://arxiv.org/html/2602.00510v1" },
];

export const SUITE_LINKS = {
  facile: "https://facile.studio",
  bulle: "https://bulle.facile.studio",
  nuage: "https://nuage.facile.studio",
  opus: "https://opus.facile.studio",
};

export const SOURCES = {
  ipc2152: "https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html",
  ipc2221b: "https://www.atlaspcb.com/tools/conductor-spacing-calculator",
  jlcpcb: "https://jlcpcb.com/capabilities/pcb-capabilities",
  bypass: "https://www.protoexpress.com/blog/decoupling-capacitor-placement-guidelines-pcb-design/",
  dfam: "https://www.dfamacademy.com/guides/enclosure-design-3d-printing",
  rtp: "https://www.rtpcompany.com/wp-content/uploads/Part-Design-Guidelines-Brochure.pdf",
};

export const RULES_YAML = `rules:
  decoupling_capacitor: warning   # off | info | warning | error

waivers:
  - rule: decoupling_capacitor
    match: U3.VDD
    reason: bulk capacitor C12 is shared, checked on rev A
    until: 2027-03-01`;

export const MANIFEST_YAML = `name: rover

boards:
  main: { entry: boards/main/index.tsx, mcu: esp32-s3 }

firmware:
  main-fw: { board: main, path: firmware/main, pins_rs: firmware/main/src/pins.rs }

bodies:
  bracket:
    source: mech/bracket.ts
    material: petg
    process: fdm

nets:
  VBUS: { voltage: 5, current: 2, max_rise: 10 }
  MAINS_L: { voltage: 230 }

fab: jlcpcb-2l
clearance_class: B2`;

export const MCP_TOOLS: string[][] = [
  ["project_info", "Boards, firmware, bodies and git state."],
  ["check", "Runs the rules and returns the version 2 report: fuse, boards, links, assembly, waived."],
  ["explain_rule", "Category, summary, level, standard and source for one rule."],
  ["list_components", "Components and ports of a board."],
  ["list_nets", "Nets of a board and their declared limits."],
  ["bom", "Bill of materials rows and totals."],
  ["firmware_pins", "Generated pins.rs, only when the fuse is intact."],
  ["edit_board", "Patch or replace a board file. Dry run by default."],
  ["apply_fix", "Apply a set_prop fix from a diagnostic to the board TSX. Dry run by default."],
  ["waive", "Append a waiver with a required reason to fusible.yml. Dry run by default."],
  ["body_new", "Scaffold a code part."],
  ["edit_body", "Patch or replace a code part."],
  ["build_body", "Build a code part to STEP, STL and GLB."],
  ["export", "Fab, CAD or docs files. Refused while the fuse is blown."],
  ["render", "SVG of the PCB or the schematic."],
];
