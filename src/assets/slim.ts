import { concat, readGlb, writeGlb, type Gltf, type Mesh, type Primitive } from "./glb";

type Merged = { position: Float32Array; normal: Float32Array; index: Uint32Array; material?: number };

const FLOAT = 5126;
const UINT = 5125;
const USHORT = 5123;
const TRIANGLES = 4;
const UNSUPPORTED = ["images", "animations", "skins", "samplers", "textures"];

const read = (gltf: Gltf, bin: Uint8Array, id: number): Float32Array | Uint32Array => {
  const a = gltf.accessors[id];
  const view = gltf.bufferViews[a.bufferView];
  const start = bin.byteOffset + (view.byteOffset ?? 0) + (a.byteOffset ?? 0);
  const width = a.type === "VEC3" ? 3 : 1;
  if (a.componentType === FLOAT) return new Float32Array(bin.buffer.slice(start, start + a.count * width * 4));
  if (a.componentType === USHORT) return Uint32Array.from(new Uint16Array(bin.buffer.slice(start, start + a.count * 2)));
  return new Uint32Array(bin.buffer.slice(start, start + a.count * 4));
};

const mergeable = (p: Primitive): boolean => {
  const keys = Object.keys(p.attributes).sort().join();
  return keys === "NORMAL,POSITION" && p.indices !== undefined && (p.mode ?? TRIANGLES) === TRIANGLES;
};

const merge = (gltf: Gltf, bin: Uint8Array, prims: Primitive[]): Merged => {
  const parts = prims.map((p) => ({
    position: read(gltf, bin, p.attributes.POSITION) as Float32Array,
    normal: read(gltf, bin, p.attributes.NORMAL) as Float32Array,
    index: read(gltf, bin, p.indices!) as Uint32Array,
  }));
  const total = (key: "position" | "normal" | "index") => parts.reduce((n, p) => n + p[key].length, 0);
  const out = { position: new Float32Array(total("position")), normal: new Float32Array(total("normal")) };
  const index = new Uint32Array(total("index"));
  let [v, i] = [0, 0];
  for (const p of parts) {
    out.position.set(p.position, v);
    out.normal.set(p.normal, v);
    index.set(p.index.map((n) => n + v / 3), i);
    [v, i] = [v + p.position.length, i + p.index.length];
  }
  return { ...out, index, material: prims[0]?.material };
};

const bounds = (position: Float32Array) => {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  position.forEach((value, n) => {
    min[n % 3] = Math.min(min[n % 3], value);
    max[n % 3] = Math.max(max[n % 3], value);
  });
  return { min, max };
};

const pack = (gltf: Gltf, meshes: Merged[][]): Uint8Array => {
  const chunks: Uint8Array[] = [];
  let offset = 0;
  const add = (data: Float32Array | Uint32Array, accessor: object, target: number) => {
    gltf.bufferViews.push({ buffer: 0, byteOffset: offset, byteLength: data.byteLength, target });
    gltf.accessors.push({ ...accessor, bufferView: gltf.bufferViews.length - 1 } as Gltf["accessors"][number]);
    chunks.push(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
    offset += data.byteLength;
    return gltf.accessors.length - 1;
  };
  const primitive = (m: Merged): Primitive => {
    const count = m.position.length / 3;
    const POSITION = add(m.position, { componentType: FLOAT, count, type: "VEC3", ...bounds(m.position) }, 34962);
    const NORMAL = add(m.normal, { componentType: FLOAT, count, type: "VEC3" }, 34962);
    const indices = add(m.index, { componentType: UINT, count: m.index.length, type: "SCALAR" }, 34963);
    return { attributes: { POSITION, NORMAL }, indices, material: m.material, mode: TRIANGLES };
  };
  [gltf.accessors, gltf.bufferViews] = [[], []];
  gltf.meshes = gltf.meshes.map((mesh, i) => ({ ...mesh, primitives: meshes[i].map(primitive) }));
  gltf.buffers = [{ byteLength: offset }];
  return concat(chunks);
};

const byMaterial = (prims: Primitive[]): Primitive[][] => {
  const groups = new Map<number | undefined, Primitive[]>();
  for (const p of prims) groups.set(p.material, [...(groups.get(p.material) ?? []), p]);
  return [...groups.values()];
};

const slimMesh = (gltf: Gltf, bin: Uint8Array, mesh: Mesh): Merged[] =>
  byMaterial(mesh.primitives).map((group) => merge(gltf, bin, group));

/**
 * Merges the primitives that share a material in each mesh of a GLB. Geometry is unchanged.
 * A GLB with textures, animations, skins or other attributes than POSITION and NORMAL is returned as is.
 */
export const slimGlb = (glb: Uint8Array): Uint8Array => {
  const { gltf, bin } = readGlb(glb);
  const plain = (gltf.meshes ?? []).every((m) => m.primitives.every(mergeable));
  if (!plain || UNSUPPORTED.some((key) => key in gltf)) return glb;
  const merged = (gltf.meshes ?? []).map((mesh) => slimMesh(gltf, bin, mesh));
  return writeGlb(gltf, pack(gltf, merged));
};

/** Counts the primitives of a GLB, for tests. */
export const primitiveCount = (glb: Uint8Array): number =>
  readGlb(glb).gltf.meshes.reduce((n, m) => n + m.primitives.length, 0);
