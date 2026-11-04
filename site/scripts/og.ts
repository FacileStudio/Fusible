import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const mark = readFileSync(new URL("../../docs/v1/assets/logo-mark.svg", import.meta.url), "utf8");
const inner = mark.replace(/^<svg[^>]*>/, "").replace(/<\/svg>\s*$/, "");
const TAGLINE = "One project for your boards, firmware and enclosure.";
const FONT = "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";

/** Builds the 1200x630 social card as an SVG string. */
function card(): string {
  return `<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="630" viewBox="0 0 1200 630">
<rect width="1200" height="630" fill="#0a0a0a"/>
<g transform="translate(96 150) scale(0.3125)" style="color:#fff">${inner}</g>
<text x="96" y="420" font-family="${FONT}" font-size="96" font-weight="600" fill="#ffffff">Fusible</text>
<text x="96" y="490" font-family="${FONT}" font-size="40" fill="#a3a3a3">${TAGLINE}</text>
</svg>`;
}

const svg = new URL("../.og.svg", import.meta.url).pathname;
const png = new URL("../static/og.png", import.meta.url).pathname;
writeFileSync(svg, card());
const run = spawnSync("magick", ["-background", "none", "-density", "96", svg, "-resize", "1200x630", png], {
  stdio: "inherit",
});
process.exit(run.status ?? 1);
