import { remember } from "./storage";
import { setSidebar, studio, THEME_KEY } from "./studio.svelte";

/** Applies a theme to the document and remembers it. */
export function setTheme(theme: "dark" | "light"): void {
  studio.theme = theme;
  document.documentElement.classList.toggle("dark", theme === "dark");
  document.documentElement.classList.toggle("light", theme === "light");
  remember(THEME_KEY, theme);
}

/** Opens or closes the sidebar, or the bottom sheet on a narrow screen. */
export function toggleSidebar(): void {
  if (studio.narrow) studio.sheetOpen = !studio.sheetOpen;
  else setSidebar(!studio.sidebarOpen);
}

/** Opens or closes the project drawer. */
export function toggleTree(): void {
  studio.treeOpen = !studio.treeOpen;
}

/** Closes whichever overlay is open and says whether there was one. */
export function closeOverlay(): boolean {
  const open = studio.treeOpen || studio.pairOpen || (studio.narrow && studio.sheetOpen);
  studio.treeOpen = false;
  studio.pairOpen = false;
  if (studio.narrow) studio.sheetOpen = false;
  return open;
}

/** Tracks the narrow-screen breakpoint that turns the sidebar into a bottom sheet. */
export function watchNarrow(): () => void {
  const query = window.matchMedia("(max-width: 900px)");
  const update = () => {
    studio.narrow = query.matches;
  };
  update();
  query.addEventListener("change", update);
  return () => query.removeEventListener("change", update);
}
