/**
 * Types of a Fusible code part. `fusible body new` copies this file to `mech/fusible.d.ts`.
 *
 * A part is a module whose default export takes a `Ctx` and returns a `Shape`.
 * Units are millimetres, Z up. The frame is the board's: X and Y are the PCB coordinates and
 * Z = 0 is the middle of the board, so its top face is at `board.thickness / 2`.
 *
 * Use the helpers (`shell`, `boss`, `standoff`, `ventSlots`, `cutout`, `draft`) to get rule
 * coverage. They record a feature that `fusible check` judges by process. Raw `box`,
 * `extrude` and `cut` record nothing: only the geometric component clearance sees them.
 */
export type Process = "fdm" | "sla" | "mjf" | "injection" | "cnc";

export type Board = {
  outline: [number, number][];
  thickness: number;
  /** Non-plated holes only: the mounting holes. */
  holes: { x: number; y: number; d: number }[];
  /** Wall openings a connector needs. `z` is the bottom of the opening, `w` runs along the board edge. */
  openings: { name: string; x: number; y: number; z: number; w: number; h: number; side: string }[];
  /** Components with a 3D model height. `top` and `bottom` are Z levels in the board frame. */
  components: { name: string; x: number; y: number; top: number; bottom: number }[];
};

export type Ctx = {
  board: Board | null;
  material: string;
  process: Process;
  cad: Cad;
};

export type Shape = { readonly __shape: unique symbol };
export type Opening = { x: number; y: number; z: number; w: number; h: number; side: string };

export type Cad = {
  /** A box with one corner at the origin. */
  box(w: number, d: number, h: number): Shape;
  /** A cylinder standing on the origin along Z. */
  cylinder(r: number, h: number): Shape;
  /** A polygon extruded along Z from z = 0. */
  extrude(points: [number, number][], h: number): Shape;
  union(...s: Shape[]): Shape;
  cut(a: Shape, ...b: Shape[]): Shape;
  intersect(a: Shape, b: Shape): Shape;
  move(s: Shape, x: number, y: number, z: number): Shape;
  /** Rotates around an axis through the origin. */
  rotate(s: Shape, axis: "x" | "y" | "z", deg: number): Shape;
  /** Rounds every edge of the shape. */
  fillet(s: Shape, r: number): Shape;
  /**
   * A tray around the outline: walls `wall` thick, `h` tall, on a floor `floor` thick.
   * The inner face sits 0.5 mm off the board edge. The floor top is 5 mm under the board.
   * Records the features "wall" (walls and floor).
   */
  shell(outline: [number, number][], wall: number, h: number, floor: number): Shape;
  /**
   * A boss that rises `h` from the floor top, with a blind bore. An `insert` bore is the
   * typical heat-set insert size (check your insert's datasheet); a `screw` bore is a
   * self-tap pilot. Records the feature "boss".
   */
  boss(x: number, y: number, opts: { insert?: "m2" | "m2.5" | "m3"; screw?: "m2" | "m2.5" | "m3"; h: number }): Shape;
  /** A plain post with a through hole. Records the feature "standoff". */
  standoff(x: number, y: number, h: number, hole: number): Shape;
  /**
   * A cutter for `n` slots, `w` wide along `dir` and `l` long across it, `pitch` apart.
   * The cutter goes through everything along Z. Records the feature "vent".
   */
  ventSlots(x: number, y: number, n: number, w: number, l: number, pitch: number, dir: "x" | "y"): Shape;
  /** A cutter through the wall in front of a connector, `margin` bigger on every side (default 1). Records "port". */
  cutout(opening: Opening, margin?: number): Shape;
  /** Declares the draft angle of the part, in degrees. It builds no geometry. Records "draft". */
  draft(deg: number): void;
};
