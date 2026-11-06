import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

function sources(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory() ? sources(join(dir, e.name)) : /\.(ts|svelte)$/.test(e.name) ? [join(dir, e.name)] : [],
  );
}

test("every theme colour referenced by name is defined", () => {
  const defined = new Set(
    [...readFileSync(join(import.meta.dir, "theme.ts"), "utf8").matchAll(/^\s+"?([a-z-]+)"?: "#/gm)].map((m) => m[1]),
  );
  const used = new Set<string>();
  for (const file of sources(import.meta.dir)) {
    const text = readFileSync(file, "utf8");
    for (const m of text.matchAll(/(?:themeHex|themeColor)\("([a-z-]+)"\)|(?:color|solid|overlay): "([a-z-]+)"/g)) {
      used.add(m[1] ?? m[2]);
    }
  }
  const missing = [...used].filter((name) => !defined.has(name));
  expect(missing).toEqual([]);
});
