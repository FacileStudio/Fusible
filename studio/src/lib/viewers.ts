import type { CadCameraController } from "../cadCamera";
import { getCadController } from "../cadCamera";
import { Tscircuit } from "../tscircuit";
import type { ViewName } from "./types";

export type Hosts = Partial<Record<Exclude<ViewName, "bom">, HTMLElement>>;

const RENDERERS = {
  schematic: "renderSchematicViewer",
  pcb: "renderPcbViewer",
  "3d": "render3dViewer",
} as const;

let webgl: boolean | null = null;

/** True when the browser can create a WebGL context; the 3D views need one. */
export function hasWebgl(): boolean {
  if (webgl === null) {
    const canvas = document.createElement("canvas");
    const gl = canvas.getContext("webgl2") || canvas.getContext("webgl");
    webgl = Boolean(gl);
    gl?.getExtension("WEBGL_lose_context")?.loseContext();
  }
  return webgl;
}

/** Renders the viewer of `view` into its host; returns a status message when it cannot. */
export function renderView(view: ViewName, hosts: Hosts, circuit: unknown, onSelect: (ref: string) => void): string {
  if (view === "bom") return "";
  const host = hosts[view];
  if (!host) return "";
  if (["3d", "mechanical", "assembly"].includes(view) && !hasWebgl()) {
    return "WebGL is unavailable. Turn on hardware acceleration in your browser to use the 3D views.";
  }
  if (view === "mechanical") Tscircuit.renderMechanicalViewer(host);
  else if (view === "assembly") Tscircuit.renderAssemblyViewer(host);
  else if (circuit) Tscircuit[RENDERERS[view]](host, circuit, onSelect);
  return "";
}

/** Stops the bodies and assembly render loops when another view is on screen. */
export function parkViewers(view: ViewName): void {
  if (view !== "mechanical") Tscircuit.hideMechanicalViewer();
  if (view !== "assembly") Tscircuit.hideAssemblyViewer();
}

/** Moves the camera of the 3D, bodies or assembly view to a preset. */
export function cameraPreset(view: ViewName, hosts: Hosts, preset: "TOP" | "ISO" | "FRONT" | "RESET"): void {
  if (view === "mechanical") return Tscircuit.mechanicalCameraPreset(preset);
  if (view === "assembly") return Tscircuit.assemblyCameraPreset(preset);
  const controller: CadCameraController | null = getCadController();
  const host = hosts["3d"];
  if (controller && host) Tscircuit.animateCadCameraPreset(host, controller, preset);
}

/** Zooms a 2D viewer by dispatching a wheel event at its centre. */
export function zoomBy(host: HTMLElement | undefined, deltaY: number): void {
  if (!host) return;
  const rect = host.getBoundingClientRect();
  const target = host.querySelector("canvas") || host.querySelector("svg") || host;
  const init = { deltaY, clientX: rect.left + rect.width / 2, clientY: rect.top + rect.height / 2, bubbles: true };
  target.dispatchEvent(new WheelEvent("wheel", init));
}

/** Asks the viewers to measure themselves again after the layout changed. */
export function refit(): void {
  window.dispatchEvent(new Event("resize"));
}
