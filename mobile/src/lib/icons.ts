import { registerIcons, type IconData } from "@facile/muse";
import data from "./icons-data.json";

export const ICON = {
  phone: "solar:smartphone-2-bold-duotone",
  qr: "solar:qr-code-linear",
  scanner: "solar:scanner-linear",
  logo: "solar:cpu-bolt-bold-duotone",
  overview: "solar:home-2-linear",
  boards: "solar:cpu-linear",
  checks: "solar:shield-check-linear",
  bom: "solar:bill-list-linear",
  bodies: "solar:box-linear",
  forget: "solar:trash-bin-minimalistic-linear",
  link: "solar:link-round-linear",
  warn: "solar:danger-triangle-linear",
  ok: "solar:check-circle-linear",
  chevron: "mdi:chevron-right",
  back: "mdi:chevron-left",
  spinner: "solar:refresh-linear",
} as const;

/** Registers the Solar glyphs this app draws, so no icon is fetched at runtime. */
export function initIcons() {
  registerIcons(data as Record<string, IconData>);
}
