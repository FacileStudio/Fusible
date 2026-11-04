import { remember, remembered } from "./storage";
import type { Diagnostic, PaneName, ViewName } from "./types";

export const THEME_KEY = "fusible-theme";
export const SIDEBAR_KEY = "fusible.sidebarCollapsed";

declare global {
  interface Window {
    FUSIBLE_DESKTOP?: boolean;
  }
}

export const studio = $state({
  view: "schematic" as ViewName,
  lastView: "schematic" as ViewName,
  pane: "inspector" as PaneName,
  sidebarOpen: remembered(SIDEBAR_KEY) !== "1",
  sheetOpen: false,
  treeOpen: false,
  pairOpen: false,
  narrow: false,
  theme: remembered(THEME_KEY) === "light" ? "light" : "dark",
  desktop: Boolean(window.FUSIBLE_DESKTOP),
  hasAssembly: false,
  boardName: "",
  circuit: null as unknown,
  revision: null as string | null,
  fuseBlown: false,
  diagnostics: [] as Diagnostic[],
  status: "",
});

/** Switches the main view; the viewport reacts to the change. */
export function setView(view: ViewName): void {
  if (view === "assembly" && !studio.hasAssembly) return;
  if (view === "bom" && studio.view !== "bom") studio.lastView = studio.view;
  studio.view = view;
}

/** Closes the parts and costs page and returns to the view underneath it. */
export function leaveBom(): void {
  if (studio.view === "bom") studio.view = studio.lastView;
}

/** Opens or closes the sidebar and remembers the choice. */
export function setSidebar(open: boolean): void {
  studio.sidebarOpen = open;
  studio.sheetOpen = open;
  remember(SIDEBAR_KEY, open ? "0" : "1");
}

/** Opens the sidebar on a given pane. */
export function openPane(pane: PaneName): void {
  studio.pane = pane;
  setSidebar(true);
}

/** Shows a one-line status over the viewport; an empty message clears it. */
export function showStatus(message: string): void {
  studio.status = message;
}
