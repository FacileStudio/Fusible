import data from "@iconify-json/solar/icons.json";

const names = [
  "smartphone-2-bold-duotone",
  "qr-code-linear",
  "scanner-linear",
  "cpu-bolt-bold-duotone",
  "cpu-bolt-linear",
  "home-2-linear",
  "shield-check-linear",
  "bill-list-linear",
  "box-linear",
  "cpu-linear",
  "trash-bin-minimalistic-linear",
  "link-round-linear",
  "danger-triangle-linear",
  "check-circle-linear",
  "layers-minimalistic-linear",
];

const out: Record<string, { body: string; width: number; height: number }> = {};
for (const name of names) {
  const icon = (data.icons as Record<string, { body: string }>)[name];
  if (!icon) throw new Error(`unknown solar icon ${name}`);
  out[`solar:${name}`] = { body: icon.body, width: data.width ?? 24, height: data.height ?? 24 };
}
await Bun.write("src/lib/icons-data.json", JSON.stringify(out, null, 1));
