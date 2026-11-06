# Track A report
Branch: feat/v1-studio   Last commit: see git log (report commit is the tip)
## Done
- studio/ now builds with vite 7, Svelte 5 and Tailwind 4 into src/assets/studio.bundle.js and studio.css.
- Added the Svelte app on muse: TopBar, ViewTabs, FusePill, Viewport, Sidebar with panes (Inspect, Parts, Nets, Checks, Rules, Bodies, Runs, Pins), ProjectDrawer, SyncView, BomView, FirmwareView, KicadPanel, PairDialog, HoverTooltip, StatusToast.
- Ported project.js, git.js, firmware.js, bom.js, kicad.js, desktop.js and drawer.js into studio/src/lib. Board switches drop stale responses with a request counter.
- tscircuit viewers kept. window.Tscircuit, pixelarticons and DOM ids removed; viewers talk to the app through exported functions.
- viewer.html replaced; assets.rs serves /studio.css and /favicon.svg; old scripts, theme.css and the font deleted; build.rs and Cargo.toml updated.
- Tests: Rust tests rewritten; bun tests for api, events backoff, keys, circuit parsing and pair countdown.
- Checked in a browser: schematic, PCB, 3D, Bodies, Parts, Checks, drawer, board switch, pair dialog (404 text), light theme, desktop mode (Projects button, window controls), narrow bottom sheet, SSE payload updates.
## Gates (paste the last lines of each)
- cargo fmt --check: ok
- cargo clippy: Finished, no warnings
- cargo test: 440 passed, 0 failed
- filet check: no output (clean)
- bun builds/tests: svelte-check 0 errors; bun test 8 pass; bun run build ok
- no-border grep: rg -n "border|divide-|<hr|Divider|variant=\"outline\"" studio/src | rg -v "border-0|border: 0|border: none" -> no hits
- pixelarticons grep: no hits
## Not done / blocked
- Sync (git) panel lives in the project drawer, not as a ViewTabs entry; the plan lists a sync icon for ViewTabs, which was ambiguous.
- Rules pane is a read-only list with On/Off badges; "toggle display only" was ambiguous. /api/rules and /api/bodies were not testable (routes belong to tracks D and E); the 404 paths were.
- Live reload: examples/*.json have no pcb or schematic elements and json entries are not watched, so I verified with a .tsx entry that a second SSE payload reaches the page, not a visible diagram change.
- Light theme: PCB viewer and the 3D stage keep dark internals; the schematic remounts on theme change.
- Tab labels in the narrow top bar can scroll out of view at about 800px.
## Notes for the integrator (conflicts, contract questions)
- src/dev/desktop_tests.rs was edited (not in the owned list): its assertions read the removed desktop.js.
- studio.css is not gitignored; .gitignore is not mine. Add it next to studio.bundle.js.
- vite needs define process.env (set in vite.config.ts); typescript pinned to ^5.9 because svelte-check rejects 7.
- The debug server caches the bundle in memory: restart `fusible studio` after rebuilding.
- icons: bun run icons regenerates src/lib/solar-icons.json from @iconify-json/solar.
