import type { Content } from "./types";

type Landing = Pick<Content, "hero" | "demos" | "agents" | "closing" | "footer">;

export const enLanding: Landing = {
  hero: {
    title: "One project for your boards, firmware and enclosure.",
    sub: "When a rule fails, the fuse blows and nothing ships.",
    download: "Download",
    copyLabel: "Copy the install command",
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
    check: {
      title: "A blown fuse stops the export",
      text: "Exit code 1, and no Gerber, STEP or firmware file gets written.",
    },
    firmware: {
      title: "Firmware follows the board",
      text: "Move a pin in the TSX and the Rust constants follow.",
    },
    cost: { title: "Costs stay current", text: "Parts, prints and prices roll up into one BOM.", label: "Total" },
    loop: {
      title: "Caught in the design, not on the bench",
      short: {
        title: "Shorts never reach the fab",
        text: "Two nets that touch blow the fuse before anything is routed.",
      },
      route: { title: "Traces route themselves", text: "fusible route turns the ratsnest into copper." },
      sync: { title: "KiCad edits come back", text: "Move a part in KiCad, and fusible sync writes it back." },
    },
    board: {
      title: "A board, in 3D",
      text: "Tap Explore in 3D, then drag to turn it.",
      presets: { iso: "Iso", top: "Top", side: "Side" },
      auto: "Auto",
      explore: "Explore in 3D",
      done: "Done",
      hint: "Drag to orbit, pinch or scroll to zoom",
      loading: "Loading 3D board",
      failed: "3D preview unavailable",
      failedText: "This browser could not start WebGL.",
      status: "Fuse intact, 0 violations",
    },
  },
  agents: {
    title: "Agents work on your boards",
    text: "Fusible speaks MCP, so Bulle, Claude Code and Codex get the fuse verdict on every edit.",
  },
  closing: { title: "Check your next board before you order it." },
  footer: { license: "GPL-3.0 licensed", github: "GitHub", crates: "crates.io", studio: "Facile Studio" },
};
