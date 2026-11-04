import { DEFAULT_THICKNESS, type Board } from "./part_hw";

type KitComponent = { name: string; layer: string; x: number; y: number; z?: number; height?: number };
type KitOpening = { component: string; side: string; from: [number, number]; to: [number, number] };
export type Kit = {
  outline: [number, number][];
  thickness: number;
  holes: { x: number; y: number; diameter: number; plated: boolean }[];
  components: KitComponent[];
  keep_out: { component: string; x: number; y: number; w: number; h: number; z0: number; z1: number }[];
  openings: KitOpening[];
};

const UNMEASURED = 2;

const span = (c: KitComponent | undefined, half: number): [number, number] => {
  if (c?.z !== undefined && c.height !== undefined) return [c.z, c.z + c.height];
  return c?.layer === "bottom" ? [-half - UNMEASURED, -half] : [half, half + UNMEASURED];
};

const opening = (o: KitOpening, components: KitComponent[], half: number) => {
  const [z0, z1] = span(components.find((c) => c.name === o.component), half);
  const w = Math.hypot(o.to[0] - o.from[0], o.to[1] - o.from[1]);
  return { name: o.component, x: (o.from[0] + o.to[0]) / 2, y: (o.from[1] + o.to[1]) / 2, z: z0, w, h: z1 - z0, side: o.side };
};

/** Turns the JSON of `fusible body kit` into the `board` of a part context. */
export const toBoard = (kit: Kit | null): Board | null => {
  if (!kit) return null;
  const thickness = kit.thickness ?? DEFAULT_THICKNESS;
  const half = thickness / 2;
  const measured = kit.components.filter((c) => c.z !== undefined && c.height !== undefined);
  return {
    outline: kit.outline,
    thickness,
    holes: kit.holes.filter((h) => !h.plated).map((h) => ({ x: h.x, y: h.y, d: h.diameter })),
    openings: kit.openings.map((o) => opening(o, kit.components, half)),
    components: measured.map((c) => ({ name: c.name, x: c.x, y: c.y, bottom: c.z!, top: c.z! + c.height! })),
  };
};

/** The component that reaches deepest under the board, with how far it reaches below the bottom face. */
export const deepestBelow = (board: Board | null): { component: string; depth: number } | null => {
  if (!board) return null;
  const half = board.thickness / 2;
  const depths = board.components.map((c) => ({ component: c.name, depth: -half - c.bottom }));
  const deepest = depths.sort((a, b) => b.depth - a.depth)[0];
  return deepest && deepest.depth > 0 ? deepest : null;
};
