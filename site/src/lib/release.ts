import type { DownloadEntry } from "@facile/muse";

type Platform = DownloadEntry["platform"];

export interface Release {
  version: string | null;
  downloads: DownloadEntry[];
}

export const EMPTY_RELEASE: Release = { version: null, downloads: [] };

interface GithubAsset {
  name: string;
  browser_download_url: string;
  size: number;
}

/** Maps a release asset file name to the platform it installs on. */
export function platformOf(name: string): Platform | null {
  if (/^Fusible-Studio-.*\.(msi|exe)$/i.test(name)) return "windows";
  if (/^Fusible-Studio-.*\.dmg$/i.test(name)) return "macos";
  if (/^Fusible-Studio-.*\.(AppImage|deb|rpm)$/i.test(name)) return "linux";
  if (name === "Fusible-android.apk") return "android";
  return null;
}

/** Reads the CPU architecture from an installer name, such as `x64` or `universal`. */
function archOf(name: string): string | undefined {
  return /-(x64|arm64|aarch64|universal)[-.]/i.exec(name)?.[1];
}

/** Builds the download entries from the GitHub latest-release payload. */
export function parseRelease(payload: { tag_name?: string; assets?: GithubAsset[] }): Release {
  const all = payload.assets ?? [];
  const urls = new Map(all.map((asset) => [asset.name, asset.browser_download_url]));
  const version = payload.tag_name ?? null;
  const downloads: DownloadEntry[] = [];
  for (const asset of all) {
    const platform = platformOf(asset.name);
    if (!platform) continue;
    const shared = platform === "android" ? undefined : urls.get("desktop-checksums.txt");
    downloads.push({
      platform,
      href: asset.browser_download_url,
      label: asset.name.slice(asset.name.lastIndexOf(".")),
      arch: archOf(asset.name),
      version: version ?? undefined,
      size: asset.size,
      checksumHref: urls.get(`${asset.name}.sha256`) ?? shared,
    });
  }
  return { version, downloads };
}
