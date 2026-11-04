import type { Ctx } from "./fusible";

export default ({ cad }: Ctx) => {
  const plate = cad.box(40, 20, 4);
  const wall = cad.box(40, 4, 24);
  const holes = [8, 32].map((x) => cad.move(cad.cylinder(1.7, 6), x, 12, -1));
  const bracket = cad.cut(cad.union(plate, wall), ...holes);
  return cad.move(bracket, 40, -10, -4);
};
