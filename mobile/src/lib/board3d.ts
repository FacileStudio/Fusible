import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";

/** Converts Circuit JSON to a binary glTF with the same library the studio uses. */
export async function circuitToGlb(circuit: unknown): Promise<ArrayBuffer> {
  const { convertCircuitJsonToGltf } = await import("circuit-json-to-gltf");
  const out = await convertCircuitJsonToGltf(circuit as never, { format: "glb" });
  if (!(out instanceof ArrayBuffer)) throw new Error("not_glb");
  return out;
}

function frame(root: THREE.Object3D, camera: THREE.PerspectiveCamera, controls: OrbitControls) {
  const box = new THREE.Box3().setFromObject(root);
  const size = box.getSize(new THREE.Vector3()).length() || 1;
  const center = box.getCenter(new THREE.Vector3());
  camera.position.copy(center).add(new THREE.Vector3(size * 0.7, size * 0.7, size * 0.7));
  controls.target.copy(center);
  controls.update();
}

function lights(scene: THREE.Scene) {
  scene.add(new THREE.HemisphereLight(0xffffff, 0x444444, 2.2));
  const sun = new THREE.DirectionalLight(0xffffff, 2);
  sun.position.set(3, 5, 4);
  scene.add(sun);
}

/** Shows a GLB in `host` with orbit controls and returns a function that frees everything. */
export async function mountGlb(host: HTMLElement, glb: ArrayBuffer): Promise<() => void> {
  const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
  const camera = new THREE.PerspectiveCamera(45, 1, 0.01, 10000);
  const scene = new THREE.Scene();
  const controls = new OrbitControls(camera, renderer.domElement);
  lights(scene);
  const gltf = await new GLTFLoader().parseAsync(glb, "");
  scene.add(gltf.scene);
  frame(gltf.scene, camera, controls);
  host.appendChild(renderer.domElement);
  const fit = () => {
    const { clientWidth: w, clientHeight: h } = host;
    renderer.setSize(w, h);
    camera.aspect = w / Math.max(h, 1);
    camera.updateProjectionMatrix();
  };
  const watch = new ResizeObserver(fit);
  watch.observe(host);
  fit();
  renderer.setAnimationLoop(() => renderer.render(scene, camera));
  return () => {
    watch.disconnect();
    renderer.setAnimationLoop(null);
    controls.dispose();
    scene.traverse((o) => disposeObject(o));
    renderer.dispose();
    renderer.domElement.remove();
  };
}

function disposeObject(o: THREE.Object3D) {
  if (!(o instanceof THREE.Mesh)) return;
  o.geometry.dispose();
  const mats = Array.isArray(o.material) ? o.material : [o.material];
  mats.forEach((m: THREE.Material) => m.dispose());
}
