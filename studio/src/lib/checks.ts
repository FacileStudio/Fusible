import type { Diagnostic, Rule, SetProp } from "./types";

export type Tone = "danger" | "warning" | "info";

export interface Group {
  category: string;
  items: { diag: Diagnostic; index: number }[];
}

const ORDER = ["netlist", "erc", "drc", "dfm", "dfa", "firmware", "mechanical", "upstream", "other"];

const LABELS: Record<string, string> = {
  netlist: "Netlist",
  erc: "Electrical",
  drc: "Layout",
  dfm: "Manufacturing",
  dfa: "Assembly",
  firmware: "Firmware",
  mechanical: "Mechanical",
  upstream: "tscircuit",
  other: "Other",
};

/** The colour family of a severity: errors are danger, warnings warning, infos neutral info. */
export function tone(severity: string | undefined): Tone {
  if (severity === "warning") return "warning";
  if (severity === "info") return "info";
  return "danger";
}

/** The heading shown for a rule category. */
export function categoryLabel(category: string): string {
  return LABELS[category] ?? category;
}

/** Groups diagnostics by the category the rule catalog gives their rule, in a fixed order. */
export function groupByCategory(diagnostics: Diagnostic[], rules: Rule[]): Group[] {
  const categories = new Map(rules.map((rule) => [rule.id, rule.category ?? "other"]));
  const groups = new Map<string, Group>();
  diagnostics.forEach((diag, index) => {
    const category = categories.get(diag.rule ?? "") ?? "other";
    const group = groups.get(category) ?? { category, items: [] };
    group.items.push({ diag, index });
    groups.set(category, group);
  });
  const rank = (category: string) => (ORDER.includes(category) ? ORDER.indexOf(category) : ORDER.length);
  return [...groups.values()].sort((a, b) => rank(a.category) - rank(b.category));
}

/** The machine-applicable `set_prop` fixes of a diagnostic. */
export function setPropFixes(diag: Diagnostic): SetProp[] {
  return (diag.fixes ?? []).flatMap((fix) => ("set_prop" in fix ? [fix.set_prop] : []));
}

/** How a `set_prop` fix reads on its button. */
export function fixLabel(fix: SetProp): string {
  return `Set ${fix.prop} = ${JSON.stringify(fix.value)} on ${fix.selector.replace(/^\./, "")}`;
}

interface Placed {
  type?: string;
  pcb_component_id?: string;
  center?: { x: number; y: number };
}

/** The PCB component whose centre is closest to (x, y), in millimetres; null on an empty board. */
export function nearestPcbComponent(circuit: unknown, x: number, y: number): string | null {
  if (!Array.isArray(circuit)) return null;
  let best: { id: string; distance: number } | null = null;
  for (const element of circuit as Placed[]) {
    if (element.type !== "pcb_component" || !element.pcb_component_id || !element.center) continue;
    const distance = Math.hypot(element.center.x - x, element.center.y - y);
    if (!best || distance < best.distance) best = { id: element.pcb_component_id, distance };
  }
  return best?.id ?? null;
}
