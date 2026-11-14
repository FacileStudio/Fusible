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

export const RELEASE_API = "https://api.github.com/repos/FacileStudio/Fusible/releases/latest";

export const ACTION_FALLBACK = "v2.5.0";

/** The `uses:` line of the GitHub Action pinned to a release tag. */
export function actionUses(version: string | null): string {
  return `uses: FacileStudio/Fusible@${version ?? ACTION_FALLBACK}`;
}

export const LOCALE_PREFIXES = { en: "", fr: "/fr", es: "/es", de: "/de" } as const;
export type SiteLocale = keyof typeof LOCALE_PREFIXES;
const DOCS_LOCALES: SiteLocale[] = ["en", "fr"];

/** The locales that ship a page: docs exist in English and French only. */
export function localesFor(page: string): SiteLocale[] {
  const all = Object.keys(LOCALE_PREFIXES) as SiteLocale[];
  return page === "/docs" ? DOCS_LOCALES : all;
}

/** Splits a pathname into its locale and the page under the locale prefix. */
export function splitPath(pathname: string): { locale: SiteLocale; page: string } {
  const match = /^\/(fr|es|de)(?=\/|$)/.exec(pathname);
  const page = (match ? pathname.slice(match[0].length) : pathname).replace(/\/$/, "");
  return { locale: (match?.[1] as SiteLocale | undefined) ?? "en", page };
}

/** The same page in another locale, or that locale's home when the page does not exist there. */
export function localeHref(locale: SiteLocale, pathname: string): string {
  const { page } = splitPath(pathname);
  const target = localesFor(page).includes(locale) ? page : "";
  return `${LOCALE_PREFIXES[locale]}${target}` || "/";
}
