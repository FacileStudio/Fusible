export const SITE_URL = "https://fusible.facile.studio";
export const REPO_URL = "https://github.com/FacileStudio/Fusible";
export const RELEASES_URL = `${REPO_URL}/releases/latest`;
export const CRATES_URL = "https://crates.io/crates/fusible";
export const PAGES = ["", "/download", "/docs"] as const;

/** The absolute URL of a page for one locale prefix (`""` or `"/fr"`). */
export function pageUrl(prefix: string, page: string): string {
  const path = `${prefix}${page}` || "/";
  return `${SITE_URL}${path}`;
}
