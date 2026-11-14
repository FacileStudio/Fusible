import type * as THREE from "three";

type Three = typeof import("three");
type Point = [number, number];

interface Part {
  size: [number, number, number];
  at: Point;
  color: number;
  metal?: boolean;
  lift?: number;
  glow?: boolean;
  caps?: boolean;
}

const W = 50;
const H = 32;
const T = 1.6;
const R = 2.5;
const PX = 24;
const MASK = "#16201c";
const COPPER = "#24372f";
const SILK = "#eef1ee";
const HOLES: Point[] = [
  [-21.5, -12.5],
  [21.5, -12.5],
  [-21.5, 12.5],
  [21.5, 12.5],
];

const PASSIVES: Point[] = [
  [-9, 5], [-9, 7.5], [1.5, -6], [3.8, -6], [10, 4], [10, 6.5], [-2, 9], [13.5, -4],
];

const PARTS: Part[] = [
  { size: [7, 0.9, 7], at: [-2, 0], color: 0x141414 },
  { size: [2.9, 1.1, 1.6], at: [9, -8], color: 0x1a1a1a },
  { size: [7.4, 3.2, 9], at: [-22, -1], color: 0xc9ced6, metal: true },
  { size: [3.2, 1.1, 1.6], at: [-12.5, -8], color: 0xe8dcc0, caps: true },
  { size: [3.2, 0.8, 2.5], at: [5, 9], color: 0xd6d9de, metal: true },
  { size: [2.5, 2.5, 15.24], at: [21, 0], color: 0x161616 },
  { size: [1.6, 0.6, 0.8], at: [15.5, -10], color: 0x39ff88, glow: true },
  ...PASSIVES.map((at, i): Part => ({ size: [1.6, 0.5, 0.8], at, color: i % 2 ? 0xb8966a : 0x1e1e1e, caps: true })),
];

const TRACES: { width: number; points: Point[] }[] = [
  { width: 0.8, points: [[-18.5, -3], [-16, -3], [-16, -8], [-14.1, -8]] },
  { width: 0.8, points: [[-10.9, -8], [-4, -8], [-2, -10], [7.5, -10], [7.5, -8.5]] },
  { width: 0.3, points: [[10.5, -8], [13.5, -8], [13.5, -5]] },
  { width: 0.3, points: [[1.5, 2], [5, 2], [5, 7.75]] },
  { width: 0.3, points: [[1.5, 0], [10, 0], [10, 3.2]] },
  { width: 0.3, points: [[1.5, -2], [12, -2], [14, -4], [19.75, -4]] },
  { width: 0.3, points: [[-2, 3.5], [-2, 8.2]] },
  { width: 0.3, points: [[-5.5, 1], [-9, 1], [-9, 4.2]] },
  { width: 0.3, points: [[14.3, -4], [14.3, -10], [14.7, -10]] },
  ...[-5.08, -2.54, 0, 2.54, 5.08].map((z) => ({
    width: 0.3,
    points: [[10.8, 6.5], [16, 6.5], [18, z + 2], [19.75, z + 2]] as Point[],
  })),
];

const SILK_TEXT: { text: string; at: Point; size: number }[] = [
  { text: "FUSIBLE", at: [6, 13], size: 2.6 },
  { text: "F1 2A", at: [-12.5, -11], size: 1.3 },
  { text: "U1", at: [-2, -5], size: 1.3 },
  { text: "J1", at: [-16.5, 5], size: 1.3 },
  { text: "REV A", at: [-12, 13], size: 1.2 },
];

/**
 * Maps a board point in millimetres to the silkscreen canvas.
 */
function px([x, z]: Point): Point {
  return [(x + W / 2) * PX, (z + H / 2) * PX];
}

/**
 * Paints the solder mask, the copper under it, the pads and the silkscreen on one canvas.
 */
function paintTop(): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.width = W * PX;
  canvas.height = H * PX;
  const ctx = canvas.getContext("2d");
  if (!ctx) return canvas;
  ctx.fillStyle = MASK;
  ctx.beginPath();
  ctx.roundRect(0, 0, canvas.width, canvas.height, R * PX);
  ctx.fill();
  ctx.strokeStyle = COPPER;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";
  for (const trace of TRACES) {
    ctx.lineWidth = trace.width * PX;
    ctx.beginPath();
    trace.points.forEach((point, i) => (i ? ctx.lineTo(...px(point)) : ctx.moveTo(...px(point))));
    ctx.stroke();
  }
  ctx.fillStyle = SILK;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  for (const label of SILK_TEXT) {
    ctx.font = `700 ${label.size * PX}px ui-monospace, monospace`;
    ctx.fillText(label.text, ...px(label.at));
  }
  ctx.strokeStyle = SILK;
  ctx.lineWidth = 0.15 * PX;
  ctx.strokeRect(...px([-6.2, -4.2]), 8.4 * PX, 8.4 * PX);
  ctx.globalCompositeOperation = "destination-out";
  for (const hole of HOLES) {
    ctx.beginPath();
    ctx.arc(...px(hole), 1.6 * PX, 0, Math.PI * 2);
    ctx.fill();
  }
  return canvas;
}

/**
 * Builds the board outline with its mounting holes, in millimetres.
 */
function outline(T3: Three): THREE.Shape {
  const shape = new T3.Shape();
  const x = -W / 2;
  const y = -H / 2;
  shape.moveTo(x + R, y);
  shape.lineTo(x + W - R, y);
  shape.quadraticCurveTo(x + W, y, x + W, y + R);
  shape.lineTo(x + W, y + H - R);
  shape.quadraticCurveTo(x + W, y + H, x + W - R, y + H);
  shape.lineTo(x + R, y + H);
  shape.quadraticCurveTo(x, y + H, x, y + H - R);
  shape.lineTo(x, y + R);
  shape.quadraticCurveTo(x, y, x + R, y);
  for (const [hx, hz] of HOLES) {
    const hole = new T3.Path();
    hole.absarc(hx, -hz, 1.6, 0, Math.PI * 2, true);
    shape.holes.push(hole);
  }
  return shape;
}

/**
 * Returns a material for a part body or a metal finish.
 */
function material(T3: Three, color: number, metal = false, glow = false) {
  return new T3.MeshStandardMaterial({
    color,
    metalness: metal ? 0.85 : 0.1,
    roughness: metal ? 0.28 : 0.7,
    emissive: glow ? color : 0x000000,
    emissiveIntensity: glow ? 1.4 : 0,
  });
}

/**
 * Adds one part, with tinned end caps for chip passives and the fuse.
 */
function addPart(T3: Three, group: THREE.Group, part: Part) {
  const [sx, sy, sz] = part.size;
  const body = new T3.Mesh(new T3.BoxGeometry(sx, sy, sz), material(T3, part.color, part.metal, part.glow));
  body.position.set(part.at[0], T + (part.lift ?? 0) + sy / 2, part.at[1]);
  group.add(body);
  if (!part.caps) return;
  const cap = material(T3, 0xd9dce0, true);
  for (const side of [-1, 1]) {
    const end = new T3.Mesh(new T3.BoxGeometry(sx * 0.18, sy * 1.04, sz * 1.04), cap);
    end.position.set(part.at[0] + side * sx * 0.41, T + sy / 2, part.at[1]);
    group.add(end);
  }
}

/**
 * Adds the gold header pins and the plated rings around the mounting holes.
 */
function addPlating(T3: Three, group: THREE.Group) {
  const gold = material(T3, 0xd4af37, true);
  for (let i = 0; i < 6; i++) {
    const pin = new T3.Mesh(new T3.BoxGeometry(0.64, 8.5, 0.64), gold);
    pin.position.set(21, T + 3.2, -6.35 + i * 2.54);
    group.add(pin);
  }
  for (const [x, z] of HOLES) {
    const ring = new T3.Mesh(new T3.RingGeometry(1.6, 2.6, 40), gold);
    ring.rotation.x = -Math.PI / 2;
    ring.position.set(x, T + 0.02, z);
    group.add(ring);
  }
}

/**
 * Builds a Fusible demo board: a matte mask, copper under it, gold plating, silkscreen and parts.
 */
export function buildBoard(T3: Three): THREE.Group {
  const group = new T3.Group();
  group.name = "board";
  const core = new T3.Mesh(
    new T3.ExtrudeGeometry(outline(T3), { depth: T, bevelEnabled: false, curveSegments: 12 }),
    new T3.MeshStandardMaterial({ color: 0x101814, roughness: 0.85, metalness: 0.05 }),
  );
  core.rotation.x = -Math.PI / 2;
  group.add(core);
  const texture = new T3.CanvasTexture(paintTop());
  texture.colorSpace = T3.SRGBColorSpace;
  texture.anisotropy = 4;
  const top = new T3.Mesh(
    new T3.PlaneGeometry(W, H),
    new T3.MeshStandardMaterial({ map: texture, transparent: true, alphaTest: 0.5, roughness: 0.62, metalness: 0.05 }),
  );
  top.rotation.x = -Math.PI / 2;
  top.position.y = T + 0.01;
  group.add(top);
  for (const part of PARTS) addPart(T3, group, part);
  addPlating(T3, group);
  return group;
}
