/** The GitHub release page of a version. */
export function releaseUrl(version: string): string {
  return `https://github.com/FacileStudio/Fusible/releases/tag/v${version}`;
}

/** Whole percent downloaded, or null while the total is unknown. */
export function percentOf(downloaded: number, total: number | null): number | null {
  if (!total || total <= 0) return null;
  return Math.min(100, Math.floor((downloaded / total) * 100));
}

/** "Checked 3 hours ago" for a Unix-seconds timestamp, or "Never checked". */
export function checkedLabel(seconds: number | null, now: number = Date.now()): string {
  if (seconds === null) return "Never checked";
  const minutes = Math.floor(Math.max(0, now - seconds * 1000) / 60000);
  if (minutes < 1) return "Checked just now";
  if (minutes < 60) return `Checked ${minutes} minute${minutes > 1 ? "s" : ""} ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 48) return `Checked ${hours} hour${hours > 1 ? "s" : ""} ago`;
  return `Checked ${Math.floor(hours / 24)} days ago`;
}
