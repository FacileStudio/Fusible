import type { ViewName } from "./types";

export const VIEW_KEYS: Record<string, ViewName> = {
  "1": "schematic",
  "2": "pcb",
  "3": "3d",
  "4": "mechanical",
  "5": "assembly",
};

export interface KeyActions {
  setView: (view: ViewName) => void;
  toggleSidebar: () => void;
  toggleTree: () => void;
  closeOverlay: () => boolean;
  hasAssembly: () => boolean;
  hasProject: () => boolean;
}

/** True when the key press comes from a text field, where shortcuts must stay quiet. */
export function isTyping(target: EventTarget | null): boolean {
  const node = target as { closest?: (selector: string) => Element | null } | null;
  return Boolean(node?.closest?.("input, textarea, select, [contenteditable]"));
}

/** Maps a key press to an action: 1 to 5 switch views, backslash the panel, [ the tree, Escape the overlay. */
export function handleKey(event: KeyboardEvent, actions: KeyActions): void {
  if (event.ctrlKey || event.metaKey || event.altKey) return;
  if (event.key === "Escape") {
    actions.closeOverlay();
    return;
  }
  if (isTyping(event.target)) return;
  const view = VIEW_KEYS[event.key];
  if (view) {
    if (view !== "assembly" || actions.hasAssembly()) actions.setView(view);
  } else if (event.key === "\\") actions.toggleSidebar();
  else if (event.key === "[" && actions.hasProject()) actions.toggleTree();
}
