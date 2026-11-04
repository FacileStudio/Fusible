import type { Comp } from "./types";

/** Finds a component by name or by any of its element ids. */
export function findComponent(components: Record<string, Comp>, key: string | null | undefined): Comp | null {
  if (!key) return null;
  const wanted = String(key).trim();
  if (components[wanted]) return components[wanted];
  const ids = (c: Comp) => [c.name, c.id, c.pcbComponentId, c.sourceComponentId, c.schematicComponentId];
  return Object.values(components).find((c) => ids(c).includes(wanted)) ?? null;
}
