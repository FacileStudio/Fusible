# Prompts for the track agents

Run track 00 first (it creates the worktrees). Then start one agent per block below, in parallel. Each prompt is self-contained.

Recommended model: Sonnet is fine for A to G. Track 00 and H touch GitHub, Dokploy, crates.io and two live sites, so run them with the strongest model you have, and stay at the keyboard.

---

## Track A: studio

```
You are working on Fusible, a Rust hardware linter with a browser studio.
Worktree: /home/yann/Code/Kono/Fusible-v1-studio (branch feat/v1-studio). Work only there.
Read, in this order and fully: docs/v1/README.md, then docs/v1/A-studio.md. Follow A-studio.md step by step.
Load the muse skill before writing any Svelte. Never use borders (see README "Design rules").
Commit often with Conventional Commits, no co-author line. Never push, merge or tag.
Run every gate in the "Gates" section. When done, write docs/v1/reports/A.md in the format from README.md and commit it.
If something in the plan is ambiguous, don't guess: note it under "Not done / blocked" and move on.
```

## Track B: desktop

```
You are working on Fusible's Tauri 2 desktop app.
Worktree: /home/yann/Code/Kono/Fusible-v1-desktop (branch feat/v1-desktop). Work only there.
Read fully: docs/v1/README.md, then docs/v1/B-desktop.md. Follow B-desktop.md step by step.
Load the muse skill before writing Svelte. Never use borders.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/B.md as described in README.md and commit it.
Don't guess on ambiguity: report it under "Not done / blocked".
```

## Track C: mobile + pairing

```
You are adding LAN phone pairing to Fusible's Rust dev server and building a read-only Capacitor 8 Android app.
Worktree: /home/yann/Code/Kono/Fusible-v1-mobile (branch feat/v1-mobile). Work only there.
Read fully: docs/v1/README.md, then docs/v1/C-mobile.md. Follow it exactly; the security semantics in section 1.4 are not optional.
Load the muse skill before writing Svelte. Never use borders.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/C.md (include the scanner plugin choice and whether gradle ran) and commit it.
Don't guess on ambiguity: report it.
```

## Track D: rules

```
You are adding physics and fabrication rules to Fusible's Rust linter.
Worktree: /home/yann/Code/Kono/Fusible-v1-rules (branch feat/v1-rules). Work only there.
Read fully: docs/v1/README.md, then docs/v1/D-rules.md. Use the constants and tables exactly as written there; do not substitute values from memory.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/D.md and commit it.
Don't guess on ambiguity: report it.
```

## Track E: mechanical

```
You are adding a code-first mechanical workspace to Fusible (TypeScript parts on occt-wasm, materials, mechanical rules).
Worktree: /home/yann/Code/Kono/Fusible-v1-mech (branch feat/v1-mech). Work only there.
Read fully: docs/v1/README.md, then docs/v1/E-mechanical.md. Read occt-wasm 5.7.1's type definitions before writing TS, as the file says; only use functions that exist.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/E.md and commit it.
Don't guess on ambiguity: report it.
```

## Track F: MCP + suite

```
You are adding an MCP server (fusible mcp) and Opus/Nuage CLI integrations to Fusible.
Worktree: /home/yann/Code/Kono/Fusible-v1-mcp (branch feat/v1-mcp). Work only there.
Read fully: docs/v1/README.md, then docs/v1/F-mcp-suite.md.
NEVER run the real opus or nuage commands: they act on the user's real accounts. Tests use the fake binaries described in the file.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/F.md (list the stubbed tools) and commit it.
Don't guess on ambiguity: report it.
```

## Track G: site

```
You are building fusible.facile.studio, a SvelteKit static site with muse, in English and French.
Worktree: /home/yann/Code/Kono/Fusible-v1-site (branch feat/v1-site). Work only there.
Read fully: docs/v1/README.md, then docs/v1/G-site.md. Read the Pastille web app and the facile.studio vitrine layouts it points to before designing.
Load the muse skill. Never use borders. Every claim on the page must match the plan files; don't invent features or flags.
Commit often (Conventional Commits, no co-author). Never push, merge, tag or deploy.
Run every gate. Write docs/v1/reports/G.md (include Lighthouse scores) and commit it.
Don't guess on ambiguity: report it.
```

## Track 00 and H (integrator)

```
You are the integrator for Fusible v1.0.0. Read docs/v1/README.md fully.
Phase 1: execute docs/v1/00-move.md exactly, then stop and report.
Phase 2 (only after agents A-G have committed their reports): execute docs/v1/H-integrate-ship.md exactly.
Ask the user before any step the file marks as needing consent. Report only verified results.
```

## Track I: fab correctness

```
You are fixing fabrication bugs in Fusible (gerber copper pours, silent routing failure, export defaults) and adding a fuse-gated `fusible release` command.
Worktree: /home/yann/Code/Kono/Fusible-v1-fab (branch feat/v1-fab). Work only there.
Read fully: docs/v1/README.md, then docs/v1/I-fab.md. Never modify Pastille's files; only read them or export to a temp dir.
Commit often (Conventional Commits, no co-author). Never push, merge or tag.
Run every gate. Write docs/v1/reports/I.md and commit it.
Don't guess on ambiguity: report it.
```
