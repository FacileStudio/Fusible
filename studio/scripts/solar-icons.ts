import { writeFileSync } from "node:fs";
import solar from "@iconify-json/solar/icons.json";

const NAMES = `routing-2-linear cpu-linear box-linear box-minimalistic-linear layers-linear bill-list-linear
refresh-circle-linear shield-check-bold-duotone danger-triangle-bold-duotone cpu-bolt-bold-duotone sun-linear
moon-linear qr-code-linear sidebar-code-linear folder-linear play-linear hamburger-menu-linear
alt-arrow-left-linear magnifer-zoom-in-linear magnifer-zoom-out-linear restart-linear eye-linear
maximize-square-2-linear arrow-up-linear arrow-down-linear box-linear square-top-down-linear
add-circle-linear trash-bin-2-linear link-linear diskette-linear download-minimalistic-linear
check-circle-linear close-circle-linear plug-circle-linear share-circle-linear cursor-linear
play-circle-linear checklist-minimalistic-linear shield-check-linear refresh-linear
minimize-square-linear maximize-square-linear close-square-linear`.split(/\s+/);

const icons = solar.icons as Record<string, { body: string }>;
const aliases = (solar as { aliases?: Record<string, { parent: string }> }).aliases ?? {};

function bodyOf(name: string): string | undefined {
  return (icons[name] ?? icons[aliases[name]?.parent])?.body;
}

const out: Record<string, { body: string; width: number; height: number }> = {};
for (const name of NAMES) {
  const body = bodyOf(name);
  if (!body) throw new Error(`solar has no icon ${name}`);
  out[`solar:${name}`] = { body, width: 24, height: 24 };
}
writeFileSync(new URL("../src/lib/solar-icons.json", import.meta.url), JSON.stringify(out, null, 1));
