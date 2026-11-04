import type { Comp, Graph } from "./types";

type El = Record<string, any>;
type Source = { name: string; mpn: string; ftype: string };

function elementsOf(json: unknown): El[] {
  if (Array.isArray(json)) return json;
  const inner = (json as { elements?: unknown } | null)?.elements;
  return Array.isArray(inner) ? inner : [];
}

function sourcesOf(elements: El[]): Record<string, Source> {
  const sources: Record<string, Source> = {};
  for (const sc of elements.filter((e) => e.type === "source_component")) {
    sources[sc.source_component_id || sc.component_id] = {
      name: sc.name || sc.component_id || "U",
      mpn: sc.manufacturer_part_number || "",
      ftype: sc.ftype || sc.kind || "",
    };
  }
  return sources;
}

function netsOf(elements: El[]): Graph["nets"] {
  const nets: Graph["nets"] = {};
  for (const sn of elements.filter((e) => e.type === "source_net")) {
    const name = sn.name || sn.net_id || "NET";
    nets[name] = { name, count: 0 };
  }
  for (const trace of elements.filter((e) => e.type === "pcb_trace" && e.connection_name)) {
    const name = trace.connection_name;
    nets[name] = nets[name] || { name, count: 0 };
    nets[name].count++;
  }
  return nets;
}

function blank(name: string, id: string, src: Partial<Source>): Comp {
  return {
    name,
    mpn: src.mpn,
    ftype: src.ftype,
    x: 0,
    y: 0,
    w: 4,
    h: 4,
    layer: "top",
    id,
    sourceComponentId: id,
    pcbComponentId: id,
    schematicComponentId: null,
    pins: [],
  };
}

function placed(pc: El, src: Partial<Source>): Comp {
  const name = src.name || pc.pcb_component_id || "COMP";
  const comp = blank(name, pc.pcb_component_id, src);
  return {
    ...comp,
    x: pc.center?.x ?? pc.x ?? 0,
    y: pc.center?.y ?? pc.y ?? 0,
    w: pc.width || 4,
    h: pc.height || 4,
    layer: pc.layer || "top",
    sourceComponentId: pc.source_component_id,
    pcbComponentId: pc.pcb_component_id,
  };
}

function componentsOf(elements: El[], sources: Record<string, Source>): Record<string, Comp> {
  const comps: Record<string, Comp> = {};
  for (const pc of elements.filter((e) => e.type === "pcb_component")) {
    const comp = placed(pc, sources[pc.source_component_id] || {});
    comps[comp.name] = comp;
  }
  if (Object.keys(comps).length) return comps;
  for (const [id, src] of Object.entries(sources)) comps[src.name || id] = blank(src.name || id, id, src);
  return comps;
}

function attachSchematic(elements: El[], sources: Record<string, Source>, comps: Record<string, Comp>): void {
  for (const sc of elements.filter((e) => e.type === "schematic_component")) {
    const comp = comps[sources[sc.source_component_id]?.name ?? ""];
    if (!comp) continue;
    comp.schematicComponentId = sc.schematic_component_id;
    for (const [pin, net] of Object.entries<string>(sc.port_labels || {})) comp.pins.push({ pin, net });
  }
}

/** Builds the component and net index of a circuit JSON payload, as the inspector, parts and nets panes read it. */
export function parseCircuit(json: unknown): Graph {
  const elements = elementsOf(json);
  const sources = sourcesOf(elements);
  const components = componentsOf(elements, sources);
  attachSchematic(elements, sources, components);
  return { components, nets: netsOf(elements) };
}
