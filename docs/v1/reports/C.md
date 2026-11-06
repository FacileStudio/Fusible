# Track C report
Branch: feat/v1-mobile   Last commit: see `git log -1` (the report commit follows 09b6208)

## Done
- `fusible studio --share`: binds 0.0.0.0, detects a private LAN IPv4 with the UDP-connect trick, exits 2 otherwise.
- `src/dev/pair/`: token, store, dirs, clock, lan, claim, remote, routes, summary, plus tests. Routes: `/api/pair/start|claim|devices`, `DELETE /api/pair/devices/<id>`, `/api/mobile/summary`.
- Remote gate: bearer, then GET only (405), then allowlist (403). Claim is the only open POST. Rate limit 5 failures per 60 s then 429 and the pending token is cleared.
- `mobile/`: Capacitor 8.6 Android app (onboarding, host list, scan, summary, board PCB/schematic/3D, checks, BOM, bodies), en and fr, no borders, Solar icons only, `?mock=1` fixtures.
- `.github/workflows/mobile.yml` modeled on Sablier's android job.
- Scanner plugin: `@capacitor/barcode-scanner` 3.1.3. Its peer is `@capacitor/core >=8.0.0`, so it fits Capacitor 8. The `@capacitor-mlkit/barcode-scanning` fallback was not needed. Android uses the ZXING library (no Play Services). The plugin has no explicit permission API: it asks for the camera when scanning starts, so onboarding step 3 only navigates to `/scan`.
- Gradle ran locally: `./gradlew assembleDebug --no-daemon` gave BUILD SUCCESSFUL (Android SDK at /home/yann/Android/Sdk, JDK 21). The release job is CI-only. Node 22 is needed for the Capacitor CLI (system node is 20).
- Manual end to end ran in a throwaway network namespace (`unshare -rn` with a dummy 192.168.50.2 interface), because this machine has a public IP and `--share` correctly refuses it (exit 2). Results: start gave a URI, claim 200, second claim 401, summary 200 with bearer, 401 without, POST /api/run 405, GET /api/git 403, `Host: 127.0.0.1` spoofed from the LAN address 401, devices list works on loopback, 403 remotely.
- Screenshots (390x844, `bun run preview`, `?mock=1`): `docs/v1/reports/C-screens/m1.png` to `m4.png`.

## Gates
- cargo fmt --check: ok
- cargo clippy -D warnings: Finished, no warnings
- cargo test --locked: 396 passed (21 new in `dev::pair`), integration suites all ok
- filet check: exit 0
- bun (mobile): svelte-check 0 errors 0 warnings; `bun test` 11 pass; `bun run build` ok; `bunx cap sync android` ok
- gradle assembleDebug: BUILD SUCCESSFUL in 3m 2s
- no-border grep: `rg -n "border|divide-|<hr|Divider|variant=\"outline\"" mobile/src | rg -v "border-0|border: 0|border: none"` -> no hits

## Not done / blocked
- The "Pair phone" button in the studio and the desktop toggle belong to tracks A and B. The server side is ready.
- `/api/bodies` (list) and `/api/rules` are in the remote allowlist but the routes come from tracks E and D. The phone does not depend on them: it uses `bodies` from the summary and `diagnostics` from `/api/circuit`.
- Schematic view of the mock fixture renders nothing (fixture has no schematic ports), so the screenshot shows the empty state. Real boards were not tried on a device.
- 3D board view bundles `circuit-json-to-gltf` (dynamic import) and the `buffer` polyfill, aliased in `mobile/vite.config.ts`. Not tested on a phone WebView.
- Release APK was not built locally (no keystore). CI is the check.

## Notes for the integrator (conflicts, contract questions)
- Ambiguity, `errors`/`warnings` in the summary: `DevDiagnostic` has no `severity` until track D lands. Fallback used: when no diagnostic has `severity`, all count as errors if the fuse is blown, else as warnings. With D's field they are counted by severity.
- Ambiguity, 429 timing: "more than 5 failures in 60 s". The failure is recorded first, so the 6th bad claim returns 429 and clears the pending token.
- Security decision beyond the spec: while sharing, a request is remote when the peer is not loopback, whatever its Host header. Without this, a LAN host could send `Host: 127.0.0.1:<port>` and bypass the whole gate. A loopback peer using the LAN Host is also treated as remote.
- Files touched outside the owned list, all additive or tiny: `src/dev/shared.rs` (State fields live there, not in state.rs), `src/dev/bom.rs` (extracted `report_of`, reused by the summary), `src/dev/project.rs` (`body_item` made `pub(super)`), `src/dispatch.rs`, `src/commands/dev.rs`, `src/main.rs` (thread `share`), `src/dev/bind_tests.rs` (new bind arg).
- Existing issue, not fixed: `GET /api/bom` calls `BomFile::load_or_create`, so a remote GET can create a starter `bom.yml`. The summary uses the non-creating `load`. Suggest switching `/api/bom` GET to `load`.
- `server.rs` `handle()` now starts with `guard::admit` (host check, remote gate, same-origin) and adds one arm `p if pair::owns(p)`. 
- Cargo.toml: added only `getrandom = "0.3"` and `sha2 = "0.10"`.
- muse v0.10.0 tag exists upstream and was used. `~/.claude/skills/muse/CHARTE.md` is missing on this machine; I read `/home/yann/Code/Facile/muse/CHARTE.md` instead.
- muse `Input` and `Badge` draw their own borders internally; the app passes `border-0` on `Input`. `Badge` uses `border-transparent`.
