import { Preferences } from "@capacitor/preferences";

export type Host = {
  id: string;
  name: string;
  url: string;
  token: string;
  paired_at: string;
  unpaired?: boolean;
};

const KEY = "fusible.hosts";
const OCTET = "(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)";
const URL_RE = new RegExp(`^http://${OCTET}\\.${OCTET}\\.${OCTET}\\.${OCTET}:(\\d{1,5})/?$`);

/** True only for `http://<RFC 1918 IPv4>:<port>`. Every request to a studio goes through this check. */
export function isPrivateHost(url: string): boolean {
  const m = URL_RE.exec(url);
  if (!m) return false;
  const [a, b] = [Number(m[1]), Number(m[2])];
  const port = Number(m[5]);
  if (port < 1 || port > 65535) return false;
  return a === 10 || (a === 172 && b >= 16 && b <= 31) || (a === 192 && b === 168);
}

/** Reads the paired studios; a missing or damaged entry is an empty list. */
export async function loadHosts(): Promise<Host[]> {
  try {
    const { value } = await Preferences.get({ key: KEY });
    const list = JSON.parse(value ?? "[]") as Host[];
    return list.filter((h) => isPrivateHost(h.url));
  } catch {
    return [];
  }
}

/** Saves the paired studios. */
export async function saveHosts(hosts: Host[]): Promise<void> {
  await Preferences.set({ key: KEY, value: JSON.stringify(hosts) });
}

/** Adds or replaces the studio with the same address. */
export async function addHost(host: Host): Promise<void> {
  const rest = (await loadHosts()).filter((h) => h.url !== host.url);
  await saveHosts([...rest, host]);
}

/** Forgets one studio on this phone. The studio keeps its record until it is revoked there. */
export async function removeHost(id: string): Promise<void> {
  await saveHosts((await loadHosts()).filter((h) => h.id !== id));
}

/** Flags a studio whose token the server no longer accepts. */
export async function markUnpaired(id: string): Promise<void> {
  const hosts = await loadHosts();
  await saveHosts(hosts.map((h) => (h.id === id ? { ...h, unpaired: true } : h)));
}

/** Whether the first-run walkthrough has been done. */
export async function isOnboarded(): Promise<boolean> {
  return (await Preferences.get({ key: "fusible.onboarded" })).value === "1";
}

/** Remembers that the walkthrough is done. */
export async function setOnboarded(): Promise<void> {
  await Preferences.set({ key: "fusible.onboarded", value: "1" });
}
