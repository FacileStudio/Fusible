import type { Ctx } from "./fusible";

export default ({ board, cad }: Ctx) => {
  const outline: [number, number][] = board?.outline ?? [[0, 0], [60, 0], [60, 40], [0, 40]];
  const base = cad.shell(outline, 2.4, 22, 2.4);
  const posts = (board?.holes ?? []).map((h) => cad.boss(h.x, h.y, { insert: "m3", h: 5 }));
  const ports = (board?.openings ?? []).map((o) => cad.cutout(o));
  const vents = cad.ventSlots(-6, -12, 4, 1.6, 8, 4, "x");
  return cad.cut(cad.union(base, ...posts), ...ports, vents);
};
