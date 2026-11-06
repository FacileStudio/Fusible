import { CLEARANCE, DEFAULT_THICKNESS, INSERTS, post, STANDOFF, type Env, type Size } from "./part_hw";
import { deepestBelow } from "./part_board";
import { cutout, vents, type Opening } from "./part_cut";
import { moves, solids } from "./part_ops";

type S = import("occt-wasm").ShapeHandle;

const ring = (env: Env, outline: [number, number][], offset: number) => {
  const { k } = env;
  const pts = outline.map(([x, y]) => ({ x, y, z: 0 }));
  const wire = k.makeWire(pts.map((p, i) => k.makeLineEdge(p, pts[(i + 1) % pts.length])));
  return k.offsetWire2D(wire, offset);
};

const tray = (env: Env, outline: [number, number][], wall: number, h: number, floor: number): S => {
  const { k, rec } = env;
  const zb = -(env.board?.thickness ?? DEFAULT_THICKNESS) / 2 - STANDOFF - floor;
  const face = (wire: S, z: number, dz: number) => k.translate(k.extrude(k.makeFace(wire), 0, 0, dz), 0, 0, z);
  rec.floorTop = zb + floor;
  rec.features.push({ kind: "wall", part: "wall", thickness: wall, at: [0, 0, zb] });
  rec.features.push({ kind: "wall", part: "floor", thickness: floor, at: [0, 0, zb] });
  rec.features.push({ kind: "clearance", gap: CLEARANCE, at: [0, 0, zb] });
  const outer = face(ring(env, outline, CLEARANCE + wall), zb, h);
  return k.cut(outer, face(ring(env, outline, CLEARANCE), zb + floor, h));
};

const column = (env: Env, x: number, y: number, d: number, h: number): S =>
  post(env.k, x, y, env.rec.floorTop, d, h);

const boss = (env: Env, x: number, y: number, o: { insert?: Size; screw?: Size; h: number }): S => {
  const hw = INSERTS[o.insert ?? o.screw ?? "m3"];
  const bore = o.insert ? hw.bore : hw.pilot;
  const depth = o.insert ? hw.length : o.h;
  const hole = o.insert ? { bore } : { pilot: bore };
  const top = env.rec.floorTop + o.h;
  const at: [number, number, number] = [x, y, env.rec.floorTop];
  const below = deepestBelow(env.board);
  const feature = { kind: "boss", insert: o.insert, screw: o.screw, ...hole, outer: hw.outer, h: o.h, below, at };
  env.rec.features.push(feature);
  return env.k.cut(column(env, x, y, hw.outer, o.h), post(env.k, x, y, top - depth, bore, depth + 1));
};

const standoff = (env: Env, x: number, y: number, h: number, hole: number): S => {
  const outer = Math.max(5, hole * 2);
  const at: [number, number, number] = [x, y, env.rec.floorTop];
  env.rec.features.push({ kind: "standoff", h, hole, outer, below: deepestBelow(env.board), at });
  return env.k.cut(column(env, x, y, outer, h), post(env.k, x, y, env.rec.floorTop, hole, h + 1));
};

/** Builds the `Cad` API of a part around one kernel and one feature recorder. */
export const makeCad = (env: Env) => ({
  ...solids(env),
  ...moves(env),
  shell: (o: [number, number][], wall: number, h: number, floor: number) => tray(env, o, wall, h, floor),
  boss: (x: number, y: number, o: { insert?: Size; screw?: Size; h: number }) => boss(env, x, y, o),
  standoff: (x: number, y: number, h: number, hole: number) => standoff(env, x, y, h, hole),
  ventSlots: (x: number, y: number, n: number, w: number, l: number, p: number, d: "x" | "y") =>
    vents(env, { x, y, n, w, l, pitch: p, dir: d }),
  cutout: (o: Opening, margin?: number) => cutout(env, o, margin),
  draft: (deg: number): void => void env.rec.features.push({ kind: "draft", deg, at: [0, 0, 0] }),
});
