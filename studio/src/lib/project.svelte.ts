import { Tscircuit } from "../tscircuit";
import { api } from "./api";
import { showFirmware } from "./firmware.svelte";
import { applyPayload } from "./payload.svelte";
import { boardOfItem, findItem, firstBoardName, initialItem, type Health } from "./projectModel";
import { resetSelection } from "./selection.svelte";
import { loadRuns } from "./runs.svelte";
import { leaveBom, setView, showStatus, studio } from "./studio.svelte";
import type { DevPayload, Project, ProjectItem } from "./types";

export const boards = new Map<string, DevPayload>();

export const project = $state({
  data: null as Project | null,
  board: null as string | null,
  health: {} as Record<string, Health>,
  item: null as string | null,
  compiling: true,
});

let boardRequest = 0;

/** The fuse health last seen for a board. */
export function healthOf(board: string): Health {
  return project.health[board] || { fuse_blown: false, violations: 0, known: false };
}

/** Shows a board: from the cache when the server already pushed it, else from /api/circuit. */
export function showBoard(name: string): void {
  project.board = name;
  resetSelection();
  const cached = boards.get(name);
  if (cached) {
    project.compiling = false;
    return applyPayload(cached);
  }
  project.compiling = true;
  const mine = ++boardRequest;
  api
    .getCircuit(name)
    .then((fresh) => {
      if (mine !== boardRequest || project.board !== name) return;
      if (fresh && (fresh.revision || fresh.board_name)) {
        project.compiling = false;
        applyPayload(fresh);
      }
    })
    .catch((err) => {
      if (mine !== boardRequest) return;
      project.compiling = false;
      showStatus(`Board ${name} unavailable: ${String(err.message || err)}`);
    });
}

function applyBody(kind: string, item: ProjectItem): void {
  const body = kind === "body";
  Tscircuit.setMechanicalBody(body ? item.name : null, body ? item.kind || "" : "", body ? item.board || "" : "");
}

function rememberItem(): void {
  try {
    const url = new URL(location.href);
    url.searchParams.set("item", project.item ?? "");
    history.replaceState(null, "", url);
  } catch (err) {
    console.warn("cannot record the selection in the URL", err);
  }
}

/** Opens a tree item: loads its board, shows its firmware or body and records it in the URL. */
export function selectItem(kind: string, name: string): void {
  const item = findItem(project.data, kind, name);
  if (!item) return;
  leaveBom();
  project.item = `${kind}:${name}`;
  const board = boardOfItem(project.data, kind, item);
  if (board !== project.board) showBoard(board);
  if (kind === "firmware") showFirmware(name);
  applyBody(kind, item);
  if (kind === "body") setView("mechanical");
  else if (kind === "board" && studio.view === "mechanical") setView("pcb");
  rememberItem();
}

/** Opens the parts and costs page from the tree. */
export function selectBom(): void {
  project.item = "bom:";
  setView("bom");
}

/** Remembers a board's fuse health for the tree dots. */
export function noteHealth(board: string, data: DevPayload): void {
  const violations = (data.diagnostics || []).length;
  project.health[board] = { fuse_blown: Boolean(data.fuse_blown), violations, known: true };
}

/** Loads the project and runs, then opens the board named in the URL or the first one. */
export async function initProject(): Promise<void> {
  try {
    project.data = await api.getProject();
    await loadRuns();
  } catch (err) {
    console.warn("project unavailable", err);
    return;
  }
  studio.hasAssembly = Boolean(project.data.assembly);
  project.board = firstBoardName(project.data);
  const [kind, name] = project.data.boards.length ? initialItem(project.data, location.search) : ["board", ""];
  project.item = `${kind}:${name}`;
  if (kind !== "board" || name !== project.board) selectItem(kind, name);
}
