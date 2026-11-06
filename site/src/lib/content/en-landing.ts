import type { Content } from "./types";
import { ARXIV, BULLE_SNIPPET, SOURCES, SUITE_LINKS } from "./shared";

type Landing = Pick<
  Content,
  | "hero"
  | "loop"
  | "pillars"
  | "rules"
  | "agents"
  | "board3d"
  | "checks"
  | "bodies"
  | "mobile"
  | "desktop"
  | "suite"
  | "footer"
>;

export const enLanding: Landing = {
  hero: {
    title: "One project for your boards, firmware and enclosure.",
    sub:
      "Fusible checks the whole design against electrical, physical and mechanical rules. " +
      "When one fails, the fuse blows and nothing gets exported, generated or flashed.",
    download: "Download",
    copyLabel: "Copy the install command",
    shotAlt: "Fusible studio showing a PCB layout with its checks",
  },
  loop: {
    title: "The fuse loop",
    steps: [
      { icon: "solar:pen-new-square-linear", title: "Design", text: "Write boards in TSX and parts in code." },
      { icon: "solar:shield-check-linear", title: "Check", text: "The fuse runs every rule on every board." },
      {
        icon: "solar:danger-triangle-linear",
        title: "Blown",
        text: "Each violation names its rule, its standard and a fix.",
      },
      {
        icon: "solar:box-minimalistic-linear",
        title: "Ship",
        text: "Gerbers, STEP, BOM and firmware pins, only when the fuse is intact.",
      },
    ],
  },
  pillars: {
    title: "One workspace, from board to box",
    items: [
      {
        icon: "solar:cpu-bolt-bold-duotone",
        title: "Boards",
        text: "tscircuit TSX, with live PCB, schematic and 3D views.",
      },
      {
        icon: "solar:bolt-circle-bold-duotone",
        title: "Physics rules",
        text: "IPC-2152 current, IPC-2221B clearance and fab capability.",
      },
      {
        icon: "solar:code-square-bold-duotone",
        title: "Firmware",
        text: "Typed Rust pins for Embassy and ESP-IDF, blocked when the board is wrong.",
      },
      {
        icon: "solar:box-bold-duotone",
        title: "Mechanical",
        text: "Parts written in TypeScript on OpenCascade, with real STEP and process-aware checks.",
      },
      {
        icon: "solar:bill-list-bold-duotone",
        title: "BOM",
        text: "Prices, picks, materials, print mass and cost.",
      },
      {
        icon: "solar:layers-linear",
        title: "Fab and KiCad",
        text: "Gerbers, drill files, STEP, GLB and a KiCad round-trip.",
      },
    ],
  },
  rules: {
    title: "Rules that name their source",
    intro: "Every violation carries the rule id, the standard it comes from and a link to the source.",
    items: [
      {
        id: "trace_current",
        summary: "A trace carries its declared current within the allowed temperature rise.",
        standard: { label: "IPC-2152", href: SOURCES.ipc2152 },
      },
      {
        id: "voltage_clearance",
        summary: "Copper of different nets keeps the spacing its voltage needs.",
        standard: { label: "IPC-2221B", href: SOURCES.ipc2221b },
      },
      {
        id: "fab_profile",
        summary: "Traces, drills, vias and board size fit the fab house's limits.",
        standard: { label: "JLCPCB 2/4-layer", href: SOURCES.jlcpcb },
      },
      { id: "trace_clearance", summary: "Copper of different nets keeps a minimum spacing." },
      { id: "shorted_nets", summary: "No trace ties differently named nets together, such as GND and 3V3." },
      { id: "unconnected_net", summary: "Every net connects at least two pads or ports." },
      { id: "pin_mux", summary: "Each pin supports the peripheral function you gave it on the chosen chip." },
      {
        id: "bypass_cap",
        summary: "Every IC power pin has a decoupling capacitor, within 3 mm when placement is known.",
        standard: { label: "layout guideline", href: SOURCES.bypass },
      },
      { id: "body_interference", summary: "A body does not overlap the board, a component or another body." },
      {
        id: "mech.wall_thickness",
        summary: "Walls meet the minimum for the print or mould process.",
        standard: { label: "DFAM Academy", href: SOURCES.dfam },
      },
      {
        id: "mech.standoff_height",
        summary: "Standoffs clear the tallest part under the board by at least 0.5 mm.",
        standard: { label: "DFAM Academy", href: SOURCES.dfam },
      },
      {
        id: "mech.draft",
        summary: "Injection-moulded parts declare a draft of at least 0.5 degrees.",
        standard: { label: "RTP Company", href: SOURCES.rtp },
      },
    ],
    disclaimer: "Design pre-checks, not certifications. Safety-rated products still need a lab.",
  },
  agents: {
    title: "Let agents work on your boards.",
    text:
      "Fusible speaks MCP. Bulle, Claude Code and Codex can check, edit and build, " +
      "and every edit returns the fuse verdict.",
    evidence: "Research on PCB agents finds that closed-loop, rule-checked agents beat one-shot generation.",
    sources: ARXIV,
    snippetLabel: "Bulle config, ~/.config/bulle/config.yml",
    snippet: BULLE_SNIPPET,
  },
  board3d: {
    title: "See the board as it will be built",
    text: "Live schematic, PCB and 3D views update on every save, with parts you can inspect pad by pad.",
    alt: "The Fusible studio showing a round PCB in 3D",
  },
  checks: {
    title: "Every violation cites its rule",
    text: "Each check names its standard and links its source. They are design pre-checks, not certifications.",
    alt: "The Fusible studio listing checks with their standards",
  },
  bodies: {
    title: "Enclosures and parts from code",
    text:
      "Write mechanical parts in TypeScript, pick a material and a process, " +
      "and get a real STEP file, mass and cost.",
    alt: "The Fusible studio showing a generated enclosure in 3D",
  },
  mobile: {
    title: "Your project in your pocket",
    text: "A read-only view of boards, checks and the parts list. Pair with a QR code on your local network.",
    alt: "The Fusible Android app showing a board summary",
    cta: { label: "Download for Android", href: "/download#android" },
  },
  desktop: {
    title: "A desktop app",
    text: "Projects, toolchain and updates in one window. Updates are verified and install in the background.",
    alt: "The Fusible desktop app listing projects",
    cta: { label: "Download the desktop app", href: "/download" },
  },
  suite: {
    title: "Part of Facile Studio",
    text: "Fusible works with the other Facile apps.",
    links: [
      { label: "Facile Studio", href: SUITE_LINKS.facile },
      { label: "Bulle, agents", href: SUITE_LINKS.bulle },
      { label: "Nuage, share fab files", href: SUITE_LINKS.nuage },
      { label: "Opus, tasks from blown fuses", href: SUITE_LINKS.opus },
    ],
  },
  footer: { license: "MIT licensed", github: "GitHub", crates: "crates.io", studio: "Facile Studio" },
};
