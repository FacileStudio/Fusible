# Track G: fusible.facile.studio

Read `docs/v1/README.md` first. Load the `muse` skill.

Worktree: `/home/yann/Code/Kono/Fusible-v1-site`, branch `feat/v1-site`.

## Goal

A clean, fast, bilingual (en default, fr) static landing site for Fusible, with download and docs pages and complete SEO. It is deployed by Dokploy from `site/`. Inspiration comes from two sites the user calls "really clean": the Pastille landing (`/home/yann/Code/Facile/Pastille/apps/web`) and the facile.studio vitrine (`/home/yann/Code/Facile/vitrine`). Read both `+layout.svelte` and `+page.svelte` files fully before designing. Copy their structure and motion, not their content.

## Paths you own

- `site/` (new, all of it)
- `filet.yml`: you may add exactly `site/build`, `site/.svelte-kit` and `site/node_modules` to `ignore:`. Nothing else.

## Stack and setup

Copy from Pastille `apps/web`:

```sh
mkdir site && cd site
bun init -y
bun add -d @sveltejs/kit@^2.53 @sveltejs/adapter-static@^3 @sveltejs/vite-plugin-svelte@^6 svelte@^5.53 vite@^7 \
  @tailwindcss/vite@^4 tailwindcss@^4 svelte-check@^4 typescript@^5.9 @iconify-json/solar
bun add "@facile/muse@github:FacileStudio/muse#v0.10.0" gsap@^3.13 three@^0.186
```

- `vite.config.ts` and `svelte.config.js`: copy Pastille's verbatim (adapter-static, `prerender.entries: ['*']`, `optimizeDeps.exclude: ['@facile/muse']`).
- `src/app.css`:

  ```css
  @import '@facile/muse/styles';
  @source '../node_modules/@facile/muse/src';
  ```

- `src/app.html`: copy Pastille's head pattern (theme bootstrap in `try{}`, storage key `fusible-theme`, `theme-color` for light and dark). The SEO tags go in a `Seo.svelte` component instead (see below), because they differ per page and locale.
- `static/`:
  - `favicon.svg` (copy `../assets/favicon.svg`; if the worktree lacks it, use `../docs/v1/assets/favicon.svg`)
  - `logo.svg`
  - `apple-touch-icon.png` (180×180): `magick -background none -density 384 ../docs/v1/assets/logo-mark.svg -resize 180x180 static/apple-touch-icon.png`
  - `icon-512.png`
  - `robots.txt`
  - `og.png` (step 6)
  - `site.webmanifest`
  - `screenshots/` (placeholder PNGs now; the integrator drops real ones in track H with the same file names: `studio-pcb.png`, `studio-3d.png`, `studio-checks.png`, `studio-bodies.png`, `desktop.png`, `mobile.png`)

## Routes

| Route | File | Content |
|---|---|---|
| `/` | `src/routes/+page.svelte` | landing, English |
| `/fr` | `src/routes/fr/+page.svelte` | landing, French (same component, `locale="fr"`) |
| `/download`, `/fr/download` | | desktop installers, Android APK, CLI install |
| `/docs`, `/fr/docs` | | quick start, manifest, rules, agents (MCP) |
| `/sitemap.xml` | `src/routes/sitemap.xml/+server.ts` (`prerender = true`) | all 6 URLs with `xhtml:link` hreflang alternates |

- Put each page's content in `src/lib/content/{en,fr}.ts` as typed objects (`Content` type in `src/lib/content/types.ts`), so en and fr can't drift in shape.
- The components read `content` from a prop. Use muse `provideLocale(locale)` for muse's own strings.

## Landing sections (in order)

Use one component per section in `src/lib/components/sections/`, each at most 250 lines.

1. **`Nav`:**
   - Floating pill nav, like Pastille: flush at the top, floating with backdrop blur after scroll.
   - Logo `solar:cpu-bolt-bold-duotone`, then "Fusible", links (Features, Agents, Mobile, Docs, Download), a theme `IconButton`, and an en/fr pill switch.
   - Mobile: muse `Drawer` behind an `IconButton` `solar:hamburger-menu-linear`.
2. **`Hero`:**
   - H1 (en): "One project for your boards, firmware and enclosure."
   - Sub: "Fusible checks the whole design against electrical, physical and mechanical rules. When one fails, the fuse blows and nothing gets exported, generated or flashed."
   - Primary pill "Download" (`/download`). Secondary pill with a copy button showing `cargo install fusible`.
   - Below: a big screenshot `screenshots/studio-pcb.png` in a `rounded-3xl bg-fc-surface p-2` frame, with a GSAP reveal.
3. **`FuseLoop`:**
   - Four steps on a horizontal pinned scroll on desktop (≥1024px), stacked on mobile. Copy Pastille and vitrine's GSAP ScrollTrigger pattern; `prefersReducedMotion()` means no pin.
   - Steps: "Design" (TSX boards, code parts), "Check" (the fuse runs every rule), "Blown" (each violation names its rule, its standard and a fix), "Ship" (Gerbers, STEP, BOM, firmware pins: only when the fuse is intact).
4. **`Pillars`:** a 2×3 grid of `bg-fc-surface rounded-3xl` tiles, no borders, each with a Solar duotone icon:

| Icon | Tile | Text |
|---|---|---|
| `solar:cpu-bolt-bold-duotone` | Boards | tscircuit TSX, live PCB, schematic and 3D |
| `solar:bolt-circle-bold-duotone` | Physics rules | IPC-2152 current, IPC-2221B clearance, fab capability |
| `solar:code-square-bold-duotone` | Firmware | typed Rust pins for Embassy and ESP-IDF, blocked when the board is wrong |
| `solar:box-bold-duotone` | Mechanical | parts written in TypeScript on OpenCascade, real STEP, process-aware checks |
| `solar:bill-list-bold-duotone` | BOM | prices, picks, materials, print mass and cost |
| `solar:layers-linear` | Fab and KiCad | Gerbers, drill, STEP, GLB, KiCad round-trip |

5. **`Rules`:**
   - A list (not a table with lines): rule id in mono, the standard as a pill linking its source, a one-line summary.
   - Rules: `trace_current` (IPC-2152), `voltage_clearance` (IPC-2221B), `fab_profile` (JLCPCB 2/4-layer), `trace_clearance`, `shorted_nets`, `unconnected_net`, `pin_mux`, `bypass_cap`, `body_interference`, `mech.wall_thickness`, `mech.standoff_height`, `mech.draft`.
   - Footer line: "Design pre-checks, not certifications. Safety-rated products still need a lab."
   - Source URLs are the ones in `docs/v1/D-rules.md` and `E-mechanical.md`.
6. **`Agents`:**
   - "Let agents work on your boards."
   - Text: Fusible speaks MCP. Bulle, Claude Code and Codex can check, edit and build, and every edit returns the fuse verdict.
   - A code block with the Bulle YAML snippet from `docs/v1/F-mcp-suite.md` §1.5.
   - Make the claim concrete, with a source: "Research on PCB agents finds that closed-loop, rule-checked agents beat one-shot generation." Cite https://arxiv.org/html/2607.05915 and https://arxiv.org/html/2602.00510v1 as small text links.
7. **`Mobile`:** the `screenshots/mobile.png` phone mock (rounded-[2.5rem] frame, no border, `bg-fc-component`). Text: "A read-only view of boards, checks and the parts list. Pair with a QR code on your local network." Pill "Download for Android".
8. **`Desktop`:** `screenshots/desktop.png`. "Projects, toolchain and updates in one window. Updates are signed and install in the background."
9. **`Suite`:** "Part of Facile Studio". Pills linking `https://facile.studio`, `https://bulle.facile.studio` (Bulle, agents), `https://nuage.facile.studio` (Nuage, share fab files), `https://opus.facile.studio` (Opus, tasks from blown fuses).
10. **`Footer`:** logo, "MIT licensed", GitHub `https://github.com/FacileStudio/Fusible` (`solar:code-square-linear` + "GitHub"), crates.io `https://crates.io/crates/fusible`, Facile Studio. Use muse `LegalFooter` only if it fits without borders.

**French copy:** translate faithfully, plain register, and never translate standard names or code.
- Hero H1 fr: "Un seul projet pour vos cartes, votre firmware et votre boîtier."
- "Design pre-checks, not certifications" in fr: "Vérifications de conception, pas des certifications."

## `/download`

- **Build time** (`+page.server.ts` with `prerender = true`): `fetch("https://api.github.com/repos/FacileStudio/Fusible/releases/latest")`.
  - If the token is absent or there's a rate limit, fall back to `{version: null}`.
  - Map the assets:

| Asset | Shown as |
|---|---|
| Name matches `Fusible-Studio-*` and `.msi`/`.exe` | Windows |
| `.dmg` | macOS (universal) |
| `.AppImage`/`.deb`/`.rpm` | Linux |
| `Fusible-android.apk` | Android |

- Each card: `bg-fc-surface rounded-3xl`, a Solar icon (`solar:monitor-linear`, `solar:smartphone-linear`), file size and a SHA-256 link when a `.sha256` or `desktop-checksums.txt` asset exists.
- **Runtime fallback:** every button also works with no data. It links `https://github.com/FacileStudio/Fusible/releases/latest`.
- CLI block: `cargo install fusible`, `facile install fusible`, and "GitHub Action: `uses: FacileStudio/Fusible@v1`".
- Note for Android: "Allow installs from this source when Android asks."
- Note for macOS/Windows: "The installers are not code-signed yet; your OS will ask to confirm." This matches the README.

## `/docs`

One page with an in-page table of contents (sticky pill list on desktop). Sections:
- Install
- Quick start (`fusible init`, `fusible check`, `fusible studio`)
- The project file (boards, firmware, bodies, nets, fab, clearance_class)
- Rules (the same list as the landing, with the standards)
- Mechanical parts (`fusible body new`, the `Cad` helpers)
- Agents (MCP tools table and snippets)
- Phone pairing (`fusible studio --share`)
- Suite (`--opus`, `share nuage`)

Copy the facts from `README.md` and the track files. Don't invent flags. Every command shown must exist in `docs/v1/*.md` or in `cargo run -- --help`.

## SEO (`src/lib/components/Seo.svelte`, used by every page)

- `<title>`, by page:
  - en: "Fusible: boards, firmware and enclosures in one checked project"
  - fr: "Fusible : cartes, firmware et boîtiers dans un seul projet vérifié"
  - Download: "Download Fusible"
  - Docs: "Fusible docs"
- `<meta name="description">`, 150–160 characters, one per page and locale.
- `<link rel="canonical">` is the absolute URL on `https://fusible.facile.studio`. Add `<link rel="alternate" hreflang="en">`, `hreflang="fr"` and `hreflang="x-default"` (pointing at en).
- Open Graph: `og:type website`, `og:site_name Fusible`, `og:url`, `og:title`, `og:description`, `og:image https://fusible.facile.studio/og.png`, `og:image:width 1200`, `og:image:height 630`, `og:image:alt`, `og:locale en_US`/`fr_FR` and `og:locale:alternate`.
- Twitter: `twitter:card summary_large_image`, title, description, image.
- JSON-LD on `/` and `/fr`:

  ```json
  {"@context":"https://schema.org","@type":"SoftwareApplication","name":"Fusible",
   "applicationCategory":"DeveloperApplication","operatingSystem":"Windows, macOS, Linux, Android",
   "description":"<en description>","url":"https://fusible.facile.studio",
   "downloadUrl":"https://fusible.facile.studio/download","softwareVersion":"1.0.0",
   "license":"https://opensource.org/licenses/MIT","isAccessibleForFree":true,
   "offers":{"@type":"Offer","price":"0","priceCurrency":"EUR"},
   "author":{"@type":"Organization","name":"Facile Studio","url":"https://facile.studio"},
   "codeRepository":"https://github.com/FacileStudio/Fusible"}
  ```

- `robots.txt`: `User-agent: *`, `Allow: /`, `Sitemap: https://fusible.facile.studio/sitemap.xml`.
- `site.webmanifest`: name, short_name, icons (`icon-512.png`), `theme_color`, `background_color`.
- Use one `h1` per page and ordered headings. Every image gets `alt`, `width` and `height`, plus `loading="lazy"` below the fold.

## og.png

- Write `site/scripts/og.ts` (bun): build a 1200×630 SVG (dark `#0a0a0a` background, logo at 160px in white, "Fusible" 96px, the tagline 40px, system sans font). Convert it with `magick` to `static/og.png`.
- Commit the PNG. The script is re-runnable by the integrator after screenshots exist.

## Deploy files

- `site/Dockerfile`: copy Pastille's root `Dockerfile`, with paths adapted. The build context is `site/` itself:

  ```dockerfile
  FROM oven/bun:1 AS build
  WORKDIR /app
  COPY package.json bun.lock ./
  RUN bun install --frozen-lockfile
  COPY . .
  RUN bun run build

  FROM nginx:1.27-alpine
  COPY nginx.conf /etc/nginx/conf.d/default.conf
  COPY --from=build /app/build /usr/share/nginx/html
  EXPOSE 80
  HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 CMD wget -qO- http://127.0.0.1/ >/dev/null || exit 1
  ```

- `site/nginx.conf`: copy Pastille's `apps/web/nginx.conf`, and add `location = /og.png { expires 7d; }` and `location /screenshots/ { add_header Cache-Control "public, max-age=0, must-revalidate"; }`.
- `site/docker-compose.yml`:

  ```yaml
  services:
    fusible:
      build:
        context: .
        dockerfile: Dockerfile
      labels:
        - "journal.app=fusible"
      expose:
        - "80"
      restart: unless-stopped
  ```

  No Traefik labels. Dokploy domains route it, as for Pastille.
- `site/.dockerignore`: `node_modules`, `build`, `.svelte-kit`.

## Gates

```sh
cd site && bun install && bunx svelte-kit sync && bunx svelte-check && bun run build && ls build build/fr build/download build/docs && cd ..
rg -n "border|divide-|<hr|Divider|variant=\"outline\"" site/src | rg -v "border-0|border: 0|border: none"
docker build -t fusible-site:test site && docker run --rm -d -p 8089:80 --name fusible-site-test fusible-site:test && sleep 2 \
  && for p in / /fr /download /docs /sitemap.xml /robots.txt /og.png; do printf "%s %s\n" "$(curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:8089$p)" "$p"; done; docker stop fusible-site-test
bunx lighthouse@12 http://127.0.0.1:8089/ --only-categories=seo,accessibility,best-practices --chrome-flags="--headless=new" --quiet --output=json --output-path=/tmp/claude-1000/lh.json && bun -e 'const r=require("/tmp/claude-1000/lh.json");for(const[k,v] of Object.entries(r.categories))console.log(k,v.score)'
```

- Run Lighthouse while the container is still up, so move `docker stop` after it.
- Every path must return 200.
- SEO, accessibility and best-practices must each score ≥ 0.95.
- The border grep must print nothing.
- If `docker` isn't available, write that in the report and run `bunx vite preview` instead.

## Exit criteria

- All gates pass.
- en and fr pages are complete, and the content type check passes.
- `/download` renders with no release data (fallback links).
- Report written to `docs/v1/reports/G.md`, with the Lighthouse scores.
