import type { Env } from "./part_hw";
import { prism } from "./part_hw";

type S = import("occt-wasm").ShapeHandle;
const AXES = { x: { x: 1, y: 0, z: 0 }, y: { x: 0, y: 1, z: 0 }, z: { x: 0, y: 0, z: 1 } };

/** The primitive and boolean operations of `Cad`. They record no feature. */
export const solids = ({ k }: Env) => ({
  box: (w: number, d: number, h: number): S => k.makeBox(w, d, h),
  cylinder: (r: number, h: number): S => k.makeCylinder(r, h),
  extrude: (points: [number, number][], h: number): S => prism(k, points, 0, h),
  union: (...s: S[]): S => (s.length === 1 ? s[0] : k.fuseAll(s)),
  cut: (a: S, ...b: S[]): S => (b.length ? k.cutAll(a, b) : a),
  intersect: (a: S, b: S): S => k.common(a, b),
});

/** Moves, rotates and fillets. */
export const moves = ({ k }: Env) => ({
  move: (s: S, x: number, y: number, z: number): S => k.translate(s, x, y, z),
  rotate: (s: S, axis: "x" | "y" | "z", deg: number): S =>
    k.rotate(s, { point: { x: 0, y: 0, z: 0 }, direction: AXES[axis] }, (deg * Math.PI) / 180),
  fillet: (s: S, r: number): S => k.fillet(s, k.getSubShapes(s, "edge"), r),
});
