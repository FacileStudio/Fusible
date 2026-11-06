# Track G report
Branch: feat/v1-site   Last commit: see `git log -1` (the report commit)
## Done
- `site/`: SvelteKit 2 static site (adapter-static, prerender all), muse v0.10.0, Svelte 5 runes, Tailwind 4.
- Routes `/`, `/fr`, `/download`, `/fr/download`, `/docs`, `/fr/docs`, `/sitemap.xml` (hreflang alternates), `robots.txt`, `site.webmanifest`.
- Landing sections, one component each: Nav, Hero, FuseLoop (GSAP ScrollTrigger pin at 1024px and up, 2x2 grid otherwise or under reduced motion), Pillars, Rules, Agents, Mobile and Desktop (ShotSection), Suite, Footer.
- Content typed in `src/lib/content/{types,en,fr}.ts`. `scripts/check-content.ts` fails when en and fr shapes differ or a meta description is outside 150-160 characters.
- `/download`: build-time GitHub latest-release fetch (`release.server.ts`), falls back to `{version: null}`; every button falls back to the releases/latest link.
- SEO: `Seo.svelte` (title, description, canonical, hreflang, Open Graph, Twitter, JSON-LD on `/` and `/fr`), `lang` set per locale through `hooks.server.ts`.
- `scripts/og.ts` writes `static/og.png`. Placeholder screenshots in `static/screenshots/` (same names the integrator will replace).
- Solar icons: `scripts/gen-icons.ts` reads `@iconify-json/solar` and writes `src/lib/icons-data.json`, registered with `registerIcons`.
- Deploy files: `Dockerfile`, `nginx.conf`, `docker-compose.yml`, `.dockerignore`. `filet.yml` gained the three allowed ignores.

## Gates
- svelte-check: 0 errors, 0 warnings
- bun run build: done, `Wrote site to "build"`
- filet check: exit 0
- check-content: all six descriptions 150-160 characters, en/fr shapes equal
- no-border grep: `rg -n "border|divide-|<hr|Divider|variant=\"outline\"" site/src | rg -v "border-0|border: 0|border: none"` -> no hits
- docker build and run: 200 for `/`, `/fr`, `/download`, `/fr/download`, `/docs`, `/fr/docs`, `/sitemap.xml`, `/robots.txt`, `/og.png`
- Lighthouse 12 (headless Chromium, container on :8089), accessibility / best-practices / seo:

| Page | a11y | best-practices | seo |
|---|---|---|---|
| `/` | 1.00 | 1.00 | 1.00 |
| `/fr` | 1.00 | 1.00 | 1.00 |
| `/download` | 1.00 | 1.00 | 1.00 |
| `/docs` | 1.00 | 1.00 | 1.00 |

## Not done / blocked
- Real screenshots: placeholders only (track H drops the real ones).
- `ls build/docs` from the plan gate fails by design: adapter-static writes `build/docs.html`, `build/fr.html` and so on, and nginx `try_files $uri.html` serves them (all return 200).
- `three` was not installed: no section uses it.
- Light theme was not audited with Lighthouse (only the default dark theme).

## Notes for the integrator
- Ambiguity: the desktop copy says "installers are not code-signed yet" (README) and "updates are signed" (G-site spec). Both are kept as written; they refer to OS code signing and updater signatures. Confirm before release.
- Ambiguity: the download description says "signed updates", same source.
- Docs list commands from tracks C, D, E, F (`--share`, `check --json`, `body new`, `mcp`, `--opus`, `share nuage`) that do not exist in `cargo run -- --help` yet; they come from the plan files and exist after those tracks merge. The `Pair phone` button label is taken from C-mobile.md.
- Rules without a standard in the plans (`trace_clearance`, `shorted_nets`, `unconnected_net`, `pin_mux`, `body_interference`) show no standard pill. `bypass_cap` uses the "layout guideline" source from D-rules.md.
- Version links: set `site/package.json` to 1.0.0 (already) and JSON-LD `softwareVersion` is 1.0.0.
- muse's CHARTE.md lives in `node_modules/@facile/muse/CHARTE.md`, not in `~/.claude/skills/muse/`.
- Build-time release fetch honors `GITHUB_TOKEN` when set; the Dokploy build has to provide it to avoid rate limits, else the page ships with fallback links.
