import { block, THROUGH, type Env } from "./part_hw";

type S = import("occt-wasm").ShapeHandle;
export type Opening = { x: number; y: number; z: number; w: number; h: number; side: string };

export type Vent = { x: number; y: number; n: number; w: number; l: number; pitch: number; dir: "x" | "y" };

export const vents = (env: Env, { x, y, n, w, l, pitch, dir }: Vent): S => {
  env.rec.features.push({ kind: "vent", w, l, n, pitch, at: [x, y, 0] });
  const slot = (i: number) => {
    const [cx, cy] = dir === "x" ? [x + i * pitch, y] : [x, y + i * pitch];
    const [sx, sy] = dir === "x" ? [w, l] : [l, w];
    return block(env.k, [cx - sx / 2, cy - sy / 2, -THROUGH / 2], [cx + sx / 2, cy + sy / 2, THROUGH / 2]);
  };
  return env.k.fuseAll(Array.from({ length: n }, (_, i) => slot(i)));
};

const reach = (o: Opening, m: number) => {
  const along = o.side.endsWith("x") ? [o.y - o.w / 2 - m, o.y + o.w / 2 + m] : [o.x - o.w / 2 - m, o.x + o.w / 2 + m];
  const out = o.side.startsWith("+") ? [0, 10] : [-10, 0];
  const base = o.side.endsWith("x") ? o.x : o.y;
  return { along, out: [base + out[0] - 1, base + out[1] + 1] };
};

export const cutout = (env: Env, o: Opening, margin = 1): S => {
  const { along, out } = reach(o, margin);
  const z = [o.z - margin, o.z + o.h + margin];
  env.rec.features.push({ kind: "port", margin, w: o.w, h: o.h, side: o.side, at: [o.x, o.y, o.z] });
  const horizontal = o.side.endsWith("x");
  const min = horizontal ? [out[0], along[0], z[0]] : [along[0], out[0], z[0]];
  const max = horizontal ? [out[1], along[1], z[1]] : [along[1], out[1], z[1]];
  return block(env.k, min, max);
};
