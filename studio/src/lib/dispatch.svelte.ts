import { Tscircuit } from "../tscircuit";
import { api } from "./api";
import { onBoard, onRun } from "./firmware.svelte";
import { applyKicad } from "./kicad.svelte";
import { applyPayload } from "./payload.svelte";
import { boards, noteHealth, project } from "./project.svelte";
import { appendLog, applyRunEvent, startRun } from "./runs.svelte";
import { setView, studio } from "./studio.svelte";
import type { DevPayload, KicadBoard, Run } from "./types";

type Message = Record<string, unknown> & { event?: string };

async function syncAssembly(): Promise<void> {
  try {
    const data = await api.getProject();
    studio.hasAssembly = Boolean(data.assembly);
    if (!studio.hasAssembly && studio.view === "assembly") setView("schematic");
  } catch (err) {
    console.warn("cannot read the project", err);
  }
}

function afterAssemblyRun(data: Message): void {
  const label = String(data.label || "");
  if (data.event !== "run" || data.status === "running" || !label.startsWith("assembly")) return;
  void syncAssembly();
  Tscircuit.refreshAssembly();
}

function onPayload(data: DevPayload): void {
  const board = data.board || "";
  boards.set(board, data);
  noteHealth(board, data);
  onBoard(board);
  if (project.board === null) project.board = board;
  if (board === project.board) applyPayload(data);
}

/** Routes one server event: log lines, run status, KiCad sync, or a board payload. */
export function handleMessage(data: Message): void {
  afterAssemblyRun(data);
  if (data.event === "log") appendLog(data.run as number, data.line as string);
  else if (data.event === "run") onRun(applyRunEvent(data as { run: number; label?: string; status?: Run["status"] }));
  else if (data.event === "sync") applyKicad((data.boards as KicadBoard[]) || []);
  else onPayload(data as DevPayload);
}

/** Starts a run and lets the firmware pane know how it went. */
export async function launch(action: string, target?: string): Promise<void> {
  onRun(await startRun(action, target));
}
