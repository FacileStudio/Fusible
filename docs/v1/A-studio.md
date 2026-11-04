# Track A: studio on Svelte 5 + muse

Read `docs/v1/README.md` first, including the design rules and contracts. Load the `muse` skill.

Worktree: `/home/yann/Code/Kono/Fusible-v1-studio`, branch `feat/v1-studio`.

## Goal

Replace the vanilla `src/assets/viewer.html` (2,276 lines) and its scripts with a Svelte 5 + muse app built from `studio/`. The Rust binary still embeds the result. The HTTP API doesn't change.

## Paths you own

- `studio/` (all of it)
- `src/assets/viewer.html`, `src/assets/theme.css`, `src/assets/undefined-medium.otf`
- `src/assets/{project,bom,firmware,desktop,drawer,git,kicad}.js`
- `src/assets/studio.bundle.js` (build output), `src/assets/studio.css` (new build output)
- `src/dev/assets.rs`, `src/dev/serve.rs`, `build.rs`
- Tests that read those files: `src/dev/panel_scripts_tests.rs`, `src/dev/api_tests.rs` (only the two tests that `include_str!("../assets/viewer.html")`), `src/dev/studio_tests.rs`, `tests/theme.rs`
- `.github/workflows/desktop.yml` and `ci.yml`, only the "Build studio bundle" step if its command changes

Don't touch any Rust route handlers, `src/linter`, `src/config` or `desktop/`.

## What exists today (read these before you start)

- `src/dev/assets.rs`:
  - Embeds `viewer.html`, the 7 scripts, `theme.css`, the font and an inline favicon.
  - `viewer_html(desktop)` injects `<script>window.FUSIBLE_DESKTOP = true;</script>` after `<head>` and `desktop.js` before `</body>`.
  - `static_response()` maps the paths.
- `build.rs` sets `cfg(studio_bundle)` when `src/assets/studio.bundle.js` exists. Release builds embed it, and debug builds read it from disk (`load_studio_bundle`).
- `studio/src/index.ts`:
  - Exports `Tscircuit = { render3dViewer, renderPcbViewer, renderSchematicViewer, animateCadCameraPreset, ...Mechanical, ...Assembly }`.
  - Sets `window.Tscircuit`.
  - Dispatches `tscircuit-ready`.
  - Imports `iconify-icon` and pixelarticons.
  - These React 19 viewers are what renders the schematic, PCB and 3D. Keep them. Don't rewrite them.
- `viewer.html` DOM:

| Area | Contents |
|---|---|
| Views (`data-view`) | `schematic`, `pcb`, `3d`, `mechanical` (labelled "Bodies"), `assembly` (hidden unless the project has one) |
| Keyboard | 1 to 5 switch views |
| Sidebar panes | `pane-inspector`, `pane-components`, `pane-nets`, `pane-pins` (firmware view), `pane-runs`, `pane-checks` (diagnostics) |
| Extras | `project-tree` drawer with `tree-open`/`tree-close`/`tree-scrim`, `sync-view` (git), `bom-view`, mechanical controls (`btn-mechanical-overlay`, `btn-mechanical-lid`, `btn-mechanical-explode`), assembly controls, `live-indicator`, `fuse-status`, `hover-tooltip`, `studio-status` |

- Fetches in use:
  - `/api/project`, `/api/circuit?board=`, `/api/bom`, `/api/runs`, `/api/run`, `/api/firmware?name=`, `/api/kicad`
  - `/api/mechanical`, `/api/mechanical.glb`, `/api/assembly`, `/api/bodies/<n>.glb?rev=`, `/api/git*`
  - `EventSource('/events')`
- Read each `src/assets/*.js` file end to end before porting it. Behaviour must match.

## Steps

### 1. Build setup

In `studio/`:

```sh
bun remove @iconify-json/pixelarticons iconify-icon
bun add -d vite@^7 @sveltejs/vite-plugin-svelte@^6 svelte@^5 @tailwindcss/vite@^4 tailwindcss@^4 svelte-check@^4 @iconify-json/solar
bun add "@facile/muse@github:FacileStudio/muse#v0.10.0" gsap@^3.13 qrcode@^1.5
bun add -d @types/qrcode
```

`studio/vite.config.ts`:

```ts
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  build: {
    outDir: "../src/assets",
    emptyOutDir: false,
    cssCodeSplit: false,
    lib: { entry: "src/main.ts", formats: ["es"], fileName: () => "studio.bundle.js" },
    rollupOptions: { output: { inlineDynamicImports: true, assetFileNames: "studio[extname]" } },
    chunkSizeWarningLimit: 4000,
  },
  optimizeDeps: { exclude: ["@facile/muse"] },
});
```

- `studio/package.json` scripts: `"build": "vite build"`, `"check": "svelte-check --tsconfig ./tsconfig.json"`.
- Add `svelte.config.js` with `vitePreprocess` from `@sveltejs/vite-plugin-svelte`.
- `studio/src/app.css`:

  ```css
  @import "@facile/muse/styles";
  @source "../node_modules/@facile/muse/src";
  ```

  Plus the tscircuit override rules from the old `viewer.html` `<style>` that target viewer internals (hover outlines `pointer-events:none`, transparent backgrounds, Html annotation colors). Rewrite those with `var(--color-fc-*)` and with **no border declarations**.
- `bun run build` must write exactly `src/assets/studio.bundle.js` and `src/assets/studio.css`, and nothing else. If WASM or worker assets appear, inline them (`?url` imports turned into `?inline`, or `build.assetsInlineLimit: 100000000`).

### 2. Keep the tscircuit modules

- Rename `studio/src/index.ts` to `studio/src/tscircuit.ts`.
- Remove the `iconify-icon` and pixelarticons imports, `addCollection`, the `declare global` block and the `window.Tscircuit` assignment.
- Export `Tscircuit` as a named export.
- Keep `installRenderCapture()`.
- Every other file in `studio/src/` (assembly, mechanical, pcbViewer, schematicViewer, cadCamera, orbit, capture, contextMenu, refdes, roots, theme, schematic) stays. Edit them only where they reference pixelarticons, `window.Tscircuit`, or DOM ids that no longer exist. `rg -n "pixelarticons|window\.Tscircuit|getElementById" studio/src` lists them.

### 3. Svelte app

Each file is at most 250 lines. Each `.ts` module has at most 8 functions of at most 30 lines.

```
studio/src/
  main.ts                 mount(App, { target: document.getElementById("app")! })
  App.svelte              provideLocale("en"), registerIcons(STUDIO_ICONS), layout grid
  lib/icons.ts            STUDIO_ICONS: solar bodies read from @iconify-json/solar (pattern: Pastille apps/web/src/lib/icons.ts)
  lib/api.ts              typed fetch helpers, one per route (getProject, getCircuit, getBom, getRuns, startRun, getFirmware, getKicad, getRules, getBodies, startPair)
  lib/events.svelte.ts    EventSource("/events") wrapper: $state payloads keyed by board, reconnect with 1s→10s backoff
  lib/studio.svelte.ts    app state: activeView, activeBoard, selection, sidebarPane, theme, desktop flag (window.FUSIBLE_DESKTOP)
  lib/keys.ts             keyboard map 1–5 views, Escape closes drawer/dialog, existing shortcuts from viewer.html
  lib/project.ts          port of project.js
  lib/git.ts              port of git.js
  lib/firmware.ts         port of firmware.js
  lib/bom.ts              port of bom.js
  lib/kicad.ts            port of kicad.js
  lib/desktop.ts          port of desktop.js (window controls via window.__TAURI_INTERNALS__.invoke, traffic lights on macOS)
  components/TopBar.svelte        logo (solar:cpu-bolt-bold-duotone, 20px) + project name, board switcher (pill Select), ViewTabs, FusePill, LiveDot, theme IconButton, PairButton
  components/ViewTabs.svelte      pill group; icons: schematic solar:routing-2-linear, pcb solar:cpu-linear, 3d solar:box-linear, bodies solar:box-minimalistic-linear, assembly solar:layers-linear, bom solar:bill-list-linear, sync solar:refresh-circle-linear
  components/FusePill.svelte      intact = bg-fc-surface + solar:shield-check-bold-duotone "Fuse intact"; blown = bg-fc-danger text-fc-danger-fg + solar:danger-triangle-bold-duotone "Fuse blown · N"
  components/Viewport.svelte      hosts the active view; containers for renderSchematicViewer / renderPcbViewer / render3dViewer / Mechanical / Assembly
  components/Sidebar.svelte       muse Tabs (pill style) switching panes
  components/panes/Inspector.svelte
  components/panes/Components.svelte
  components/panes/Nets.svelte
  components/panes/Pins.svelte
  components/panes/Runs.svelte
  components/panes/Checks.svelte  diagnostics list; each row: rule id, message, help; if `standard` present show a pill linking `source_url` and the line "Design pre-check, not certification."
  components/panes/Rules.svelte   GET /api/rules (404 → hide pane); toggle display only, no writes
  components/panes/Bodies.svelte  GET /api/bodies (404 → fall back to the current mechanical list); name, kind, material, mass, cost
  components/ProjectDrawer.svelte muse Drawer, file tree from /api/project
  components/SyncView.svelte      git panel
  components/BomView.svelte
  components/FirmwareView.svelte
  components/KicadPanel.svelte
  components/PairDialog.svelte    muse Modal; POST /api/pair/start; render `uri` with qrcode.toString(uri,{type:"svg",margin:0}) inside a bg-white rounded-3xl p-4 tile; countdown to expires_at; 404 → text "Run fusible studio --share to pair a phone."
  components/HoverTooltip.svelte
  components/StatusToast.svelte   muse toast() for studio-status messages
```

**Layout:**
- `bg-fc-page` full-height grid: top bar (56px), then viewport and sidebar (360px, `bg-fc-surface`, `rounded-3xl`, 8px gutter).
- Below 900px the sidebar becomes a bottom sheet (muse `Drawer`).
- No borders. The viewport is `bg-fc-component rounded-3xl overflow-hidden`.

**Desktop shell:**
- When `window.FUSIBLE_DESKTOP` is true, `TopBar` leaves 78px on the left for the window pill on macOS and shows the "Projects" button.
- Port the behaviour of `desktop.js` into `lib/desktop.ts` and `TopBar`. Its CSS has `border: 0` only. Re-do it with classes.

### 4. `viewer.html`

Replace it with this exact file:

```html
<!doctype html>
<html lang="en" class="dark">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Fusible Studio</title>
  <link rel="icon" type="image/svg+xml" href="/favicon.svg">
  <script>try{var t=localStorage.getItem("fusible-theme");if(t==="light"){document.documentElement.classList.remove("dark");document.documentElement.classList.add("light")}}catch(e){}</script>
  <link rel="stylesheet" href="/studio.css">
  <script type="module" src="/studio.bundle.js"></script>
</head>
<body class="bg-fc-page text-fc-fg">
  <div id="app"></div>
</body>
</html>
```

### 5. `src/dev/assets.rs`

- Delete the consts `PROJECT_JS`, `BOM_JS`, `FIRMWARE_JS`, `DRAWER_JS`, `GIT_JS`, `KICAD_JS`, `THEME_CSS`, `FONT_DATA`, `FAVICON_SVG`, and their routes.
- Add `const FAVICON_SVG: &str = include_str!("../../assets/favicon.svg");`, served at `/favicon.svg` and `/favicon.ico`.
- Add `load_studio_css()`, mirroring `load_studio_bundle()` (both cfg variants, file `src/assets/studio.css`), served at `/studio.css` as `text/css; charset=utf-8`. The missing-file fallback is an empty stylesheet with status 200.
- `build.rs`: add `println!("cargo::rerun-if-changed=src/assets/studio.css");`.
- `viewer_html(desktop)`: keep the `window.FUSIBLE_DESKTOP = true` injection. Remove the `desktop.js` injection (the bundle handles it now).
- Delete the `.js` files, `theme.css` and `undefined-medium.otf` from `src/assets/`.
- Check `Cargo.toml` `include = [...]`. It already includes `/src/**/*`. Add `"/assets/favicon.svg"`.

### 6. Tests

- `src/dev/panel_scripts_tests.rs`: it asserts strings in `viewer.html` and `git.js`. Rewrite it so it asserts that:
  - `viewer.html` contains `<div id="app"></div>`, `/studio.bundle.js` and `/studio.css`;
  - `viewer_html(true)` contains `window.FUSIBLE_DESKTOP = true`;
  - `static_response("/favicon.svg", ...)` returns `image/svg+xml`.
- `src/dev/api_tests.rs` lines ~92 and ~100: update the asserted strings the same way.
- `tests/theme.rs`: read it. If it checks `theme.css` tokens, replace it with a test that `src/assets/studio.css` (when present) contains `--color-fc-page`. Skip the check when the file is absent (fresh clone without a bun build).
- Add `studio/src/lib/*.test.ts` with `bun test` for `api.ts` URL building, `events.svelte.ts` backoff, and `keys.ts`.

### 7. GLB and model loading (Pastille lessons)

- Every `.glb` fetch carries `?rev=<revision>` (already the case for bodies). Keep it for mechanical and assembly too.
- When the view or board switches while a model loads, drop the stale result: keep a request counter and ignore any response whose id isn't the latest.

### 8. CI

- In `.github/workflows/desktop.yml` and `ci.yml`, the step that builds the studio bundle must run `cd studio && bun install --frozen-lockfile && bun run build`.
- Check the current step text and change it only if it differs.

## Gates

```sh
cd studio && bun install && bun run check && bun test && bun run build && cd ..
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
rg -n "border|divide-|<hr|Divider|variant=\"outline\"" studio/src | rg -v "border-0|border: 0|border: none"
rg -n "pixelarticons" studio src
```

The last two commands must print nothing.

Manual run:

```sh
cargo run -- studio examples/circuit.json --port 3020 &
sleep 8
bunx playwright@1 screenshot --wait-for-timeout 4000 http://127.0.0.1:3020/ /tmp/claude-1000/studio-check.png
```

- Use `waitUntil: domcontentloaded`. Never use `networkidle`, because SSE keeps the network busy.
- Check that every view renders, the fuse pill shows, the checks pane lists diagnostics for `examples/circuit-broken.json`, and switching boards works.
- Kill the server afterwards.

## Exit criteria

- All gates pass.
- `fusible studio` shows all views and panes with muse styling.
- Live reload works: touch the example file and the payload updates.
- Desktop mode (`cargo run -- studio examples/circuit.json --desktop`) shows the Projects button.
- Report written to `docs/v1/reports/A.md`.
