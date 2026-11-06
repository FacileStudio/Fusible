import type { Theme } from "./types";

const KEY = "fusible-theme";

function resolve(theme: Theme): "dark" | "light" {
  if (theme !== "system") return theme;
  return window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
}

/** Puts the chosen theme on the root element and remembers it for the next cold start. */
export function applyTheme(theme: Theme): void {
  const root = document.documentElement;
  root.classList.remove("dark", "light");
  root.classList.add(resolve(theme));
  try {
    localStorage.setItem(KEY, theme);
  } catch {
    return;
  }
}
