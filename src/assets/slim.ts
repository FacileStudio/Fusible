import { FLOAT, readGlb, TRIANGLES, USHORT, writeGlb, type Gltf, type Mesh, type Primitive } from "./glb";
import { pack, type Merged } from "./slim_pack";

const UNSUPPORTED = ["images", "animations", "skins", "samplers", "textures"];

const read = (gltf: Gltf, bin: Uint8Array, id: number): Float32Array | Uint32Array => {
  const a = gltf.accessors[id];
  const view = gltf.bufferViews[a.bufferView];
  const start = bin.byteOffset + (view.byteOffset ?? 0) + (a.byteOffset ?? 0);
  const width = a.type === "VEC3" ? 3 : 1;
  if (a.componentType === FLOAT) return new Float32Array(bin.buffer.slice(start, start + a.count * width * 4));
  if (a.componentType === USHORT) {
    return Uint32Array.from(new Uint16Array(bin.buffer.slice(start, start + a.count * 2)));
  }
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
