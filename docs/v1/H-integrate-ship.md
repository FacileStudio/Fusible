# Track H: integrate, review, polish, ship v1.0.0, deploy (integrator)

Read `docs/v1/README.md` first. Start only when `docs/v1/reports/{A..G}.md` exist on their branches and each says its gates pass. Read every report fully before merging. Their "Notes for the integrator" sections list the conflicts and the stubs to wire.

Work on `main` in `/home/yann/Code/Kono/Fusible`.

## 1. Merge

The order minimizes conflicts: core first, UI last.

```sh
cd /home/yann/Code/Kono/Fusible
git checkout main && git pull --ff-only
for b in fab rules mech mcp mobile studio desktop site; do
  git merge --no-ff feat/v1-$b -m "merge: v1 $b track" || break
  cargo test --locked -q 2>&1 | tail -3
done
```

When a merge stops, resolve the conflict and run `cargo test`, then continue the loop from the next branch. Expected conflicts:

| File | Resolution |
|---|---|
| `src/cli.rs` / `src/cli/args.rs` | keep every new variant and flag (D `--json`, E `BodyAction::New`, F `Mcp`/`Share`/`--opus`, C `--share`) |
| `src/dev/server.rs` `handle()` | keep every new arm: `/api/rules` (D), `/api/bodies` (E), `/api/pair/*` + `/api/mobile/summary` + the remote branch (C). The remote-request branch from C must run **before** the `match`. |
| `src/config/types.rs`, `src/config/mod.rs` | keep every new field |
| `Cargo.toml` / `Cargo.lock` | union of the deps (only `getrandom` and `sha2` are expected), then `cargo generate-lockfile` is **not** allowed. Run `cargo update -w` only if the lock is broken. |
| `.github/workflows/*.yml` | keep both edits |
| `filet.yml` | only track G's `site/*` ignores |

## 2. Wire the stubs

- In track F's MCP tools, `check`, `explain_rule`, `body_new`, `edit_body` and `build_body` were stubbed until D and E merged. Wire them to `Linter::describe()`, the `--json` builder and E's body functions. Update `tests/mcp.rs` to assert real results.
- The studio (A) `Rules` and `Bodies` panes call `/api/rules` and `/api/bodies`. Check that they now render data instead of their 404 fallback.
- The studio `PairDialog` (A) and the server pairing (C): check the flow end to end (step 5).
- The desktop (B) `allow_phone_pairing` passes `--share`, which C added. Run `cargo run -- studio --help | grep share`.

## 3. Gates on the merged tree

Run all of these. Every one must pass.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
(cd studio && bun install --frozen-lockfile && bun run check && bun test && bun run build)
(cd desktop/ui && bun install --frozen-lockfile && bun run check && bun test && bun run build)
(cd desktop && bun test scripts)
(cd desktop/src-tauri && cargo clippy --all-targets -- -D warnings && cargo test)
(cd mobile && bun install --frozen-lockfile && bunx svelte-kit sync && bunx svelte-check && bun test && bun run build)
(cd site && bun install --frozen-lockfile && bunx svelte-kit sync && bunx svelte-check && bun run build)
filet check
rg -n --glob '!**/node_modules/**' --glob '!**/build/**' --glob '!**/dist/**' --glob '!src/assets/studio.*' \
  "border|divide-|<hr|Divider|variant=\"outline\"" studio/src desktop/ui/src mobile/src site/src | rg -v "border-0|border: 0|border: none"
rg -n "pixelarticons|KonoLabs|undefined-medium" --glob '!**/node_modules/**' --glob '!CHANGELOG.md' --glob '!docs/v1/**' .
```

The last two commands must print nothing.

## 4. Review

- Run the `facile-review` skill on `git diff <pre-v1 main sha>..main`, where the sha is the `chore: move repo` commit from track 00.
- Fix every finding of severity medium or higher, and re-run the gates.
- Check these Pastille lessons explicitly:
  - GLB URLs carry `?rev=`, and stale loads are ignored.
  - The theme is set before hydration and storage errors are caught.
  - STEP output is reproducible (E's test).
  - Every public claim on the site is sourced.
  - The release does not ship fab files from a blown board (step 7).

## 5. End-to-end manual checks

1. **Studio:** `cargo run -- studio examples/mech` (or any example with a manifest). Every view and pane renders, live reload works, the fuse pill flips on a broken edit, and Rules and Bodies show data.
2. **Pairing:** `cargo run -- studio examples/mech --share`. Press "Pair phone" and check that the QR shows. Then:
   - Claim with curl as in track C, and check that the summary returns.
   - Install the debug APK on a real Android phone if one is available (`adb install mobile/android/app/build/outputs/apk/debug/app-debug.apk`). Scan, and check the summary, board views, checks, BOM and bodies.
   - If no phone is available, say so in the final report. Don't claim it was tested.
3. **MCP:** add the Bulle snippet to a scratch Bulle config (`BULLE_` env or a temp config file; don't edit the user's real `~/.config/bulle/config.yml` without asking). Run one Bulle prompt: "check the board and explain the violations". Check that Bulle calls `check`.
4. **Desktop updater:**
   - Build the v0.20.9 desktop app from tag `v0.20.9` (`git worktree add /tmp/claude-1000/fus-old v0.20.9`, then build per `desktop.yml` locally for linux-x64).
   - After the v1.0.0 release is published (step 7), launch it and check that it finds 1.0.0, downloads, verifies and restarts into 1.0.0.
   - That proves the signing key is unchanged.
5. **Opus/Nuage:** with the user's consent only (they write to the real accounts). Otherwise rely on the fake-binary tests and say so.

## 6. Screenshots, logo, README, repo polish

Screenshots go in `assets/screenshots/`, as PNG, 2× device scale:

| Shot | Size | Command |
|---|---|---|
| `studio-pcb.png`, `studio-3d.png`, `studio-schematic.png`, `studio-checks.png`, `studio-bodies.png` | 1600×1000 | Playwright on `cargo run --release -- studio examples/mech --port 3020`, `page.goto(url, {waitUntil: "domcontentloaded"})`, wait 5 s, press 1–5 to switch views |
| `desktop.png` | 1280×800 | desktop UI in mock mode (`desktop/ui` `bunx vite preview`, `?skip-onboarding`) |
| `desktop-onboarding.png` | 1280×800 | mock mode, first run |
| `mobile.png`, `mobile-board.png` | 390×844 | `mobile` `bunx vite preview` with `?mock=1` |
| `site.png` | 1440×900 | built site |

Write a small `scripts/screenshots.ts` (bun + `playwright@1`) that takes them all. Copy the studio, desktop and mobile shots into `site/static/screenshots/` too, then re-run `site/scripts/og.ts`.

README rewrite (then run the `unslop` skill on it):
- Logo (`assets/logo.svg`, 64px, centred via `<p align="center">`), the name, a one-line pitch, badges (crates.io version, CI, license, site) as plain shields.io images.
- A gallery: one wide studio PCB shot, then a two-column table of studio-3d and studio-checks, then desktop and mobile side by side. Every image gets alt text.
- Sections: What you get (updated table: Linter, Physics rules, Studio, Mechanical, Firmware, BOM, Fab, Agents/MCP, Desktop, Mobile, Suite), Install, Quick start, The project file (with `nets`, `fab`, `clearance_class` and code bodies), Rules (table: id, what, standard, source URL), Agents (MCP snippet + link `docs/agents.md`), Phone pairing, Facile suite (Opus, Nuage, Bulle), Desktop app (updates are signed), Exit codes, License.
- Remove `assets/demo1.png` to `demo4.png` once nothing references them (`rg -n "demo[1-4]" .`).

Repo polish:

```sh
gh repo edit FacileStudio/Fusible \
  --description "Boards, firmware and enclosures in one checked project. Electrical, physical and mechanical rules gate every export. MCP-native." \
  --homepage https://fusible.facile.studio \
  --add-topic eda --add-topic pcb --add-topic electronics --add-topic hardware --add-topic tscircuit \
  --add-topic kicad --add-topic linter --add-topic rust --add-topic firmware --add-topic embassy \
  --add-topic cad --add-topic opencascade --add-topic mcp --add-topic ai-agents --add-topic 3d-printing \
  --add-topic facile-studio
```

- **Social preview:** GitHub has no API for it. Tell the user to upload `site/static/og.png` under Settings, General, Social preview.

## 7. Ship v1.0.0 (use the `ship` skill)

- **Versions:** set `1.0.0` in `Cargo.toml`, `desktop/src-tauri/Cargo.toml`, `desktop/src-tauri/tauri.conf.json`, `desktop/ui/package.json`, `mobile/package.json`, the Android `versionName` `1.0.0` and `versionCode` = previous + 1 (or 10000 if none), `site/package.json`, and the JSON-LD `softwareVersion` (already 1.0.0). The `desktop.yml` version check must pass.
- **CHANGELOG:** a `## [1.0.0] - 2026-11-xx` section covering:
  - Added: physics rules, `check --json`, mechanical workspace, materials, mechanical rules, MCP server, Opus/Nuage integration, phone pairing, mobile app, desktop onboarding, Svelte studio, site.
  - Changed: the repo moved to FacileStudio, the new UI, Solar logo.
  - Removed: pixel theme, Undefined font.
  - Security: phone pairing tokens, `requireSignedVersion`.
- **Release gate (Pastille lesson):** before tagging, run `cargo run -- check` on every example that is meant to pass, and on the FieldCam and Pastille projects (`/home/yann/Code/Kono/FieldCam`, `/home/yann/Code/Facile/Pastille`). Note their verdicts in the release notes. Never attach fab files from a blown board.
- Run the `release-preflight` flow, then follow the `ship` skill: tag `v1.0.0`, push the tag.
- Watch CI with the `ci-status` flow until `release`, `desktop` and `mobile` are green. Then:

  ```sh
  gh release view v1.0.0 --json assets -q '.assets[].name'
  ```

  This must list the CLI archives for every target in `release.yml`, `Fusible-Studio-*` installers plus `.sig`, `latest.json`, `desktop-checksums.txt`, `Fusible-android.apk` and its `.sha256`.
- Mark the release latest: `gh release edit v1.0.0 --latest`.
- crates.io: confirm the release workflow published it (`cargo search fusible --limit 1` shows `1.0.0`). If it didn't, run `cargo publish` locally (the crate owner is saravenpi; ask before publishing by hand).
- Run `facile install fusible` (or `facile doctor`) to confirm the catalog resolves `FacileStudio/Fusible`.

## 8. Deploy fusible.facile.studio

- Use the Dokploy API at https://gare.facile.studio. Read `~/.mycelium/memory/tools/dokploy.md` first: the CLI `compose one` 400s, so use tRPC directly with the token from `@dokploy/cli/config.json`, and send a curl User-Agent.
- Copy the GitHub provider fields from the existing Pastille compose (`dbHzDoz2X_iMl1N9V01S_`, read with `compose.one`).
- Create compose `Fusible` in project `Facile`, environment `production`:
  - `sourceType: github`, owner `FacileStudio`, repository `Fusible`, branch `main`
  - `composePath: ./site/docker-compose.yml`
  - `autoDeploy: true`
- Create the domain: host `fusible.facile.studio`, `serviceName: fusible`, port 80, https on, `certificateType: letsencrypt`. DNS is a Cloudflare wildcard and already resolves.
- Run `compose.deploy`, then poll `compose.one` deployments (sort by `createdAt`) until done. Remember that deploys queue serially behind other composes.
- Run the `deploy-verify` flow with `VERIFY_URLS="https://fusible.facile.studio https://fusible.facile.studio/fr https://fusible.facile.studio/download https://fusible.facile.studio/docs"`, then the `cert-expiry` flow with `CERT_HOSTS=fusible.facile.studio`.
- Add a Sonde monitor through the `sonde` skill: `https://fusible.facile.studio`, HTTP 200, 60 s interval.
- The vitrine links from track 00 now resolve. `curl -sI https://facile.studio` must still be 200.

## 9. Wiki (end gate)

- Rewrite `~/.mycelium/memory/projects/fusible.md`:
  - repo FacileStudio/Fusible, v1.0.0, and the Dokploy compose id;
  - the architecture of studio, desktop, mobile and site;
  - the MCP tool list, the pairing security model, and the rule data sources.
- Mark the Kono-styling findings `[SUPERSEDED by: v1.0.0, 2026-11-xx]`.
- Add a finding to `projects/vitrine.md` (the Fusible links) and to `projects/bulle.md` (the Fusible MCP snippet).
- Append to `log.md`.
- Run `mycelium doctor`.

## 10. Final report to the user

State only what is verified, each with its evidence:
- the release URL and asset list;
- the crates.io version;
- the site HTTP codes and Lighthouse scores;
- the Sonde monitor id;
- whether the desktop update from 0.20.9 worked;
- whether a real phone paired;
- anything skipped and why.

The user asked for the closing line "deployed and running, the implementation is wonderfully robust and production ready, enjoy creating projects with this ^^". Send it only if every item above is verified. Otherwise send the honest status.
