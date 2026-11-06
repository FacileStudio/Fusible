import type { DocsContent } from "./types";
import { BULLE_SNIPPET, CODEX_SNIPPET, MANIFEST_YAML, MCP_TOOLS } from "./shared";

export const enDocs: DocsContent = {
  title: "Fusible docs",
  intro: "Everything you need to run your first checked project.",
  toc: "On this page",
  sections: [
    {
      id: "install",
      title: "Install",
      blocks: [
        { kind: "code", code: "cargo install fusible\nfacile install fusible" },
        {
          kind: "p",
          text:
            "Linting Circuit JSON, Gerbers and SVG needs nothing else. " +
            "To compile .tsx boards or export STEP, install tscircuit and check the toolchain.",
        },
        { kind: "code", code: "bun add -g tscircuit\nfusible doctor\nfusible doctor --install" },
        {
          kind: "p",
          text:
            "The desktop app for Windows, macOS and Linux is attached to each GitHub release. " +
            "Installers are not code-signed with Apple or Microsoft certificates, so your OS will warn you. " +
          "In-app updates are verified against Fusible's own signing key.",
        },
      ],
    },
    {
      id: "quick-start",
      title: "Quick start",
      blocks: [
        { kind: "code", code: "fusible init --name sensor-node\nfusible check\nfusible studio" },
        { kind: "p", text: "init writes a fusible.yml. check lints every board. studio opens the live viewer." },
        {
          kind: "p",
          text:
            "Exit codes: 0 means the fuse is intact, 1 means it is blown, 2 means a tool error. " +
            "fusible check --json prints the verdict and every diagnostic as JSON.",
        },
      ],
    },
    {
      id: "project-file",
      title: "The project file",
      blocks: [
        { kind: "code", code: MANIFEST_YAML },
        {
          kind: "list",
          items: [
            "boards, firmware and bodies are named things. Names are unique across all three.",
            "nets declares the voltage and current of a net. The physics rules read it.",
            "fab picks a fab profile: jlcpcb-2l, jlcpcb-4l, or your own limits.",
            "clearance_class is B1, B2 or B4. The default is B2.",
            "copper sets the copper weight: outer_oz and inner_oz.",
          ],
        },
      ],
    },
    {
      id: "rules",
      title: "Rules",
      blocks: [
        {
          kind: "table",
          head: ["Rule", "Standard"],
          rows: [
            ["trace_current", "IPC-2152"],
            ["voltage_clearance", "IPC-2221B (classes B1, B2, B4)"],
            ["fab_profile", "JLCPCB 2-layer and 4-layer"],
            ["trace_clearance, shorted_nets, unconnected_net, pin_mux", "Topology and silicon checks"],
            ["bypass_cap", "Layout guideline, 3 mm"],
            ["body_interference", "Board and body geometry"],
            ["mech.wall_thickness, mech.standoff_height, mech.draft", "DFAM Academy, RTP Company"],
          ],
        },
        { kind: "p", text: "Design pre-checks, not certifications. Safety-rated products still need a lab." },
      ],
    },
    {
      id: "mechanical",
      title: "Mechanical parts",
      blocks: [
        { kind: "code", code: "fusible body new bracket --material petg --process fdm\nfusible body bracket" },
        {
          kind: "p",
          text:
            "body new writes mech/bracket.ts, mech/fusible.d.ts and a bodies entry in fusible.yml. " +
            "Building a code part writes STEP, STL and GLB. Processes are fdm, sla, mjf, injection and cnc.",
        },
        {
          kind: "p",
          text:
            "A part is a TypeScript function that receives the board and a Cad object. " +
            "Cad has box, cylinder, extrude, union, cut, intersect, move, rotate, shell, boss, " +
            "standoff, ventSlots, cutout and draft. Use the helpers to get rule coverage.",
        },
      ],
    },
    {
      id: "agents",
      title: "Agents",
      blocks: [
        { kind: "p", text: "fusible mcp serves these tools over stdio. Every write returns the diff and the fuse." },
        { kind: "table", head: ["Tool", "What it does"], rows: MCP_TOOLS },
        {
          kind: "code",
          code: "fusible mcp --print-config bulle\nfusible mcp --print-config claude\nfusible mcp --print-config codex",
        },
        { kind: "code", code: BULLE_SNIPPET },
        { kind: "code", code: "claude mcp add fusible -- fusible mcp" },
        { kind: "code", code: CODEX_SNIPPET },
      ],
    },
    {
      id: "phone",
      title: "Phone pairing",
      blocks: [
        { kind: "code", code: "fusible studio --share" },
        {
          kind: "p",
          text:
            "Press Pair phone in the studio and scan the QR code with the Android app. " +
            "Both devices must be on the same private network. The phone view is read-only.",
        },
      ],
    },
    {
      id: "suite",
      title: "Suite",
      blocks: [
        { kind: "code", code: "fusible check --opus\nfusible bom --opus" },
        {
          kind: "p",
          text: "A blown fuse or an unbought part becomes an Opus task. The Opus side never changes the verdict.",
        },
        { kind: "code", code: "fusible share nuage --space <name> --preset fab --link --expires 30d" },
        { kind: "p", text: "The export lands in a synced Nuage space. Nothing is copied while the fuse is blown." },
      ],
    },
  ],
};
