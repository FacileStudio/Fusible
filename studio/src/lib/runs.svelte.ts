import { api } from "./api";
import { openPane } from "./studio.svelte";
import type { Run } from "./types";

const MAX_LINES = 2000;
const MAX_ORPHAN_RUNS = 8;

export const runs = $state({
  list: [] as Run[],
  active: null as number | null,
  error: "",
});

const orphans: Record<number, string[]> = {};

/** True while any run is still going; buttons that start runs disable themselves on it. */
export function isBusy(): boolean {
  return runs.list.some((run) => run.status === "running");
}

function adopt(run: Run): Run {
  runs.list.push(run);
  const adopted = runs.list[runs.list.length - 1];
  (orphans[run.id] || []).forEach((line) => addLine(adopted, line));
  delete orphans[run.id];
  return adopted;
}

function knownRun(id: number, label?: string): Run | null {
  const found = runs.list.find((run) => run.id === id);
  if (found) return found;
  return label ? adopt({ id, label, status: "running", lines: [] }) : null;
}

function addLine(run: Run, line: string): void {
  if (run.lines.length < MAX_LINES) run.lines.push(line);
}

/** Records a log line; lines that arrive before their run is known wait in a small buffer. */
export function appendLog(id: number, line: string): void {
  const run = runs.list.find((r) => r.id === id);
  if (run) return addLine(run, line);
  const ids = Object.keys(orphans);
  if (!(id in orphans) && ids.length >= MAX_ORPHAN_RUNS) delete orphans[Number(ids[0])];
  const pending = (orphans[id] = orphans[id] || []);
  if (pending.length < MAX_LINES) pending.push(line);
}

/** Applies a run status event and returns the run's new status. */
export function applyRunEvent(data: { run: number; label?: string; status?: Run["status"] }): Run["status"] | null {
  const run = knownRun(data.run, data.label);
  if (!run) return null;
  if (data.label) run.label = data.label;
  if (data.status) run.status = data.status;
  return run.status;
}

/** Starts a run on the server and makes it the active one; returns the new status or null on failure. */
export async function startRun(action: string, target?: string): Promise<Run["status"] | null> {
  openPane("runs");
  const label = target ? `${action} ${target}` : `${action} all`;
  try {
    const { run: id } = await api.startRun(action, target);
    runs.error = "";
    const run = knownRun(id, label);
    if (!run) return null;
    run.label = label;
    runs.active = run.id;
    return run.status;
  } catch (err) {
    runs.error = String((err as Error).message || err);
    return null;
  }
}

/** Loads the run history the server keeps and selects the latest. */
export async function loadRuns(): Promise<void> {
  runs.list = await api.getRuns();
  runs.active = runs.list.length ? runs.list[runs.list.length - 1].id : null;
}
