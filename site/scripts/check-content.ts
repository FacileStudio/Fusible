import { en } from "../src/lib/content/en";
import { fr } from "../src/lib/content/fr";

let bad = 0;
for (const c of [en, fr]) {
  for (const [page, meta] of Object.entries(c.meta)) {
    const n = meta.description.length;
    const ok = n >= 150 && n <= 160;
    if (!ok) bad++;
    console.log(c.locale, page, n, ok ? "ok" : "OUT OF RANGE");
  }
}
/** Reduces a value to its key and type structure so en and fr can be compared. */
function shape(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(shape);
  if (!v || typeof v !== "object") return typeof v;
  const entries = Object.entries(v as Record<string, unknown>).sort(([a], [b]) => a.localeCompare(b));
  return Object.fromEntries(entries.map(([k, x]) => [k, shape(x)]));
}
if (JSON.stringify(shape(en)) !== JSON.stringify(shape(fr))) {
  console.log("en and fr shapes differ");
  bad++;
}
process.exit(bad ? 1 : 0);
