import type { Project, ProjectItem } from "./types";

export type ItemKind = "board" | "firmware" | "body";
export type Health = { fuse_blown: boolean; violations: number; known: boolean };

export const GROUPS: { list: keyof Project; kind: ItemKind; title: string; action: string; hint: string }[] = [
  { list: "boards", kind: "board", title: "Boards", action: "check", hint: "Check this board" },
  { list: "firmware", kind: "firmware", title: "Firmware", action: "build", hint: "Build this firmware" },
  { list: "bodies", kind: "body", title: "Bodies", action: "body", hint: "Build this body" },
];

/** The items of one kind in the project. */
export function itemsOf(project: Project | null, kind: string): ProjectItem[] {
  const group = GROUPS.find((g) => g.kind === kind);
  return (group && project && (project[group.list] as ProjectItem[])) || [];
}

/** Finds an item by kind and name. */
export function findItem(project: Project | null, kind: string, name: string): ProjectItem | undefined {
  return itemsOf(project, kind).find((item) => item.name === name);
}

/** The name of the first board, or an empty string. */
export function firstBoardName(project: Project | null): string {
  return project?.boards[0]?.name || "";
}

/** The board an item belongs to: itself for a board, else its declared board, else the first board. */
export function boardOfItem(project: Project | null, kind: string, item: ProjectItem): string {
  return kind === "board" ? item.name : item.board || firstBoardName(project);
}

/** Reads the item to open from the `item` query parameter, falling back to the first board. */
export function initialItem(project: Project, search: string): [string, string] {
  const wanted = new URLSearchParams(search).get("item") || "";
  const [kind, ...rest] = wanted.split(":");
  const name = rest.join(":");
  return findItem(project, kind, name) ? [kind, name] : ["board", firstBoardName(project)];
}

/** The second line of a tree row: fuse state for boards, target board for firmware, origin for bodies. */
export function subLabel(kind: string, item: ProjectItem, health: Health): string {
  if (kind === "board") {
    const n = health.violations;
    return health.fuse_blown ? `${n} violation${n === 1 ? "" : "s"}` : "Fuse intact";
  }
  if (kind === "firmware") return `to ${item.board || "board"}`;
  if (item.kind === "authored") return item.exists === false ? "Missing" : "Authored";
  return "Generated";
}
