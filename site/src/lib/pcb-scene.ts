import { createViewport, demandLoop, stepTransition, type Transition } from "$lib/pcb-viewport";
import { buildBoard } from "$lib/pcb-board";

export type Preset = "iso" | "top" | "side";

export interface SceneControl {
  applyPreset: (preset: Preset) => void;
  setRotating: (value: boolean) => void;
  setInteractive: (value: boolean) => void;
  dispose: () => void;
}

export interface SceneHooks {
  onUserInteract: () => void;
  onHidden: () => void;
}

type Vec3 = import("three").Vector3;

/**
 * Configures three-point studio lighting on a Three.js scene.
 */
function addLights(THREE: typeof import("three"), scene: import("three").Scene) {
  scene.add(new THREE.HemisphereLight(0xffffff, 0x181818, 1.6));
  const key = new THREE.DirectionalLight(0xffffff, 2.5);
  key.position.set(70, 130, 90);
  scene.add(key);
  const fill = new THREE.DirectionalLight(0xd4e2dc, 1.0);
  fill.position.set(-90, 70, -70);
  scene.add(fill);
  const rim = new THREE.DirectionalLight(0xffffff, 0.6);
  rim.position.set(0, -80, -60);
  scene.add(rim);
}

/**
 * Releases a material and every texture it holds.
 */
function disposeMaterial(material: unknown) {
  if (!material || typeof material !== "object") return;
  for (const val of Object.values(material as Record<string, unknown>)) {
    const tex = val as { isTexture?: boolean; dispose?: () => void };
    if (tex?.isTexture && tex.dispose) tex.dispose();
  }
  (material as { dispose?: () => void }).dispose?.();
}

/**
 * Releases geometry, material, and texture allocations for a Three.js object tree.
 */
function disposeObject(root: import("three").Object3D) {
  root.traverse((obj: unknown) => {
    const mesh = obj as { isMesh?: boolean; geometry?: { dispose: () => void }; material?: unknown };
    if (!mesh.isMesh) return;
    mesh.geometry?.dispose();
    (Array.isArray(mesh.material) ? mesh.material : [mesh.material]).forEach(disposeMaterial);
  });
}

/**
 * Mounts an interactive Three.js view of the demo board that renders only while something moves on screen.
 */
export async function mountPcbScene(
  host: HTMLDivElement,
  initialRotate: boolean,
  hooks: SceneHooks,
): Promise<SceneControl> {
  const [THREE, { OrbitControls }] = await Promise.all([
    import("three"),
    import("three/examples/jsm/controls/OrbitControls.js"),
  ]);

  const { renderer, camera, controls } = createViewport(THREE, OrbitControls, host, hooks.onUserInteract);
  const scene = new THREE.Scene();
  addLights(THREE, scene);

  const directions: Record<Preset, Vec3> = {
    iso: new THREE.Vector3(1, 0.92, 1).normalize(),
    top: new THREE.Vector3(0, 1, 0.0015).normalize(),
    side: new THREE.Vector3(1, 0.34, 0.08).normalize(),
  };

  let transition: Transition | null = null;

  function fit() {
    const { clientWidth: w, clientHeight: h } = host;
    if (!w || !h || Math.abs(camera.aspect - w / h) < 0.001) return;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }

  function setInteractive(on: boolean) {
    controls.enabled = on;
    renderer.domElement.style.touchAction = on ? "none" : "pan-y";
    renderer.domElement.style.cursor = on ? "grab" : "pointer";
  }

  const loop = demandLoop(
    renderer,
    host,
    () => {
      fit();
      if (transition && !stepTransition(transition, camera, controls.target)) transition = null;
      const moved = controls.update();
      renderer.render(scene, camera);
      return moved || controls.autoRotate || transition !== null;
    },
    () => {
      setInteractive(false);
      hooks.onHidden();
    },
  );
  controls.addEventListener("change", loop.wake);
  setInteractive(false);

  fit();
  const board = buildBoard(THREE);
  scene.add(board);
  const sphere = new THREE.Box3().setFromObject(board).getBoundingSphere(new THREE.Sphere());
  const center = sphere.center.clone();
  const radius = Math.max(sphere.radius, 1);
  camera.near = radius / 100;
  camera.far = radius * 100;
  camera.updateProjectionMatrix();
  const vertical = (camera.fov / 2) * (Math.PI / 180);
  const half = Math.min(vertical, Math.atan(Math.tan(vertical) * camera.aspect));
  const distance = (radius / Math.sin(half)) * (camera.aspect < 1 ? 0.95 : 0.8);
  controls.minDistance = distance * 0.35;
  controls.maxDistance = distance * 3;
  controls.target.copy(center);
  camera.position.copy(center).add(directions.iso.clone().multiplyScalar(distance));
  controls.update();
  controls.autoRotate = initialRotate;
  loop.wake();

  function applyPreset(next: Preset) {
    transition = {
      start: performance.now(),
      fromPos: camera.position.clone(),
      toPos: center.clone().add(directions[next].clone().multiplyScalar(distance)),
      fromTarget: controls.target.clone(),
      toTarget: center.clone(),
    };
    loop.wake();
  }

  return {
    applyPreset,
    setRotating: (val: boolean) => {
      controls.autoRotate = val;
      loop.wake();
    },
    setInteractive,
    dispose: () => {
      loop.dispose();
      controls.dispose();
      disposeObject(board);
      renderer.dispose();
      renderer.domElement.remove();
    },
  };
}
