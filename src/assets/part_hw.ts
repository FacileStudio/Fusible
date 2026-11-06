import type { OcctKernel, ShapeHandle } from "occt-wasm";

export type Size = "m2" | "m2.5" | "m3";
export type Hardware = { bore: number; length: number; outer: number; clearance: number; pilot: number };
export type Feature = { kind: string; at: [number, number, number]; [key: string]: unknown };
export type Rec = { features: Feature[]; floorTop: number };
export type Board = {
  outline: [number, number][];
  thickness: number;
  holes: { x: number; y: number; d: number }[];
  openings: { name: string; x: number; y: number; z: number; w: number; h: number; side: string }[];
  components: { name: string; x: number; y: number; top: number; bottom: number }[];
};
export type Env = { k: OcctKernel; rec: Rec; board: Board | null };

/** Typical brass heat-set insert sizes. They are not from a datasheet: check your insert's datasheet. */
export const INSERTS: Record<Size, Hardware> = {
  m2: { bore: 3.2, length: 4, outer: 6, clearance: 2.4, pilot: 1.6 },
  "m2.5": { bore: 3.6, length: 5, outer: 7, clearance: 2.9, pilot: 2.1 },
  m3: { bore: 4, length: 5.7, outer: 8, clearance: 3.4, pilot: 2.5 },
};

export const CLEARANCE = 0.5;
export const STANDOFF = 5;
export const DEFAULT_THICKNESS = 1.6;
export const THROUGH = 400;

export const prism = (k: OcctKernel, points: [number, number][], z: number, h: number): ShapeHandle => {
  const ring = points.map(([x, y]) => ({ x, y, z: 0 }));
  const wire = k.makeWire(ring.map((p, i) => k.makeLineEdge(p, ring[(i + 1) % ring.length])));
  return k.translate(k.extrude(k.makeFace(wire), 0, 0, h), 0, 0, z);
};

export const block = (k: OcctKernel, min: number[], max: number[]): ShapeHandle =>
  k.makeBoxFromCorners(
    { x: min[0], y: min[1], z: min[2] },
    { x: max[0], y: max[1], z: max[2] },
  );

export const post = (k: OcctKernel, x: number, y: number, z: number, d: number, h: number): ShapeHandle =>
  k.translate(k.makeCylinder(d / 2, h), x, y, z);
