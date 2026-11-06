# Track B report
Branch: feat/v1-desktop   Last commit: d823dc0 (before this report)
## Done
- Rebuilt `desktop/ui` on Svelte 5 + muse v0.10.0 with vite 7 and Tailwind 4. Old vanilla TS, `styles.css`, the Undefined font and the build scripts are gone.
- Structure follows B-desktop.md: `App.svelte` (state routing, hash links), `lib/` (api, mock, updater store, projects store, settings store, theme, icons), `components/` (TitleBar, Sidebar, ProjectList, ProjectCard, NewProject, Settings with four tabs, UpdateBadge, five onboarding steps).
- `UpdaterStore` mirrors Courrier's states and calls the Rust commands. Ten transition tests, run through a bun preload that compiles `.svelte.ts` runes (`desktop/ui/scripts/test-setup.ts`).
- Rust: `Settings.onboarded` and `allow_phone_pairing` (serde default, false), `launch::studio_args` adds `--share` when pairing is on (unit tested), `requireSignedVersion: true` (tauri-plugin-updater 2.14.0 has it).
- App icons regenerated from `assets/logo-mark.svg` (android and ios folders removed, unused).
- Solar glyphs are bundled by `bun run icons` into `src/lib/solar-icons.json` (22 icons), so the bundle does not carry the full Solar set (300 kB JS instead of 12 MB).
- CI step "Build the desktop pages" already matched the spec; no change needed.
- Mock mode walked with playwright: onboarding (5 steps), projects, new project modal, four settings tabs, update flow available, downloading, ready. No console errors. `?skip-onboarding` works in mock mode only.
## Gates (paste the last lines of each)
- cargo fmt --check: clean
- cargo clippy: Finished, no warnings (-D warnings)
- cargo test: 32 passed, 0 failed
- filet check: no output, exit 0
- bun builds/tests: `bun run check` 0 errors, 0 warnings; `bun test` 26 pass, 0 fail; `bun run build` built; `bun test scripts` in desktop 6 pass
- no-border grep: `rg -n "border|divide-|<hr|Divider|variant=\"outline\"" desktop/ui/src | rg -v "border-0|border: 0|border: none"` -> no hits
- pixelarticons/theme.css/undefined-medium grep over desktop -> no hits
## Not done / blocked
- Toolchain "Install" button: no install flow exists in the Rust commands, so both screens show `fusible doctor --install` with a copy button, as the spec allows.
- `--share` is not verified against a real `fusible` binary (track C adds the flag); only the arg builder is tested.
- `requireSignedVersion` rejects any release whose signature lacks a version. Releases signed with an old tauri CLI will fail to update. CI uses `bunx @tauri-apps/cli@2`, which writes it; not verified on a real release.
- Not run: a real `tauri build` or a launch inside Tauri (needs sidecar binaries and a display). Local gates used empty stub sidecars (gitignored).
## Notes for the integrator (conflicts, contract questions)
- muse `SideBar` has no footer slot, so the version and update badge sit in a card under it. It navigates through `#/projects` and `#/settings` hash links.
- muse `Input`, `Select` and `IconButton` (default variant) draw borders; every use passes `border-0` or the ghost variant.
- `typescript` is pinned to ^5.9 in `desktop/ui` because `latest` is 7, which svelte-check 4 rejects.
- "Open a folder" in the onboarding sets `onboarded` first, because opening a project navigates the window away from the launcher.
- Mock UI window controls use `plugin:window|*` with `label: "main"`, as before.
