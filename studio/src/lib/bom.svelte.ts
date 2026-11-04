import { api } from "./api";
import type { BomData, BomFile, BomLine } from "./types";

export const bom = $state({ data: null as BomData | null, error: "", saving: false });

let seq = 0;
let timer: ReturnType<typeof setTimeout> | undefined;

/** Turns a free-text part name into the stable id the server stores it under. */
export function slug(name: string): string {
  return name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}

/** Loads the parts and costs report. */
export async function loadBom(): Promise<void> {
  bom.data = null;
  try {
    bom.data = await api.getBom();
    bom.error = "";
  } catch (err) {
    bom.error = String((err as Error).message || err);
  }
}

function prune(file: BomFile, lines: BomLine[]): void {
  const kinds = Object.fromEntries(lines.map((l) => [l.id, l.kind]));
  for (const [id, entry] of Object.entries(file.parts)) {
    if (kinds[id] === "component" && !Object.keys(entry).length) delete file.parts[id];
  }
  for (const [id, entry] of Object.entries(file.prints)) {
    if (!Object.keys(entry).length) delete file.prints[id];
  }
}

/** Saves the edited file and adopts the recomputed report; a stale reply is dropped. */
export async function saveBom(): Promise<void> {
  if (!bom.data) return;
  const mine = ++seq;
  prune(bom.data.file, bom.data.report.lines);
  try {
    const reply = await api.saveBom($state.snapshot(bom.data.file) as BomFile);
    if (mine !== seq) return;
    bom.error = "";
    bom.data = reply;
  } catch (err) {
    bom.error = String((err as Error).message || err);
  }
}

/** Saves after a short pause so typing a number does not post every key. */
export function saveLater(): void {
  clearTimeout(timer);
  timer = setTimeout(saveBom, 300);
}

/** Records a field edit of a line in the file, dropping the key when the value is back to the default. */
export function editLine(line: BomLine, field: string, value: unknown): void {
  if (!bom.data) return;
  const isPrint = line.kind === "print";
  const bag = isPrint ? bom.data.file.prints : bom.data.file.parts;
  const key = isPrint ? line.id.slice("print-".length) : line.id;
  const entry = (bag[key] = bag[key] || {});
  const unchanged = field === "quantity" && line.kind === "component" && value === line.designators.length;
  if (value === undefined || value === "" || unchanged) delete entry[field];
  else entry[field] = value;
}

/** Adds a manual part and saves; reports a duplicate id as an error. */
export async function addPart(name: string): Promise<void> {
  const id = slug(name);
  if (!id || !bom.data) return;
  if (bom.data.report.lines.some((l) => l.id === id)) {
    bom.error = `"${id}" is already in the list`;
    return;
  }
  bom.data.file.parts[id] = { description: name.trim(), quantity: 1 };
  await saveBom();
}

/** Removes a manual part and saves. */
export async function removePart(id: string): Promise<void> {
  if (!bom.data) return;
  delete bom.data.file.parts[id];
  await saveBom();
}
