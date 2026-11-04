export interface ReleaseAsset {
  name: string;
  url: string;
  size: number;
  checksum: string | null;
}

export interface Release {
  version: string | null;
  assets: Record<string, ReleaseAsset | undefined>;
}

export const EMPTY_RELEASE: Release = { version: null, assets: {} };

interface GithubAsset {
  name: string;
  browser_download_url: string;
  size: number;
}

/** Maps a release asset file name to the platform card it belongs to. */
export function platformOf(name: string): string | null {
  if (/^Fusible-Studio-.*\.(msi|exe)$/i.test(name)) return "windows";
  if (/^Fusible-Studio-.*\.dmg$/i.test(name)) return "macos";
  if (/^Fusible-Studio-.*\.(AppImage|deb|rpm)$/i.test(name)) return "linux";
  if (name === "Fusible-android.apk") return "android";
  return null;
}

/** Builds the page data from the GitHub latest-release payload. */
export function parseRelease(payload: { tag_name?: string; assets?: GithubAsset[] }): Release {
  const all = payload.assets ?? [];
  const urls = new Map(all.map((asset) => [asset.name, asset.browser_download_url]));
  const release: Release = { version: payload.tag_name ?? null, assets: {} };
  for (const asset of all) {
    const key = platformOf(asset.name);
    if (!key || release.assets[key]) continue;
    const shared = key === "android" ? null : (urls.get("desktop-checksums.txt") ?? null);
    const checksum = urls.get(`${asset.name}.sha256`) ?? shared;
    release.assets[key] = { name: asset.name, url: asset.browser_download_url, size: asset.size, checksum };
  }
  return release;
}
