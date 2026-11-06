const DARK = {
  bg: "#0d0d0d",
  surface: "#181818",
  silver: "#e6e6e6",
  green: "#88a888",
  olive: "#a8b365",
  text: "#ffffff",
  muted: "#bdbdbd",
  dim: "#8a8a8a",
  alert: "#c55454",
  line: "#2e2e2e",
  light: "#ffffff",
  "light-fill": "#90cdf4",
  "stage-ground": "#18181b",
  "grid-major": "#353535",
  edge: "#1f1f1f",
  "part-base": "#6f7d6b",
};

const LIGHT: typeof DARK = {
  ...DARK,
  bg: "#f7f7f7",
  surface: "#eeeeee",
  silver: "#3a3a3a",
  green: "#4d7a48",
  text: "#111111",
  muted: "#5a5a5a",
  dim: "#7a7a7a",
  alert: "#b03a3a",
  line: "#cfcfc6",
  "grid-major": "#d4d4cc",
  edge: "#e2e2db",
};

export type ThemeName = keyof typeof DARK;

/** Reads a canvas colour of the active theme; viewers draw to canvas and cannot take utility classes. */
export function themeColor(name: string): string {
  const light = document.documentElement.classList.contains("light");
  return (light ? LIGHT : DARK)[name as ThemeName];
}

/** Reads a canvas colour of the active theme as the number three.js expects. */
export function themeHex(name: string): number {
  return parseInt(themeColor(name).replace("#", ""), 16);
}
