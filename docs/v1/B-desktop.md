# Track B: desktop app on Svelte 5 + muse, self-update, onboarding

Read `docs/v1/README.md` first. Load the `muse` skill.

Worktree: `/home/yann/Code/Kono/Fusible-v1-desktop`, branch `feat/v1-desktop`.

## Goal

Rebuild the desktop launcher UI (`desktop/ui`, vanilla TS today) in Svelte 5 + muse. Give it a clean self-update flow modelled on Courrier's and a first-run onboarding. Use the logo for the app icons. Keep every Tauri command.

## Paths you own

- `desktop/ui/` (all of it)
- `desktop/src-tauri/` (all of it)
- `desktop/scripts/`
- `.github/workflows/desktop.yml`

Don't touch `src/`, `studio/` or `mobile/`.

## What exists today (read before starting)

- **Tauri commands** (`desktop/src-tauri/src/commands/*.rs`): `check_update`, `install_update`, `restart_app`, `toolchain_status`, `list_projects`, `create_project`, `inspect_folder`, `forget_project`, `reveal_project`, `get_settings`, `set_settings`, `open_project`, `open_projects`, `last_error`. Plus the plugins dialog, opener, store, shell, single-instance and updater.
- **The updater is already in Rust:** `desktop/src-tauri/src/updates.rs` and `commands/update.rs`.
  - `check_update(auto)` returns `UpdateCheck` = `skipped | up_to_date | available{version,notes,date} | unsupported{url}`.
  - Progress is the event `update-progress` `{downloaded,total}`.
  - `install_update`, then `restart_app`.
  - **Keep this Rust side.** It works and is tested.
- **UI types:** `desktop/ui/src/types.ts` (`Project`, `Settings`, `ToolchainStatus`, `UpdateCheck`, `UpdateProgress`, `CreateArgs`).
- **API wrapper:** `desktop/ui/src/api.ts`, including the mock mode used when `window.__TAURI_INTERNALS__` is missing (`mock.ts`, `mockUpdate.ts`, `fixtures.ts`).
- **Tests:** `desktop/ui/src/helpers.test.ts`, `desktop/ui/src/updateState.test.ts` (bun test). `desktop/scripts/latest-json.test.ts`.
- **Build:** `desktop/ui/scripts/build.ts` writes `desktop/ui/dist` (`frontendDist` in `tauri.conf.json`). It copies `src/assets/theme.css` and the Undefined font. Those are going away (track A deletes them), so your build must not reference them.
- **Reference implementation:** Courrier's updater store, `/home/yann/Code/Facile/Courrier/apps/client/src/lib/updater.svelte.ts` and its test `updater.svelte.test.ts`. Read both.

## Steps

### 1. Build setup in `desktop/ui`

```sh
cd desktop/ui
bun add -d vite@^7 @sveltejs/vite-plugin-svelte@^6 svelte@^5 @tailwindcss/vite@^4 tailwindcss@^4 svelte-check@^4 @iconify-json/solar
bun add "@facile/muse@github:FacileStudio/muse#v0.10.0" gsap@^3.13
```

- `vite.config.ts`: plugins `tailwindcss()` and `svelte()`, `build.outDir: "dist"`, `emptyOutDir: true`, `optimizeDeps.exclude: ["@facile/muse"]`, `base: "./"`.
- Scripts: `"build": "vite build"`, `"dev": "vite --port 5173"`, `"check": "svelte-check --tsconfig ./tsconfig.json"`, `"test": "bun test"`.
- Delete `scripts/build.ts`, `scripts/dev.ts`, `styles.css`, `assets/undefined-medium.otf`, and every old `src/*.ts` once its logic has been ported.
- `index.html`: same theme bootstrap as track A (inline `try{}` script, storage key `fusible-theme`, but driven by `Settings.theme`: system, dark or light), `<div id="app">`, `<script type="module" src="/src/main.ts">`.
- `src/app.css`:

  ```css
  @import "@facile/muse/styles";
  @source "../node_modules/@facile/muse/src";
  ```

### 2. Structure

Each file is at most 250 lines.

```
desktop/ui/src/
  main.ts
  App.svelte                 routes between Onboarding, Projects, Settings with $state (no router lib)
  lib/api.ts                 keep the existing api.ts content (typed invoke + mock); add getSettings/setSettings fields below
  lib/types.ts               keep; add Settings.onboarded: boolean, Settings.allow_phone_pairing: boolean
  lib/updater.svelte.ts      UpdaterStore (see step 3)
  lib/updater.svelte.test.ts
  lib/projects.svelte.ts     project list state, filter, sort by last_opened
  lib/theme.ts               apply Settings.theme to <html> class, write fusible-theme
  lib/icons.ts               solar bodies registered with registerIcons
  lib/mock.ts, lib/fixtures.ts, lib/mockUpdate.ts   moved from src/, unchanged behaviour
  components/TitleBar.svelte     drag region (data-tauri-drag-region), window controls on Windows/Linux (port windowControls.ts), macOS keeps native traffic lights; logo + "Fusible"
  components/Sidebar.svelte      muse SideBar with NavButton: Projects (solar:folder-open-linear), Settings (solar:settings-linear); footer shows version + UpdateBadge
  components/ProjectList.svelte  cards bg-fc-surface rounded-3xl, no borders; name, path (muted), boards/firmware/bodies counts as pill Badges, last opened; actions: Open (primary pill), Reveal, Forget (IconButton)
  components/NewProject.svelte   muse Modal: folder picker (api.pickFolder), name Input, create-folder Switch, currency Select (EUR, USD, GBP); validation messages from validate.ts logic
  components/EmptyProjects.svelte muse EmptyState with "New project" and "Open folder"
  components/Settings.svelte     muse Tabs: General, Toolchain, Updates, Phone
  components/settings/General.svelte   projects_dir, default_currency, port, open_last_project, theme
  components/settings/Toolchain.svelte toolchain_status: bun, node, tsci rows (SettingsRow), versions, "Install" pill runs the existing install flow if present, else shows the `fusible doctor --install` command with a copy button
  components/settings/Updates.svelte   current version, auto_update Switch, Check now, progress bar, Restart to update
  components/settings/Phone.svelte     allow_phone_pairing Switch + explanation; when on, text "Open a project and press Pair phone in the studio top bar."
  components/UpdateBadge.svelte  pill shown when updater.state is available|ready
  components/onboarding/Onboarding.svelte  step container, GSAP slide between steps, reduced-motion = no animation
  components/onboarding/Welcome.svelte
  components/onboarding/Toolchain.svelte
  components/onboarding/FirstProject.svelte
  components/onboarding/Phone.svelte
  components/onboarding/Agents.svelte
```

### 3. `UpdaterStore` (`lib/updater.svelte.ts`)

Copy Courrier's shape and states, but call Fusible's Rust commands instead of `@tauri-apps/plugin-updater`.

- `state: "idle" | "checking" | "available" | "downloading" | "ready" | "error" | "upToDate"`, plus `version`, `notes`, `progress {downloaded,total}`, `error`.
- `checkNow(auto: boolean)` calls `api.checkUpdate(auto)` and maps the result:

| `UpdateCheck` | State |
|---|---|
| `skipped` | `idle` |
| `up_to_date` | `upToDate` |
| `available` | `available` |
| `unsupported` | `error`, with a link to `url` |

- `installNow()`: `downloading`, subscribes to `onUpdateProgress`, awaits `api.installUpdate()`, then `ready`.
- `restartNow()` calls `api.restartApp()`.
- On boot, `App.svelte` calls `updater.checkNow(true)` when `settings.auto_update` is true. The Rust side already throttles to once per 24h.
- Tests: port `updateState.test.ts`, and add one test per transition using the mock api.

### 4. Signed-version hardening (`desktop/src-tauri`)

- Run `cargo tree -p tauri-plugin-updater` in `desktop/src-tauri` to get the version. Open `https://docs.rs/tauri-plugin-updater/<that version>/tauri_plugin_updater/struct.Config.html`.
- If `require_signed_version` exists in that version:
  - set `"requireSignedVersion": true` under `plugins.updater` in `tauri.conf.json`;
  - confirm the CI's tauri CLI writes the version into the signature's trusted comment (it builds with `bunx @tauri-apps/cli@2`). Read the field doc on docs.rs, which says releases signed before that CLI change carry no version.
- If the field doesn't exist, run `cargo update -p tauri-plugin-updater` to the newest 2.x that has it. If none has it, write that in your report and skip this step.
- Keep `endpoints` as `https://github.com/FacileStudio/Fusible/releases/latest/download/latest.json` (track 00 sets it).

### 5. Settings additions (Rust)

- `desktop/src-tauri/src/model.rs` `Settings`: add `onboarded: bool` and `allow_phone_pairing: bool`, both `#[serde(default)]`, default `false`.
- `validate.rs`: pass them through.
- Where the studio sidecar is launched (`desktop/src-tauri/src/studio.rs` / `launch.rs`, find the arg list with `rg -n "\"studio\"" desktop/src-tauri/src`): append `"--share"` when `allow_phone_pairing` is true. Track C adds the `--share` flag to the CLI. Until the merge it's unknown to your local `fusible`, so guard your test with the arg-list builder only (a unit test on the function that builds args).
- Update `model` and `validate` tests.

### 6. Onboarding (desktop flavour)

- Show it when `settings.onboarded` is false. Every step has "Skip" (text pill) and "Continue" (primary pill). The last step sets `onboarded = true` through `set_settings`.
- **Welcome:**
  - `solar:cpu-bolt-bold-duotone` at 56px, title "Welcome to Fusible".
  - Subtitle: "Boards, firmware and enclosures in one checked project. When a rule fails, the fuse blows and nothing ships."
  - Three feature rows: `solar:bolt-circle-bold-duotone` "Checks electrical and physical rules", `solar:code-square-bold-duotone` "Generates firmware pin bindings", `solar:box-bold-duotone` "Builds parts and enclosures from code".
- **Toolchain:**
  - Runs `toolchain_status` and lists bun, node and tsci with `check-circle` or `close-circle` icons.
  - When `bundled_bun` is set: "Fusible bundles bun, you are ready."
  - Otherwise show `fusible doctor --install` with a copy button.
- **First project:**
  - Two big option cards (muse `OptionCards`): "Create a project" (opens `NewProject`) and "Open a folder" (`pickFolder`, then `inspect_folder`, then `open_project`).
  - Skippable.
- **Phone:**
  - Switch "Allow pairing a phone on this network" bound to `allow_phone_pairing`.
  - Text: "Your phone gets a read-only view of boards, checks and the BOM. Pairing needs a code shown by the studio and only works on your local network."
- **Agents:**
  - Title "Let agents work on your boards".
  - Text: "Fusible speaks MCP. Bulle, Claude Code or Codex can check boards, edit them and build parts, and every edit returns the fuse verdict."
  - Two read-only code blocks, each with a copy button.
  - The first is "Bulle (`~/.config/bulle/config.yml`)":

    ```yaml
    sources:
      mcp:
        fusible:
          command: fusible
          args: [mcp]
    ```

  - The second is "Claude Code": `claude mcp add fusible -- fusible mcp`.
  - These match track F's `fusible mcp --print-config`.

### 7. App icons

- Run `bunx @tauri-apps/cli@2 icon ../../assets/logo-mark.svg` from `desktop/src-tauri`. The SVG lands in the repo root `assets/` in track 00. If your worktree predates it, copy `docs/v1/assets/logo-mark.svg`.
- Commit the generated `icons/`.

### 8. CI (`.github/workflows/desktop.yml`)

- The "Build the desktop pages" step becomes `cd desktop/ui && bun install --frozen-lockfile && bun run build`.
- The studio bundle step (`cd studio && bun install --frozen-lockfile && bun run build`) stays before it.
- Don't change the matrix, signing or release steps.

## Gates

```sh
cd desktop/ui && bun install && bun run check && bun test && bun run build && cd ../..
cd desktop && bun test scripts && cd ..
cd desktop/src-tauri && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test && cd ../..
filet check
rg -n "border|divide-|<hr|Divider|variant=\"outline\"" desktop/ui/src | rg -v "border-0|border: 0|border: none"
rg -n "pixelarticons|theme.css|undefined-medium" desktop
```

The last two commands must print nothing.

Visual check in mock mode:

```sh
cd desktop/ui
bun run build
bunx vite preview --port 5174 &
sleep 2
bunx playwright@1 screenshot --viewport-size=1200,800 http://127.0.0.1:5174/ /tmp/claude-1000/desktop-check.png
```

The first screenshot must show onboarding (mock settings have `onboarded: false`). Add a `?skip-onboarding` query read in mock mode only, so the projects screen can be captured too.

## Exit criteria

- All gates pass.
- In mock mode you can walk through onboarding, projects, new project and every settings tab, and the update flow goes through available, downloading and ready.
- Report written to `docs/v1/reports/B.md`.
