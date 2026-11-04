# Fusible v1.0: execution plan

This folder is the full plan for Fusible v1.0.0. Each track file is a self-contained task for one agent. An agent reads this file first, then its own track file, and nothing else is needed to start.

Research behind the plan: https://mycelium.facile.studio/artifacts/2026-11-04-fusible-v1-0-deep-research-report-c0bd47

## Goal

Ship Fusible v1.0.0 as a Facile Studio project. It is one open workspace that takes a hardware product from boards to firmware to mechanical parts to BOM to fab files. Every output stays behind the fuse. Humans, Bulle and any MCP agent drive it through the same checked loop.

## Files

| File | Who | When | Branch / worktree |
|---|---|---|---|
| `00-move.md` | integrator | first, alone | `main` |
| `A-studio.md` | agent A | after 00 | `feat/v1-studio` in `~/Code/Kono/Fusible-v1-studio` |
| `B-desktop.md` | agent B | after 00 | `feat/v1-desktop` in `~/Code/Kono/Fusible-v1-desktop` |
| `C-mobile.md` | agent C | after 00 | `feat/v1-mobile` in `~/Code/Kono/Fusible-v1-mobile` |
| `D-rules.md` | agent D | after 00 | `feat/v1-rules` in `~/Code/Kono/Fusible-v1-rules` |
| `E-mechanical.md` | agent E | after 00 | `feat/v1-mech` in `~/Code/Kono/Fusible-v1-mech` |
| `F-mcp-suite.md` | agent F | after 00 | `feat/v1-mcp` in `~/Code/Kono/Fusible-v1-mcp` |
| `G-site.md` | agent G | after 00 | `feat/v1-site` in `~/Code/Kono/Fusible-v1-site` |
| `I-fab.md` | agent I | after 00 | `feat/v1-fab` in `~/Code/Kono/Fusible-v1-fab` |
| `H-integrate-ship.md` | integrator | after A to G report done | `main` |
| `PROMPTS.md` | you | copy-paste prompts for each agent | |
| `assets/` | all | `logo.svg`, `favicon.svg`, `logo-mark.svg` (Solar `cpu-bolt-bold-duotone`) | |

Tracks A to G and I run in parallel. They share no files except the ones listed under "Shared files" below.

## Facts every agent must know

- **Repo:** after track 00 it is `https://github.com/FacileStudio/Fusible`. The local checkout stays at `/home/yann/Code/Kono/Fusible`.
- **Language and stack:**
  - Rust CLI crate `fusible`, version 0.20.9 today, 1.0.0 at release.
  - The studio is served by `tiny_http` in `src/dev/`.
  - The browser bundle is built from `studio/` with bun.
  - The desktop app is Tauri 2 in `desktop/`.
- **Tooling:**
  - `bun` for every JS/TS task (never npm, never npx: use `bunx`).
  - `mise` is the task runner.
  - `cargo` for Rust.
- **Quality gate:** `filet check` at the repo root. Limits in `filet.yml`:

| Limit | Value |
|---|---|
| Lines per file | 250 |
| Functions per file | 8 |
| Lines per function | 30 |
| Line length | 120 |
| Parameters | 5 |
| Returns | 3 |
| Nesting | 4 |
| Complexity | 10 |

  - filet reads `.rs` and `.ts` only.
  - **Never raise a limit. Never add a path to `ignore:` to hide your code.** Split files instead.
- **Other Rust gates:** `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`. CI runs all three.
- **Exit-code contract (never change):**

| Code | Meaning |
|---|---|
| 0 | fuse intact |
| 1 | fuse blown |
| 2 | tool error |

- **Comments:**
  - No inline comments.
  - No comments inside function bodies.
  - One doc comment (`///` in Rust, `/** */` in TS) above each public function.
  - Svelte components: no comments at all.
- **Dead code:** delete it. Never leave `#[allow(dead_code)]` for code you wrote.
- **Commits:**
  - Conventional Commits, `type(scope): summary`, imperative, lowercase, no trailing period, under 72 characters. Types: feat, fix, refactor, perf, docs, style, test, build, ci, chore.
  - No gitmoji (the hook adds it).
  - No `Co-Authored-By`.
  - No "Generated with" text.
  - Commit often, on your track branch only.
- **Never push.** Never merge. Never tag. Never edit files outside your track's "Paths you own" list. The integrator merges.

## Design rules (UI tracks A, B, C, G)

Load the `muse` skill before writing UI (`Skill: muse`). muse source: `/home/yann/Code/Facile/muse/src/lib`.

1. **NO BORDERS. ANYWHERE.** This is the user's hardest rule.
   - Forbidden: any `border`, `border-*` class or CSS `border:`, any `divide-*`, `<hr>`, muse `<Divider>`, muse `Button variant="outline"`, 1px lines drawn with `box-shadow` or gradients, and table cell borders.
   - Allowed: `border-0` / `border: 0` / `border: none` (to remove a border).
   - Separate things with background steps (`bg-fc-page`, `bg-fc-surface`, `bg-fc-component`), spacing, and type weight or size.
   - Focus indicator: muse's `focus-visible:outline-*` ring stays. It is an accessibility focus state, not a border. Don't remove it.
   - **Check:** `rg -n --glob '!**/node_modules/**' "border|divide-|<hr|Divider|variant=\"outline\"" <your paths>`. Every hit must be `border-0`, `border: 0` or `border: none`.
   - When a muse component draws a border, pass `class="border-0"`. muse merges classes with `tailwind-merge`, so yours wins.
2. **Pill buttons.**
   - Use muse `Button` / `IconButton`. They are already `rounded-fc-pill`.
   - Any custom clickable chip, tab or toggle also gets `rounded-full`.
   - Group related buttons inside a pill surface: `inline-flex gap-1 rounded-full bg-fc-surface p-1`.
3. **Icons: Solar only.**
   - Use muse `<Icon icon="solar:...">`. Chrome icons are `solar:*-linear`. Feature and brand marks are `solar:*-bold-duotone`.
   - Remove every `pixelarticons:` use and the `@iconify-json/pixelarticons` dependency.
   - Use names from muse's `icons` map when it has one (`import { icons } from '@facile/muse'`).
   - For other Solar names, register their bodies with `registerIcons({...})`, reading them from `@iconify-json/solar/icons.json` (add `@iconify-json/solar` as a devDependency). Pastille does this in `apps/web/src/lib/icons.ts`. Copy that pattern.
4. **Logo:** `solar:cpu-bolt-bold-duotone`. Files are in `docs/v1/assets/`. The integrator copies them to `assets/logo.svg`, `assets/favicon.svg` and `assets/logo-mark.svg` in track 00. Use those paths.
5. **Tokens:** use only `fc-*` colors (`bg-fc-surface`, `text-fc-fg-muted`, ...). No hex, no `rgb()`, no Tailwind stock palette (muse deletes it). Exception: the tscircuit viewers' internal canvas colors, which can't take classes.
6. **Theme:**
   - Dark by default.
   - A `.dark` / `.light` class on `<html>`, set by an inline script in `<head>` before the bundle loads, inside `try/catch`, so blocked storage never breaks the page.
   - The storage key is `fusible-theme`.
7. **Svelte 5 runes only** (`$state`, `$derived`, `$effect`, `$props`). No `svelte/store`. No `export let`.
8. **Motion:** GSAP or CSS transitions. Always guard with muse `prefersReducedMotion()`.
9. **Copy:**
   - Plain English, sentence case, no em dashes, no emoji.
   - Site and mobile copy also ship in French (see track G for the i18n setup).
   - Physics claims always name the standard and say "design pre-check, not certification".
10. **Accessibility:**
    - Real `<button>` for actions and `<a>` for links.
    - `aria-label` on every icon-only button.
    - Text contrast stays at muse defaults.

## Shared contracts (do not change without telling the integrator)

### Studio HTTP API

These existing routes stay byte-compatible. See `src/dev/server.rs` `handle()`.

| Route | Returns |
|---|---|
| `GET /api/circuit?board=<name>` | `DevPayload` |
| `GET /api/project` | |
| `GET /api/bom` | |
| `GET /api/runs` | |
| `POST /api/run` | |
| `GET /api/firmware?name=<fw>` | |
| `GET /api/kicad` | |
| `GET /api/mechanical`, `GET /api/mechanical.glb` | |
| `GET /api/assembly`, `GET /api/assembly/model` | |
| `GET /api/bodies/<name>.glb?rev=<r>` | |
| `GET/POST /api/git`, `/api/git/save`, `/api/git/update`, `/api/git/resolve` | |
| `GET /events` | SSE of `DevPayload` JSON |

`DevPayload` (`src/dev/pipeline.rs`) is `{board_name, board?, fuse_blown, diagnostics:[DevDiagnostic], circuit_json, revision}`.

New routes (owner track in brackets):

| Route | Returns | Owner |
|---|---|---|
| `GET /api/rules` | `[{"id":"trace_current","standard":"IPC-2152","source_url":"https://...","severity":"error","enabled":true,"summary":"..."}]` | D |
| `GET /api/bodies` | `[{"name":"bracket","kind":"code","source":"mech/bracket.ts","material":"petg","process":"fdm","mass_g":12.4,"cost":0.31,"bbox":[40,20,8],"glb":"/api/bodies/bracket.glb?rev=<hash>"}]`. `kind` is `"authored"`, `"generated"` or `"code"`. | E |
| `POST /api/pair/start` | `{"uri":"fusible://pair?h=192.168.1.20:3020&t=<token>","expires_at":"<rfc3339>"}`. Loopback-only, same-origin. | C |
| `POST /api/pair/claim` | body `{"token":"..."}`, returns `{"device_id":"<uuid>","device_token":"<64 hex>"}`. LAN allowed. | C |
| `GET /api/pair/devices` | `[{"device_id","name","paired_at","last_seen"}]`. Loopback-only. | C |
| `DELETE /api/pair/devices/<id>` | loopback-only | C |
| `GET /api/mobile/summary` | `{"project":"name","boards":[{"name","fuse_blown","errors","warnings"}],"bom_total":{"amount":12.3,"currency":"EUR"},"bodies":[{"name","kind"}],"last_run":{...}\|null}`. Device-token auth: `Authorization: Bearer <device_token>`. | C |

### `DevDiagnostic` and `fusible check --json`

Track D owns this and adds optional fields. Every consumer must treat them as optional:

```json
{"rule":"trace_current","message":"...","help":"...","severity":"error",
 "standard":"IPC-2152","source_url":"https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html",
 "elements":["pcb_trace_12"],"fix":"widen VBUS to 0.8 mm"}
```

`fusible check --json` prints `{"fuse":"intact"|"blown","boards":[{"name":"main","diagnostics":[...]}]}` to stdout. It keeps the exit codes.

### Manifest additions (`fusible.yml`)

```yaml
nets:                       # track D
  VBUS: { voltage: 5, current: 2, max_rise: 10 }
  MAINS_L: { voltage: 230 }
fab: jlcpcb-2l              # track D: jlcpcb-2l | jlcpcb-4l | { min_trace, min_space, min_drill, min_via_pad, min_annular, min_via_to_via, drill_step, board_min, board_max }
clearance_class: B2         # track D: B1 | B2 | B4 (default B2). B3, A5-A7 and IEC 62368-1 creepage are NOT in v1 (unverified tables)
copper: { outer_oz: 1, inner_oz: 0.5 }   # track D, defaults shown
bodies:
  bracket:
    source: mech/bracket.ts # track E: a .ts source = code-first part
    material: petg          # track E
    process: fdm            # track E: fdm | sla | mjf | injection | cnc
```

Note: `assembly.parts` already exists (placements). Mechanical parts are **bodies** with a `.ts` `source`. There is no top-level `parts:` key.

### Shared files (merge conflicts expected, keep your edits minimal and additive)

- `src/cli.rs` and `src/cli/args.rs`: D (`--json` on `check`), E (`New` variant of `BodyAction`, i.e. `fusible body new <name>`), F (`mcp` and `share` subcommands, `--opus` on `check` and `bom`), C (`--share` flag on `studio`). Add your variant or flag only. Don't reformat the file.
- `src/dev/server.rs` `handle()`: C, D and E each add match arms. Add arms only.
- `Cargo.toml`: add dependencies only if your track file allows it.
- `src/config/types.rs` / `src/config/mod.rs`: D adds `nets`, `fab`, `clearance_class`, `safety`. E adds `material`, `process` to `BodyConfig`. Add fields only.

## Report format (every agent, at the end)

Write `docs/v1/reports/<track>.md` in your worktree and commit it. It contains:

```markdown
# Track <X> report
Branch: feat/v1-<x>   Last commit: <sha>
## Done
- ...
## Gates (paste the last lines of each)
- cargo fmt --check: ...
- cargo clippy: ...
- cargo test: ...
- filet check: ...
- bun builds/tests: ...
- no-border grep: <command> -> <output or "no hits">
## Not done / blocked
- ...
## Notes for the integrator (conflicts, contract questions)
- ...
```

If you get stuck on a decision these files don't answer, don't guess. Write it under "Not done / blocked" and stop that sub-task.
