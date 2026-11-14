import type * as THREE from "three";

type Three = typeof import("three");

interface OverlayBox {
  size: [number, number, number];
  at: [number, number];
  lift?: number;
  color: number;
  metal?: boolean;
}

interface BoardOverlay {
  name: string;
  pcb: [number, number];
  rotation: number;
  boxes: OverlayBox[];
}

const BOARD_TOP_MM = 0.8;
const DEG = Math.PI / 180;

/**
 * Pastille parts that tscircuit exports without a 3D model, copied from Pastille's pcb-components.ts.
 * `pcb` and `rotation` are pcbX, pcbY and pcbRotation; `size` is [x, height, y] in millimetres.
 */
const BOARD_OVERLAYS: BoardOverlay[] = [
  {
    name: "U_ESP32_S3",
    pcb: [-3.47, 0],
    rotation: 0,
    boxes: [
      { size: [20.5, 0.8, 15.4], at: [-2.55, 0], color: 0x141414 },
      { size: [15.1, 1.6, 15.4], at: [0.15, 0], lift: 0.8, color: 0xd0d4dc, metal: true },
    ],
  },
  {
    name: "U_MIC",
    pcb: [10.8, -11.01],
    rotation: 180,
    boxes: [{ size: [2.65, 0.98, 3.5], at: [0, -0.15], color: 0xd4af37, metal: true }],
  },
  { name: "J_LCD", pcb: [0, 11.2], rotation: 0, boxes: [{ size: [8.6, 1.0, 3.8], at: [0, 0.1], color: 0x222225 }] },
  { name: "U_ESD", pcb: [7.2, -8.6], rotation: 90, boxes: [{ size: [2.9, 1.1, 1.6], at: [0, 0], color: 0x18181a }] },
];

/**
 * Builds one overlay part as a group placed in the GLB board frame (local x = -pcbX, z = pcbY).
 */
function overlayGroup(T: Three, part: BoardOverlay): THREE.Group {
  const group = new T.Group();
  group.name = part.name;
  group.position.set(-part.pcb[0], BOARD_TOP_MM, part.pcb[1]);
  group.rotation.y = part.rotation * DEG;
  for (const box of part.boxes) {
    const material = box.metal
      ? new T.MeshStandardMaterial({ color: box.color, metalness: 0.85, roughness: 0.25 })
      : new T.MeshStandardMaterial({ color: box.color, metalness: 0.1, roughness: 0.75 });
    const mesh = new T.Mesh(new T.BoxGeometry(...box.size), material);
    mesh.position.set(-box.at[0], (box.lift ?? 0) + box.size[1] / 2, box.at[1]);
    group.add(mesh);
  }
  return group;
}

/**
 * Adds the parts the tscircuit GLB lacks to its `board` node, skipping any part it already carries.
 */
export function enrichBoard(T: Three, root: THREE.Object3D) {
  const board = root.getObjectByName("board");
  if (!board) return;
  for (const part of BOARD_OVERLAYS) {
    if (!board.getObjectByName(part.name)) board.add(overlayGroup(T, part));
  }
}
