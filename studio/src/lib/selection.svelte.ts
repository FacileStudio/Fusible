import { parseCircuit } from "./circuit";
import { findComponent } from "./find";
import { openPane } from "./studio.svelte";
import type { Comp, Graph, Tip } from "./types";

export const graph = $state<Graph>({ components: {}, nets: {} });

export const selection = $state({
  pinned: null as string | null,
  preview: null as Comp | null,
  net: null as string | null,
  tip: null as Tip | null,
});

let tipTimer: ReturnType<typeof setTimeout> | undefined;

/** Rebuilds the component and net index from a circuit JSON payload. */
export function loadGraph(json: unknown): void {
  const next = parseCircuit(json);
  graph.components = next.components;
  graph.nets = next.nets;
}

/** Looks a component up by name or element id. */
export function lookup(key: string | null | undefined): Comp | null {
  return findComponent(graph.components, key);
}

/** Shows the tooltip at the given client position. */
export function showTip(tip: Tip): void {
  selection.tip = tip;
}

/** Hides the tooltip. */
export function hideTip(): void {
  selection.tip = null;
}

/** Pins a component in the inspector, as a click in a viewer or on a list row does. */
export function selectComponent(key: string): void {
  const comp = lookup(key);
  if (!comp) return;
  selection.pinned = comp.name;
  selection.preview = null;
  openPane("inspector");
}

/** Shows a hovered component in the inspector without pinning it. */
export function previewComponent(comp: Comp | null): void {
  selection.preview = selection.pinned ? null : comp;
}

/** Highlights a net and flashes its name over the viewport. */
export function selectNet(name: string): void {
  selection.net = name;
  const rows: [string, string][] = [["Net", name]];
  showTip({ x: window.innerWidth / 2, y: 72, name: `Net ${name}`, type: "Signal trace highlight", rows });
  clearTimeout(tipTimer);
  tipTimer = setTimeout(hideTip, 2000);
}

/** Clears the pinned component and the preview, as a board switch does. */
export function resetSelection(): void {
  selection.pinned = null;
  selection.preview = null;
  selection.net = null;
  hideTip();
}
