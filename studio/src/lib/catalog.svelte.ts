import { ApiError, api } from "./api";
import type { BodyInfo, Rule } from "./types";

export const catalog = $state({
  rules: [] as Rule[],
  rulesAvailable: true,
  bodies: null as BodyInfo[] | null,
});

/** Loads the rule catalog; a 404 means the server predates it and the pane stays hidden. */
export async function loadRules(): Promise<void> {
  try {
    catalog.rules = await api.getRules();
    catalog.rulesAvailable = true;
  } catch (err) {
    catalog.rulesAvailable = !(err instanceof ApiError && err.status === 404);
  }
}

/** Loads the body table; a 404 leaves `bodies` null so the pane falls back to the project list. */
export async function loadBodies(): Promise<void> {
  try {
    catalog.bodies = await api.getBodies();
  } catch (err) {
    if (!(err instanceof ApiError && err.status === 404)) console.warn("cannot read the bodies", err);
    catalog.bodies = null;
  }
}
