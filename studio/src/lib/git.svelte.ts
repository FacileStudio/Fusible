import { ApiError, api } from "./api";
import type { GitStatus } from "./types";

const POLL_MS = 30000;
const CHECK_EVERY = 4;
const NOTICE_MS = 4000;

export const git = $state({
  status: null as GitStatus | null,
  busy: false,
  draft: "",
  notice: null as { text: string; details?: string; error?: boolean } | null,
});

let noticeTimer: ReturnType<typeof setTimeout> | undefined;

/** Reads the repository status; `check` also asks the remote for updates. */
export async function loadGit(check: boolean): Promise<void> {
  if (git.busy) return;
  try {
    git.status = await api.getGit(check);
  } catch (err) {
    console.warn("sync status unavailable", err);
  }
}

/** Runs a save, update or resolve action and shows its notice. */
export async function actGit(action: string, body?: unknown): Promise<void> {
  clearTimeout(noticeTimer);
  git.busy = true;
  git.notice = null;
  try {
    const reply = await api.postGit(action, body);
    if (reply.status) git.status = reply.status;
    git.notice = { text: reply.message ?? "" };
    if (action === "save") git.draft = "";
    noticeTimer = setTimeout(() => (git.notice = null), NOTICE_MS);
  } catch (err) {
    const failure = err instanceof ApiError ? err : null;
    git.notice = failure
      ? { text: failure.message || "That did not work.", details: failure.body.details, error: true }
      : { text: "Cannot reach Fusible Studio.", details: String(err), error: true };
  }
  git.busy = false;
}

/** Clears the notice and checks the remote again. */
export async function recheckGit(): Promise<void> {
  git.notice = null;
  await loadGit(true);
}

/** Starts polling the repository status; returns the stop function. */
export function startGitPolling(): () => void {
  let ticks = 0;
  void loadGit(true);
  const timer = setInterval(() => loadGit(++ticks % CHECK_EVERY === 0), POLL_MS);
  return () => clearInterval(timer);
}

/** Describes how long ago a unix time in seconds was. */
export function ago(seconds: number, now = Date.now() / 1000): string {
  const diff = Math.max(0, now - seconds);
  if (diff < 90) return "just now";
  const units: [number, string][] = [[86400, "day"], [3600, "hour"], [60, "minute"]];
  const [size, name] = units.find(([s]) => diff >= s) as [number, string];
  const n = Math.floor(diff / size);
  return `${n} ${name}${n === 1 ? "" : "s"} ago`;
}

/** One-line summary of what is waiting in the repository. */
export function summary(s: GitStatus): string {
  const stuck = s.conflicts.length;
  if (stuck) return `${stuck} file${stuck === 1 ? " needs" : "s need"} your decision.`;
  const parts: string[] = [];
  if (s.changes.length) parts.push(`${s.changes.length} change${s.changes.length === 1 ? "" : "s"} not saved`);
  if (s.has_remote && s.upstream && s.ahead) parts.push(`${s.ahead} save${s.ahead === 1 ? "" : "s"} not shared yet`);
  if (s.behind) parts.push(`${s.behind} update${s.behind === 1 ? "" : "s"} waiting`);
  if (parts.length) return `${parts.join(", ")}.`;
  return s.has_remote ? "Everything is saved and shared." : "Everything is saved.";
}
