import type { Ctx } from "./fusible";

export default ({ board, cad }: Ctx) => {
  const outline: [number, number][] = board?.outline ?? [[0, 0], [60, 0], [60, 40], [0, 40]];
  const base = cad.shell(outline, 1.2, 22, 2);
  const vents = cad.ventSlots(-6, -12, 4, 0.6, 8, 4, "x");
  return cad.cut(base, vents);
};
