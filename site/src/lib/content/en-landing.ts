import type { Content } from "./types";
import { ARXIV, BULLE_SNIPPET, SOURCES, SUITE_LINKS } from "./shared";

type Landing = Pick<
  Content,
  | "hero"
  | "loop"
  | "demos"
  | "rules"
  | "agents"
  | "board3d"
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
  demos: {
    hero: {
      label: "The Fusible studio blocks an export until a too narrow trace is fixed",
      intact: "Fuse intact",
      blown: "Fuse blown",
      check: "Check all",
      export: "Export fab",
      checks: "Checks",
      violation: "VBUS carries 2 A. A 0.2 mm trace overheats, it needs 0.8 mm.",
      refused: "Export blocked: the fuse is blown.",
      views: ["Schematic", "PCB", "3D", "Bodies", "Parts"],
    },
    agent: {
      prompt: "Add USB power to the board.",
      placeholder: "Message the agent",
      answer: "Done. The first VBUS trace was too narrow, so I widened it to 0.8 mm. The fuse is intact.",
      blown: "Fuse blown",
      intact: "Fuse intact",
    },
    title: "Nothing ships until the fuse holds",
    intro: "The same verdict in the terminal, in your firmware and in your costs.",
    check: {
      title: "A blown fuse stops the export",
      text: "Exit code 1, and no Gerber, STEP or firmware file gets written.",
    },
    firmware: {
      title: "Firmware follows the board",
      text: "Move a pin in the TSX and the typed Rust constants follow on the next check.",
    },
    cost: { title: "Costs stay up to date", text: "Parts, prints and prices roll up into one BOM.", label: "Total" },
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
      {
        id: "decoupling_capacitor",
        summary: "Every IC power pin has a decoupling capacitor, within 3 mm when placement is known.",
        standard: { label: "layout guideline", href: SOURCES.bypass },
      },
      {
        id: "mech.wall_thickness",
        summary: "Walls meet the minimum for the print or mould process.",
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
