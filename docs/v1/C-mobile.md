# Track C: mobile viewer (Capacitor) and LAN pairing

Read `docs/v1/README.md` first. Load the `muse` skill.

Worktree: `/home/yann/Code/Kono/Fusible-v1-mobile`, branch `feat/v1-mobile`.

## Goal

A read-only Android app that pairs with a running `fusible studio --share` (or the desktop app with phone pairing on) by scanning a QR code. It shows each board's fuse state, checks, BOM, bodies, and 2D/3D board views. Nothing on the phone can edit a project. The server enforces that, not just the UI.

## Paths you own

- `mobile/` (new)
- `src/dev/pair/` (new module)
- `src/dev/bind.rs`, `src/dev/guard.rs`, `src/dev/state.rs` (additive changes)
- `src/dev/server.rs`: only new match arms and the remote-request branch in `handle()`
- `src/dev/mod.rs`: `mod pair;`
- `src/cli.rs`: the `--share` flag on `Studio` only. Pass it through to the dev server entry (`rg -n "Commands::Studio" src` finds the call site).
- `Cargo.toml`: you may add exactly `getrandom = "0.3"` and `sha2 = "0.10"`.
- `.github/workflows/mobile.yml` (new)

## Part 1: server side (Rust)

### 1.1 The `--share` flag

- `Studio { ..., #[arg(long)] share: bool }`, doc comment: `/// Also listen on the local network so a paired phone can view the project`.
- Thread `share: bool` into the dev server config (follow how `desktop` is threaded).
- `bind.rs`: when `share`, bind `0.0.0.0` instead of `127.0.0.1` with the same port search.
- **LAN IP detection, std only, no packet sent:**

  ```rust
  let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
  socket.connect("192.0.2.1:80")?;
  let ip = socket.local_addr()?.ip();
  ```

  Keep it only if `ip` is private (`10/8`, `172.16/12`, `192.168/16`). Otherwise refuse `--share` with exit code 2: `"no private LAN address found; --share only works on a local network"`.
- On start with `--share`, print:
  - `sharing on http://<ip>:<port> for paired phones (read-only)`
  - `pair a phone: open the studio and press "Pair phone"`

### 1.2 `State` additions

- `state.rs`: add `pub pair: Option<Arc<PairState>>` (None when not sharing) and `pub lan_host: Option<String>` (`"<ip>:<port>"`).
- Fix every constructor in tests with `pair: None, lan_host: None`.

### 1.3 `src/dev/pair/`

Each file has at most 8 functions and at most 250 lines.

```
pair/mod.rs        pub(super) use; PairState { pending: Mutex<Option<Pending>>, devices: Mutex<Devices>, failures: Mutex<Vec<Instant>> }
pair/token.rs      random_hex(bytes: usize) -> String (getrandom), sha256_hex(&str) -> String (sha2), ct_eq(a: &[u8], b: &[u8]) -> bool (length check then XOR-fold, no early return)
pair/store.rs      Devices load/save at state_dir()/devices.json: [{device_id, name, token_sha256, paired_at, last_seen}]; atomic write (write tmp + rename)
pair/dirs.rs       state_dir(): $XDG_STATE_HOME/fusible, else ~/.local/state/fusible on Linux; ~/Library/Application Support/fusible on macOS; %LOCALAPPDATA%\fusible on Windows; create_dir_all
pair/routes.rs     start, claim, list, revoke handlers
pair/remote.rs     authorize_remote(request, state) -> Result<(), u16>: bearer check + method + allowlist
pair/summary.rs    GET /api/mobile/summary builder
pair/tests.rs      unit tests
```

### 1.4 Semantics

These are exact. Test each one.

- `POST /api/pair/start` (loopback and same-origin only, the existing guard):
  - Make a 32-byte token, store `sha256(token)` plus `expires = now + 5 min`, replacing any pending one.
  - Return `{"uri":"fusible://pair?h=<lan_host>&t=<token>&n=<project name, percent-encoded>","expires_at":"<rfc3339 UTC>"}`.
  - Return 404 when `state.pair` is None, so the studio dialog shows "Run fusible studio --share".
  - Build the RFC 3339 time with std: seconds since epoch, then a small civil-date function. Look for an existing helper first: `rg -n "rfc3339|fn timestamp" src desktop/src-tauri/src`. `desktop/src-tauri/src/timestamp.rs` has one; copy its logic.
- `POST /api/pair/claim` (remote allowed), body `{"token":"<hex>","name":"<device name, max 64 chars>"}`:
  - If more than 5 failures in the last 60 s: return 429 and clear the pending token.
  - If there is no pending token, it has expired, or the hash doesn't match (`ct_eq`): record a failure and return 401.
  - On success:
    - clear the pending token (single use);
    - make `device_id` (16 random bytes as hex) and `device_token` (32 bytes as hex);
    - save `sha256(device_token)`;
    - return `{"device_id","device_token"}`.
- `GET /api/pair/devices` (loopback only) returns the list without hashes. `DELETE /api/pair/devices/<id>` (loopback only) returns 204 or 404.
- **Remote requests:** the Host isn't loopback, `share` is on, and Host equals `lan_host`.
  - Skip the same-origin check. Native HTTP sends no Origin header.
  - `POST /api/pair/claim` is the only non-GET allowed.
  - Every other remote request needs `Authorization: Bearer <device_token>` that matches a stored hash (`ct_eq` on the sha256 bytes). Otherwise 401.
  - Method must be GET, otherwise 405.
  - Path must be in this allowlist, otherwise 403: `/api/mobile/summary`, `/api/circuit`, `/api/bom`, `/api/bodies`, `/api/bodies/<name>.glb`, `/api/rules`, `/api/project`.
  - On success, update `last_seen` (at most one write per minute per device).
- `is_allowed_host`: accept `lan_host` only when sharing. Loopback rules stay exactly as they are.
- `GET /api/mobile/summary`:
  - `{"project": <manifest name or folder name>, "revision": <hash of all board revisions>, "boards":[{"name","fuse_blown","errors","warnings"}], "bom_total":{"amount","currency"}|null, "bodies":[{"name","kind"}], "last_run": <latest of /api/runs or null>}`.
  - Build it from the same state the existing `/api/circuit`, `/api/bom` and `/api/runs` handlers use. Call their internal functions, don't duplicate them.

### 1.5 Tests (`cargo test`)

- token: `ct_eq` equal/unequal/different length; sha256 known vector (`sha256("abc")` = `ba7816bf...15ad`).
- claim:
  - works once;
  - a second claim with the same token gives 401;
  - an expired token gives 401 (inject a clock: pass `now: Instant` into the function);
  - six bad claims in 60 s give 429 and clear the pending token.
- remote:
  - no bearer gives 401;
  - a wrong bearer gives 401;
  - POST to `/api/run` with a good bearer gives 405;
  - GET `/api/git` with a good bearer gives 403;
  - GET `/api/mobile/summary` with a good bearer gives 200.
- host: `lan_host` is accepted only when sharing.

## Part 2: the app (`mobile/`)

### 2.1 Stack

Match Sablier's versions (`/home/yann/Code/Facile/Sablier/apps/client/package.json`): Capacitor 8.6.

```sh
mkdir mobile && cd mobile
bun init -y
bun add -d @sveltejs/kit@^2 @sveltejs/adapter-static@^3 @sveltejs/vite-plugin-svelte@^6 svelte@^5 vite@^7 \
  @tailwindcss/vite@^4 tailwindcss@^4 svelte-check@^4 typescript@^5 @capacitor/cli@8.6.0 @iconify-json/solar
bun add @capacitor/core@8.6.0 @capacitor/android@8.6.0 @capacitor/app@8.1.2 @capacitor/preferences@8.0.1 \
  "@facile/muse@github:FacileStudio/muse#v0.10.0" gsap@^3.13 three@^0.186 circuit-to-svg circuit-json-to-gltf
bun add @capacitor/barcode-scanner
```

- After installing `@capacitor/barcode-scanner`, check that its `peerDependencies` accept `@capacitor/core` 8 (`cat node_modules/@capacitor/barcode-scanner/package.json`).
- If they don't, use `@capacitor-mlkit/barcode-scanning` at the version whose peer is `^8`. Record which one you used in the report.

`capacitor.config.ts`:

```ts
import type { CapacitorConfig } from "@capacitor/cli";
const config: CapacitorConfig = {
  appId: "studio.facile.fusible",
  appName: "Fusible",
  webDir: "build",
  server: { androidScheme: "https" },
  android: { allowMixedContent: false },
  plugins: { CapacitorHttp: { enabled: true } },
};
export default config;
```

- `CapacitorHttp` sends `fetch` through native HTTP. That's why the server sees no Origin header, and why the HTTPS webview can reach `http://<lan-ip>`.
- `svelte.config.js`: `adapter({ fallback: "index.html" })`.
- `src/routes/+layout.ts`: `export const ssr = false; export const prerender = false;`.
- Android cleartext:
  - Add `android/app/src/main/res/xml/network_security_config.xml` with `<base-config cleartextTrafficPermitted="true"/>`.
  - Reference it from `AndroidManifest.xml` (`android:networkSecurityConfig="@xml/network_security_config"`).
  - In code, `lib/hosts.ts` refuses any host that isn't an RFC 1918 IPv4 address with a port. That's the real guard. Write a unit test for it.
- Signing: copy the `signingConfigs` block from `/home/yann/Code/Facile/Sablier/apps/client/android/app/build.gradle` (env `ANDROID_KEYSTORE_PATH`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`, `ANDROID_KEY_PASSWORD`).
- Icons: generate from `../assets/logo-mark.svg` (`bunx @capacitor/assets generate --android` with `assets/icon.png` made by `magick -background none -density 384 ../assets/logo-mark.svg -resize 1024x1024 assets/icon.png`).

### 2.2 Code

Each file is at most 250 lines.

```
mobile/src/
  app.html                theme bootstrap like track A (dark default, key fusible-theme), viewport-fit=cover
  app.css                 @import "@facile/muse/styles"; @source "../node_modules/@facile/muse/src"; safe-area padding
  lib/i18n.ts             en + fr catalogs with muse createTranslator; provideLocale from navigator.language (fr* → fr)
  lib/icons.ts            solar bodies via registerIcons
  lib/hosts.ts            Host {id, name, url, token, paired_at}; load/save in Preferences key "fusible.hosts"; isPrivateHost(url)
  lib/hosts.test.ts
  lib/pair.ts             parsePairUri("fusible://pair?h=..&t=..&n=..") -> {host, token, name} | error; claim(host, token, deviceName)
  lib/pair.test.ts
  lib/client.ts           get<T>(host, path): fetch with Bearer; 401 → mark host "unpaired"; timeout 8s via AbortController
  lib/poll.svelte.ts      poll /api/mobile/summary every 5s while visible (App resume/pause from @capacitor/app); refetch details only when revision changes
  lib/board3d.ts          circuit_json → GLB (circuit-json-to-gltf) → three.js scene; OrbitControls; dispose on destroy
  routes/+layout.svelte   provideLocale, registerIcons, theme, bottom nav (muse MobileNav) inside a host
  routes/+page.svelte     first run → Onboarding; else host list (cards bg-fc-surface rounded-3xl), "Pair a studio" primary pill
  routes/scan/+page.svelte        scanner; on success parsePairUri → claim → save → go to host
  routes/host/+page.svelte        ?id=; summary: fuse pill per board, BOM total, last run, bodies count
  routes/host/board/+page.svelte  ?id=&board=; segmented pill: PCB (circuit-to-svg convertCircuitJsonToPcbSvg), Schematic (convertCircuitJsonToSchematicSvg), 3D (board3d); pinch zoom via CSS transform + pointer events
  routes/host/checks/+page.svelte diagnostics list; standard pill linking source_url; "Design pre-check, not certification."
  routes/host/bom/+page.svelte    BOM rows (designator, value, qty, price), total
  routes/host/bodies/+page.svelte list + 3D GLB view of /api/bodies/<name>.glb?rev=
  components/Onboarding.svelte    3 steps, GSAP slide, reduced-motion aware
  components/FusePill.svelte      same look as track A
  components/HostCard.svelte
```

- **No edit controls anywhere.** No buttons that call non-GET routes except the pairing claim.

### 2.3 Mobile onboarding

Its content differs from the desktop one. Every step has Continue (primary pill) and Skip (text pill). Stored as Preferences `fusible.onboarded = "1"`.

1. `solar:smartphone-2-bold-duotone` 56px, "Fusible on your phone". "See your boards, checks and parts list while you are away from the desk. Nothing here can change a design."
2. `solar:qr-code-linear`, "Pair with your studio". "On your computer run fusible studio --share, or turn on phone pairing in the desktop app. Press Pair phone and scan the code. Both devices must be on the same network."
3. Camera permission: `solar:scanner-linear`, "Allow the camera". "The camera is only used to scan the pairing code." The button requests permission through the scanner plugin, then goes to `/scan`.

### 2.4 Copy

- Every string is in `lib/i18n.ts` with `en` and `fr`.
- French examples: "Fusible sur votre téléphone", "Associer un studio", "Autoriser la caméra", "Vérification de conception, pas une certification."

## Part 3: CI (`.github/workflows/mobile.yml`)

- Model it on the `android` job in `/home/yann/Code/Facile/Sablier/.github/workflows/desktop.yml` (lines ~100 to 205). Copy its secret check, keystore decode, java 21, `android-actions/setup-android@v4`, `gradle/actions/setup-gradle@v4`, `bunx cap sync android`, `./gradlew assembleRelease --no-daemon`, `apksigner verify`, and keystore removal.
- Trigger: `push: tags: ["v*"]` and `workflow_dispatch` with an optional `tag` input.
- Secrets: `ANDROID_KEYSTORE_BASE64`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`, `ANDROID_KEY_PASSWORD`. If any is missing, fail with a clear message.
- Build steps:

  ```sh
  cd mobile && bun install --frozen-lockfile && bun run build && bunx cap sync android
  ```

- Output `Fusible-android.apk` plus `Fusible-android.apk.sha256`.
- Upload with the same `gh release create "$TAG" --latest=false || true` then `gh release upload --clobber` pattern as `desktop.yml` `publish`.
- Commit the generated `mobile/android/` project (Sablier commits its `android/`). Leave out build outputs (`android/app/build`, `.gradle`), and add those to `mobile/.gitignore`.

## Gates

```sh
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
cd mobile && bun install && bunx svelte-kit sync && bunx svelte-check && bun test && bun run build && bunx cap sync android && cd ..
(cd mobile/android && ./gradlew assembleDebug --no-daemon)
rg -n "border|divide-|<hr|Divider|variant=\"outline\"" mobile/src | rg -v "border-0|border: 0|border: none"
```

- If `./gradlew` can't run locally (no Android SDK), write that in the report. The CI job is then the check.
- The last command must print nothing.

**Manual end-to-end** (browser stand-in for the phone):
1. Run `cargo run -- studio examples/circuit.json --share`.
2. `curl -X POST -H "Host: 127.0.0.1:3020" http://127.0.0.1:3020/api/pair/start`, then claim with `curl -X POST http://<lan-ip>:3020/api/pair/claim -d '{"token":"..","name":"test"}'`.
3. `curl -H "Authorization: Bearer <device_token>" http://<lan-ip>:3020/api/mobile/summary` returns the summary.
4. `curl -X POST -H "Authorization: Bearer <t>" http://<lan-ip>:3020/api/run` returns 405.

## Exit criteria

- All gates pass and the curl sequence behaves as above.
- The app builds and shows onboarding, host list, summary, board views, checks, BOM and bodies (screenshots taken with `bun run preview` at 390×844, using a mock host flag `?mock=1` that loads `lib/fixtures.ts` data).
- Report written to `docs/v1/reports/C.md`, including the scanner plugin choice and whether gradle ran locally.
