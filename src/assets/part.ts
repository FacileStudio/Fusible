import { writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { OcctKernel, type ShapeHandle } from "occt-wasm";
import { toBoard, type Kit } from "./part_board";
import { makeCad } from "./part_cad";
import { block, type Env, type Feature } from "./part_hw";
import { slimGlb } from "./slim";

type Input = {
  source: string;
  kit: Kit | null;
  material: string;
  process: string;
  gap: number;
  outputs: { format: "step" | "stl" | "glb" | "brep"; path: string }[];
};

const STEP_DATE = "2000-01-01T00:00:00";
const MIN_VOLUME = 0.01;

const stableStep = (text: string): string => text.replace(/(FILE_NAME\s*\(\s*'[^']*'\s*,\s*')[^']*'/, `$1${STEP_DATE}'`);

const interferences = (k: OcctKernel, shape: ShapeHandle, kit: Kit | null, gap: number): Feature[] =>
  (kit?.keep_out ?? []).flatMap((c) => {
    const min = [c.x - c.w / 2 - gap, c.y - c.h / 2 - gap, c.z0 - gap];
    const max = [c.x + c.w / 2 + gap, c.y + c.h / 2 + gap, c.z1 + gap];
    const volume = k.getVolume(k.common(shape, block(k, min, max)));
    const at: [number, number, number] = [c.x, c.y, c.z0];
    return volume > MIN_VOLUME ? [{ kind: "interference", component: c.component, gap, volume, at }] : [];
  });

const writers = (k: OcctKernel, shape: ShapeHandle) => ({
  step: () => stableStep(k.exportStep(shape)),
  stl: () => k.exportStl(shape, 0.05),
  brep: () => k.toBREP(shape),
  glb: () => {
    const doc = k.createXCAFDocument();
    doc.addShape(shape, { name: "part", color: [0.18, 0.2, 0.23] });
    const bytes = slimGlb(doc.exportGLTF());
    doc.close();
    return bytes;
  },
});

const run = async (input: Input) => {
  const k = await OcctKernel.init();
  const board = toBoard(input.kit);
  const env: Env = { k, rec: { features: [], floorTop: 0 }, board };
  const ctx = { board, material: input.material, process: input.process, cad: makeCad(env) };
  const shape = (await import(pathToFileURL(input.source).href)).default(ctx) as ShapeHandle;
  const write = writers(k, shape);
  for (const { format, path } of input.outputs) writeFileSync(path, write[format]());
  const b = k.getBoundingBox(shape);
  const features = [...env.rec.features, ...interferences(k, shape, input.kit, input.gap)];
  const bbox = [b.xmax - b.xmin, b.ymax - b.ymin, b.zmax - b.zmin];
  console.log(JSON.stringify({ features, bbox, volume_mm3: k.getVolume(shape) }));
};

await run(JSON.parse(await Bun.stdin.text()));
