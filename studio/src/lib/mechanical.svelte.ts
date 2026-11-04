import { Tscircuit } from "../tscircuit";
import { studio } from "./studio.svelte";

export const mech = $state({ lid: true, explode: false, overlay: false, lidAvailable: true });

/** Pushes the overlay toggle to the 3D viewer; the overlay only draws over the 3D view. */
export function syncOverlay(host: HTMLElement | null): void {
  Tscircuit.setMechanicalOverlay(mech.overlay, studio.view === "3d", host);
}

/** Flips the enclosure overlay, the lid or the explode toggle. */
export function toggleMechanical(option: "lid" | "explode" | "overlay", host: HTMLElement | null): void {
  if (option === "overlay") {
    mech.overlay = !mech.overlay;
    syncOverlay(host);
  } else {
    mech[option] = Tscircuit.toggleMechanicalOption(option);
  }
}

/** Reads the saved toggles and follows whether the shown body has a lid; returns the unsubscribe function. */
export function initMechanical(host: HTMLElement | null): () => void {
  const saved = Tscircuit.getMechanicalOptions();
  Object.assign(mech, saved);
  syncOverlay(host);
  return Tscircuit.onLidControls((available) => {
    mech.lidAvailable = available;
  });
}
