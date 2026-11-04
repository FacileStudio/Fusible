# Track 00: move Fusible to FacileStudio (integrator, serial, first)

Read `docs/v1/README.md` first. This track runs alone, before A to G, because every track branches from its result.

## Paths you own

- Whole Fusible repo on `main`, for reference rewrites only.
- `/home/yann/Code/Facile/vitrine`
- `/home/yann/Code/Kono/Vitrine`
- `/home/yann/Code/Facile/Pastille`
- `/home/yann/Code/Kono/FieldCam`, links only.

## Steps

### 1. Preflight

```sh
cd /home/yann/Code/Kono/Fusible
git status -sb
git fetch origin && git log --oneline origin/main..main
gh secret list -R KonoLabs/Fusible
```

- `main` is 1 commit ahead of origin (`6e38042 fix(build): name the missing tool...`). Push it first with `git push origin main`.
- Expected secrets: `CARGO_REGISTRY_TOKEN`, `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.

### 2. Transfer the repo

This is outward-facing. The user approved it on 2026-11-04.

```sh
gh api repos/KonoLabs/Fusible/transfer -f new_owner=FacileStudio
sleep 10
gh repo view FacileStudio/Fusible --json name,visibility,url
git remote set-url origin https://github.com/FacileStudio/Fusible.git
git fetch origin
gh secret list -R FacileStudio/Fusible
```

- If any of the three secrets is missing after the transfer, stop and ask the user to re-add it with `gh secret set <NAME> -R FacileStudio/Fusible`.
- The Tauri key must be the same key. Its public key sits in `desktop/src-tauri/tauri.conf.json` `plugins.updater.pubkey`.

### 3. Rewrite references in Fusible

Run `rg -n "KonoLabs/Fusible|konolabs.vercel.app|Kono Labs|KonoLabs" --glob '!target' --glob '!node_modules' --glob '!dist' --glob '!CHANGELOG.md'`.

Replace:

| Old | New |
|---|---|
| `KonoLabs/Fusible` | `FacileStudio/Fusible` |
| `https://konolabs.vercel.app/fusible` | `https://fusible.facile.studio` |
| `https://konolabs.vercel.app/fusible/download` | `https://fusible.facile.studio/download` |
| "Kono Labs" as the author/org in prose | "Facile Studio" |

Known files:
- `Cargo.toml`: `repository = "https://github.com/FacileStudio/Fusible"`, and add `homepage = "https://fusible.facile.studio"` and `documentation = "https://fusible.facile.studio/docs"` below it.
- `facile.toml`: `repo = "FacileStudio/Fusible"`.
- `action.yml`, `src/config/template.rs`, `README.md`, `AGENTS.md`, `docs/DESKTOP_PLAN.md`.
- `desktop/src-tauri/tauri.conf.json` (the `endpoints` URL), `desktop/src-tauri/Cargo.toml`, `desktop/src-tauri/src/updates.rs` (`RELEASES_URL`), `desktop/src-tauri/src/actions.rs`.
- `desktop/ui/src/updateText.ts`, `desktop/ui/src/mockUpdate.ts`, `desktop/ui/src/updateState.test.ts`, `desktop/scripts/latest-json.test.ts`.

Leave old `CHANGELOG.md` entries alone (history). Then:

```sh
mkdir -p assets
cp docs/v1/assets/logo.svg docs/v1/assets/favicon.svg docs/v1/assets/logo-mark.svg assets/
magick -background none -density 384 assets/logo-mark.svg -resize 512x512 assets/logo.png
cargo test --locked 2>&1 | tail -3
(cd desktop/ui && bun test 2>&1 | tail -3)
(cd desktop && bun test scripts 2>&1 | tail -3)
filet check 2>&1 | tail -3
git add -A && git commit -m "chore: move repo to FacileStudio and add the solar logo"
git push origin main
```

Every gate must pass before the push.

### 4. Create the track worktrees

```sh
cd /home/yann/Code/Kono/Fusible
for t in studio desktop mobile rules mech mcp site fab; do git worktree add ../Fusible-v1-$t -b feat/v1-$t main; done
git worktree list
```

### 5. Facile vitrine (`/home/yann/Code/Facile/vitrine`, push to main, approved)

In `src/lib/data.ts`, change the Fusible entry (around line 570) to:

```ts
  {
    id: "fusible",
    name: "Fusible",
    tagline: "Boards, firmware and enclosures in one checked project",
    description: "Linter électrique et physique, firmware Rust généré, pièces 3D en code, BOM et fichiers de fabrication. Pilotable par Bulle via MCP.",
    icon: "solar:cpu-bolt-bold-duotone",
    category: "hardware",
    categoryLabel: "Hardware & IoT",
    url: "https://fusible.facile.studio",
    githubUrl: "https://github.com/FacileStudio/Fusible",
    tag: "Rust · PCB · CAD · MCP",
    badge: "Nouveau",
  },
```

- The vitrine copy is French, so the `tagline` must be French too: `"Cartes, firmware et boîtiers dans un seul projet vérifié"`.
- `src/lib/components/StudioGraph.svelte:15` already uses `solar:cpu-bolt-bold-duotone`. If it renders a link, add `href: "https://fusible.facile.studio"`.
- `src/lib/components/PcbViewer.svelte:169` says "ERC/DRC Fusible validés". Wrap "Fusible" in `<a href="https://fusible.facile.studio">`. Text links are never underlined there (wiki convention).

Then:

```sh
cd /home/yann/Code/Facile/vitrine && bun run build && git add -A && git commit -m "feat: link fusible to its site and facile repo" && git push origin main
```

### 6. Kono vitrine (`/home/yann/Code/Kono/Vitrine`, push to main, approved)

- `git rm -r src/routes/fusible src/lib/components/sections/Fusible.svelte`
- `src/routes/+page.svelte`: remove the `import Fusible ...` line (line 10) and `<Fusible />` (line 101).
- `src/lib/components/sections/TopNav.svelte`: remove `fusibleHref` (line 10) and both `<a href={fusibleHref}>FUSIBLE</a>` links (around lines 65 and 147). Keep the layout of the other links intact.
- `src/routes/sitemap.xml/+server.ts:3`: remove `"/fusible"` and `"/fusible/download"`.
- `src/lib/components/sections/FieldCam.svelte:34` and `src/routes/fieldcam/+page.svelte:51`: wrap the word "Fusible" in `<a href="https://fusible.facile.studio">Fusible</a>`. Leave the `keywords` string as is.
- `rg -n -i "fusible" src` must show only those FieldCam links and the keyword.

Then:

```sh
bun run build && git add -A && git commit -m "feat: remove the fusible pages now hosted by facile studio" && git push origin main
```

### 7. Other repos (links only, push to main)

- `/home/yann/Code/Facile/Pastille`:
  - In `README.md`, `apps/web/src/lib/data.ts` and `apps/web/src/routes/+page.svelte`, point every Fusible link at `https://fusible.facile.studio`, and every repo link at `https://github.com/FacileStudio/Fusible`.
  - Run `cd apps/web && bun run build`.
  - Commit `docs: point fusible links at the facile site`.
- `/home/yann/Code/Kono/FieldCam`: in `README.md`, `AGENTS.md`, `hardware/README.md` and `docs/*.md`, replace `KonoLabs/Fusible` and `konolabs.vercel.app/fusible` links. Leave `pins.rs`, `main.rs` and `CHANGELOG.md` alone (generated code and history). Commit `docs: point fusible links at the facile site`.
- Facile CLI catalog: `rg -n -i "KonoLabs/Fusible|konolabs" /home/yann/Code/Facile/facile /home/yann/Code/Facile/homebrew-tap /home/yann/Code/Facile/Suite --glob '!**/node_modules/**'`. Fix any hit the same way, run that repo's own check, commit, push.

## Exit criteria

- `gh repo view FacileStudio/Fusible` works, and `curl -sI https://github.com/KonoLabs/Fusible | head -1` shows a 301.
- `rg -n "KonoLabs/Fusible" /home/yann/Code --glob '!**/node_modules/**' --glob '!**/target/**' --glob '!**/CHANGELOG.md' --glob '!**/dist/**'` returns nothing.
- Eight worktrees exist (`git worktree list`).
- Both vitrines build and are pushed.
