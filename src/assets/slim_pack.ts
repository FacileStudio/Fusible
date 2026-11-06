import { concat, FLOAT, TRIANGLES, UINT, type Gltf, type Primitive } from "./glb";

export type Merged = { position: Float32Array; normal: Float32Array; index: Uint32Array; material?: number };

const bounds = (position: Float32Array) => {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  position.forEach((value, n) => {
    min[n % 3] = Math.min(min[n % 3], value);
    max[n % 3] = Math.max(max[n % 3], value);
  });
  return { min, max };
};

export const pack = (gltf: Gltf, meshes: Merged[][]): Uint8Array => {
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
