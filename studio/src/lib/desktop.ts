export const LABEL = "main";
export const MAC_PILL = 100;

export interface WindowButton {
  name: "close" | "min" | "max";
  label: string;
  icon: string;
  cmd: string;
}

export const WINDOW_BUTTONS: WindowButton[] = [
  { name: "min", label: "Minimize", icon: "solar:minimize-square-linear", cmd: "plugin:window|minimize" },
  { name: "max", label: "Maximize", icon: "solar:maximize-square-linear", cmd: "plugin:window|toggle_maximize" },
  { name: "close", label: "Close", icon: "solar:close-square-linear", cmd: "plugin:window|close" },
];

/** True on macOS, where the system draws the traffic lights in the reserved pill. */
export function isMac(): boolean {
  return /Mac|iPhone|iPad/.test(navigator.platform) || /Mac OS X/.test(navigator.userAgent);
}

/** Calls a Tauri command; outside the desktop shell it resolves to nothing. */
export function invoke(cmd: string, args?: Record<string, unknown>): Promise<unknown> {
  const tauri = (window as unknown as { __TAURI_INTERNALS__?: { invoke: typeof invoke } }).__TAURI_INTERNALS__;
  return tauri ? tauri.invoke(cmd, args) : Promise.resolve();
}

/** Goes back to the projects window. */
export function openProjects(): Promise<unknown> {
  return invoke("open_projects", {});
}

const PLAIN: Record<string, () => Promise<unknown>> = {
  n: () => invoke("open_projects", { fragment: "new" }),
  o: () => invoke("open_projects", { fragment: "open" }),
  ",": () => invoke("open_projects", { fragment: "settings" }),
  q: () => invoke("plugin:window|close", { label: LABEL }),
};

/** Binds the window shortcuts that macOS gets from its menu to Ctrl on other platforms; returns the unbind. */
export function installDesktopKeys(): () => void {
  if (isMac()) return () => undefined;
  const onKey = (event: KeyboardEvent) => {
    const target = event.target as Element | null;
    if (event.defaultPrevented || target?.closest?.("input, textarea, select, [contenteditable]")) return;
    if (!(event.ctrlKey || event.metaKey) || event.altKey) return;
    const key = event.key.toLowerCase();
    const run = event.shiftKey ? (key === "o" ? openProjects : undefined) : PLAIN[key];
    if (!run) return;
    event.preventDefault();
    void run();
  };
  document.addEventListener("keydown", onKey);
  return () => document.removeEventListener("keydown", onKey);
}
