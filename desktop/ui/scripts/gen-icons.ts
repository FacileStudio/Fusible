import solar from "@iconify-json/solar/icons.json";
import { mkdirSync, writeFileSync } from "node:fs";
import { NAMES } from "../src/lib/iconNames";

type Glyph = { body: string; width?: number; height?: number };
type Pack = { icons: Record<string, Glyph>; width?: number; height?: number };

const pack = solar as Pack;
const out: Record<string, { body: string; width: number; height: number }> = {};
const missing: string[] = [];

for (const name of NAMES) {
  const icon = pack.icons[name];
  if (!icon) missing.push(name);
  else {
    const width = icon.width ?? pack.width ?? 24;
    out[`solar:${name}`] = { body: icon.body, width, height: icon.height ?? pack.height ?? width };
  }
}

if (missing.length) {
  console.error(`Unknown Solar icons: ${missing.join(", ")}`);
  process.exit(1);
}

mkdirSync(`${import.meta.dir}/../src/lib`, { recursive: true });
writeFileSync(`${import.meta.dir}/../src/lib/solar-icons.json`, `${JSON.stringify(out, null, 1)}\n`);
