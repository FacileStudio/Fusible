import { api } from "./api";
import { graph, selectNet, lookup, selection } from "./selection.svelte";
import { openPane } from "./studio.svelte";
import type { FirmwareData } from "./types";
import { runs } from "./runs.svelte";

export const firmware = $state({
  name: null as string | null,
  data: null as FirmwareData | null,
  error: null as string | null,
  picked: "",
});

let seq = 0;
let timer: ReturnType<typeof setTimeout> | undefined;

/** The latest build run that covers this firmware: its own, or a build of everything. */
export function lastBuild(name: string) {
  const own = runs.list.filter((run) => run.label === `build ${name}` || run.label === "build all");
  return own[own.length - 1];
}

async function load(name: string): Promise<void> {
  const mine = ++seq;
  let data: FirmwareData | null = null;
  let error: string | null = null;
  try {
    data = await api.getFirmware(name);
  } catch (err) {
    error = String((err as Error).message || err);
  }
  if (mine !== seq) return;
  firmware.data = data;
  firmware.error = error;
}

function reload(delay: number): void {
  clearTimeout(timer);
  if (firmware.name) timer = setTimeout(() => load(firmware.name as string), delay);
}

/** Shows the pin map of a firmware in the pins pane and loads it. */
export function showFirmware(name: string): void {
  if (name !== firmware.name) Object.assign(firmware, { name, data: null, error: null });
  openPane("pins");
  void load(name);
}

/** Reloads the pin map when its board was rebuilt. */
export function onBoard(board: string): void {
  if (firmware.data?.board === board) reload(300);
}

/** Reloads the pin map when a run settles. */
export function onRun(status: string | null): void {
  if (status && status !== "running") reload(0);
}

/** Selects a pin row: highlights its net and, if the net is not in the viewer, inspects the remote part. */
export function selectPin(key: string, net: string, part: string): void {
  firmware.picked = key;
  if (net) selectNet(net);
  if (part && !graph.nets[net]) {
    const comp = lookup(part);
    if (comp) selection.pinned = comp.name;
  }
}
