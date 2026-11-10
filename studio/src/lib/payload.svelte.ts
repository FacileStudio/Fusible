import { Tscircuit } from "../tscircuit";
import { loadGraph } from "./selection.svelte";
import { setCircuit, studio } from "./studio.svelte";
import type { DevPayload } from "./types";

let lastBoard = "";

/** Re-sends the last known revision to the mechanical viewer, once it is ready. */
export function resendRevision(): void {
  if (studio.revision !== null) Tscircuit.setMechanicalRevision(studio.revision, lastBoard);
}

/** Applies a dev payload to the studio: board name, revision, circuit index, fuse state, diagnostics and waivers. */
export function applyPayload(data: DevPayload | null): void {
  if (!data) return;
  if (data.board_name) studio.boardName = data.board_name;
  if (typeof data.revision === "string") {
    studio.revision = data.revision;
    lastBoard = data.board || "";
    studio.board = lastBoard;
    resendRevision();
  }
  if (data.circuit_json) {
    setCircuit(data.circuit_json);
    loadGraph(data.circuit_json);
  }
  studio.fuseBlown = Boolean(data.fuse_blown);
  studio.diagnostics = data.diagnostics || [];
  studio.waived = data.waived || [];
}
