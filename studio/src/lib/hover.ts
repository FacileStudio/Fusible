import { hideTip, lookup, previewComponent, selectComponent, selection, showTip } from "./selection.svelte";
import type { Comp } from "./types";

const SELECTORS = {
  schematic: ["[data-schematic-component-id]", "data-schematic-component-id"],
  pcb: ["[data-pcb-component-id], [data-component-id]", "data-pcb-component-id", "data-component-id"],
} as const;

function idOf(target: EventTarget | null, kind: keyof typeof SELECTORS): string | null {
  const [selector, ...attrs] = SELECTORS[kind];
  const el = (target as Element | null)?.closest(selector);
  if (!el) return null;
  return attrs.map((a) => el.getAttribute(a)).find(Boolean) ?? null;
}

function describe(comp: Comp, x: number, y: number): void {
  const rows: [string, string][] = [];
  if (comp.mpn) rows.push(["Part", comp.mpn]);
  rows.push(["Position", `(${comp.x.toFixed(2)}, ${comp.y.toFixed(2)}) mm`]);
  rows.push(["Layer", comp.layer || "top"], ["Pins", String(comp.pins.length)]);
  showTip({ x, y, name: comp.name, type: comp.ftype || "component", rows });
}

function onMove(event: MouseEvent, kind: keyof typeof SELECTORS): void {
  const comp = lookup(idOf(event.target, kind));
  if (comp) {
    previewComponent(comp);
    describe(comp, event.clientX, event.clientY);
    return;
  }
  const port = kind === "schematic" ? (event.target as Element).closest("[data-schematic-port-id]") : null;
  if (port) {
    const name = `Port ${port.getAttribute("data-schematic-port-id")}`;
    showTip({ x: event.clientX, y: event.clientY, name, type: "Port", rows: [] });
    return;
  }
  if (kind === "schematic") leave();
}

function leave(): void {
  hideTip();
  if (!selection.pinned) previewComponent(null);
}

/** Wires hover tooltips, the inspector preview and click selection on a viewer host; returns the detach function. */
export function attachHover(host: HTMLElement, kind: keyof typeof SELECTORS): () => void {
  const move = (event: MouseEvent) => onMove(event, kind);
  const click = (event: MouseEvent) => {
    const comp = lookup(idOf(event.target, kind));
    if (comp) selectComponent(comp.name);
  };
  host.addEventListener("mousemove", move);
  host.addEventListener("mouseleave", leave);
  if (kind === "schematic") host.addEventListener("click", click);
  return () => {
    host.removeEventListener("mousemove", move);
    host.removeEventListener("mouseleave", leave);
    host.removeEventListener("click", click);
  };
}
