import { EMPTY_RELEASE, parseRelease, type Release } from "./release";

const API = "https://api.github.com/repos/FacileStudio/Fusible/releases/latest";

/** Fetches the latest release at build time. Any failure returns an empty release. */
export async function fetchRelease(fetcher: typeof fetch): Promise<Release> {
  try {
    const headers: Record<string, string> = { Accept: "application/vnd.github+json" };
    const token = process.env.GITHUB_TOKEN;
    if (token) headers.Authorization = `Bearer ${token}`;
    const response = await fetcher(API, { headers });
    if (!response.ok) return EMPTY_RELEASE;
    return parseRelease(await response.json());
  } catch {
    return EMPTY_RELEASE;
  }
}
