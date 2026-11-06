import { registerIcons, type IconData } from "@facile/muse";
import solar from "./solar-icons.json";

/** Registers the Solar glyphs the studio draws that muse does not bundle; call once before the first render. */
export function registerStudioIcons(): void {
  registerIcons(solar as Record<string, IconData>);
}
