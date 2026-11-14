import { en } from "../src/lib/content/en";
import { fr } from "../src/lib/content/fr";
import { es } from "../src/lib/content/es";
import { de } from "../src/lib/content/de";

let bad = 0;
for (const c of [en, fr, es, de]) {
  for (const [page, meta] of Object.entries(c.meta)) {
    const n = meta.description.length;
    const ok = n >= 150 && n <= 160;
    if (!ok) bad++;
    console.log(c.locale, page, n, ok ? "ok" : "OUT OF RANGE");
  }
}
/** Reduces a value to its key and type structure so the locales can be compared. */
function shape(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(shape);
  if (!v || typeof v !== "object") return typeof v;
  const entries = Object.entries(v as Record<string, unknown>).sort(([a], [b]) => a.localeCompare(b));
  return Object.fromEntries(entries.map(([k, x]) => [k, shape(x)]));
}
for (const c of [fr, es, de]) {
  if (JSON.stringify(shape(en)) !== JSON.stringify(shape(c))) {
    console.log(`en and ${c.locale} shapes differ`);
    bad++;
  }
}
process.exit(bad ? 1 : 0);
