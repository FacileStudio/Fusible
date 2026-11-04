import { api } from "./api";
import type { KicadBoard } from "./types";

export const kicad = $state({ boards: [] as KicadBoard[], open: false });

/** Replaces the KiCad comparison result, as a sync event or a fresh read delivers it. */
export function applyKicad(boards: KicadBoard[]): void {
  kicad.boards = boards;
}

/** Reads the KiCad comparison once at start. */
export async function loadKicad(): Promise<void> {
  try {
    applyKicad(await api.getKicad());
  } catch (err) {
    console.warn("cannot read the KiCad status", err);
  }
}

/** The state and label of the KiCad chip. */
export function kicadSummary(boards: KicadBoard[]): { kind: "error" | "in_sync" | "differs"; text: string } {
  if (boards.some((board) => board.state === "error")) return { kind: "error", text: "KiCad: error" };
  const count = boards.reduce((sum, board) => sum + board.differences.length, 0);
  if (count === 0) return { kind: "in_sync", text: "KiCad: in sync" };
  return { kind: "differs", text: `KiCad: ${count} ${count === 1 ? "difference" : "differences"}` };
}

/** The lines the KiCad panel lists, prefixed by board name when there are several. */
export function kicadLines(boards: KicadBoard[]): string[] {
  const several = boards.length > 1;
  return boards.flatMap((board) => {
    const prefix = several && board.board ? `${board.board}: ` : "";
    if (board.state === "error") return [`${prefix}${board.message}`];
    return board.differences.map((line) => prefix + line);
  });
}
