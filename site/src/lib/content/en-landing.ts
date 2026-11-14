import type { Content } from "./types";

type Landing = Pick<Content, "hero" | "demos" | "agents" | "suite" | "dev" | "closing" | "footer">;

export const enLanding: Landing = {
  hero: {
    title: "Check your board before you order it.",
    sub: "Describe your circuit board in code. Fusible checks every rule and blocks the export while an error remains.",
    note: "Free and open source (GPL-3.0). No Facile account needed.",
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
      title: "Pastille, in 3D",
      text: "The real Pastille board, designed with Fusible. Tap Explore in 3D, then drag to turn it.",
      link: "See Pastille",
      pcb: "3D PCB",
      enclosure: "Enclosure",
      presets: { iso: "iso", top: "top", side: "side" },
      auto: "Auto",
      explore: "Explore in 3D",
      done: "Done",
      hint: "Drag to orbit · Pinch or scroll to zoom",
      loading: "Loading 3D geometry",
      failed: "3D preview unavailable",
      failedText: "Could not load the 3D model. Reload the page to try again.",
      status: "Fuse intact · 0 DRC violations",
    },
  },
  agents: {
    title: "Agents work on your boards",
    text: "Fusible speaks MCP, so Bulle, Claude Code and Codex get the fuse verdict on every edit.",
  },
  suite: {
    description: "Fusible plugs into three Facile apps.",
    nuage: "Send the fab files to your team in one command.",
    bulle: "An assistant that edits and checks your board.",
    pastille: "A real board built with Fusible.",
    suite: "Fusible is part of the Facile suite.",
    suiteLink: "See every app",
  },
  dev: {
    description: "Install the desktop app or use cargo, and plug in Bulle or your agents over MCP.",
    mcp: "Bulle, Claude Code and Codex read the fuse verdict and edit the board through 15 tools.",
    actionTitle: "GitHub Action",
    action: "Blow the fuse in CI on every pull request.",
    source: "GPL-3.0. Read the code, file an issue, send a fix.",
  },
  closing: { title: "Your next board, checked before the fab." },
  footer: {
    tagline: "Check your circuit board before you order it.",
    license: "GPL-3.0 licensed",
    github: "GitHub",
    crates: "crates.io",
    studio: "Facile Studio",
  },
};
