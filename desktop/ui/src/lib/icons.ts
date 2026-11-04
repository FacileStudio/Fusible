import { registerIcons, type IconData } from "@facile/muse";
import bundled from "./solar-icons.json";

/** Registers the Solar glyphs muse does not ship. */
export function registerSolar(): void {
  registerIcons(bundled as Record<string, IconData>);
}
