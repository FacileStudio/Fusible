import { registerIcons, type IconData } from "@facile/muse";
import data from "./icons-data.json";

export const LOGO = "solar:cpu-bolt-bold-duotone";
export const GITHUB = "solar:code-square-linear";
export const MENU = "solar:hamburger-menu-linear";

/** Registers the Solar glyphs the site draws that muse does not ship. */
export function registerSiteIcons(): void {
  registerIcons(data as Record<string, IconData>);
}
