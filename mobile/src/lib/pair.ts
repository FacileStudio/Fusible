import { isPrivateHost } from "./hosts";

export type PairTarget = { host: string; token: string; name: string };
export type PairParse = ({ ok: true } & PairTarget) | { ok: false; error: string };
export type Claimed = { device_id: string; device_token: string };

const PREFIX = "fusible://pair?";
const TOKEN_RE = /^[0-9a-f]{64}$/;

/** Reads a `fusible://pair?h=<ip:port>&t=<token>&n=<name>` link. Refuses any host outside private IPv4. */
export function parsePairUri(uri: string): PairParse {
  const text = uri.trim();
  if (!text.startsWith(PREFIX)) return { ok: false, error: "not_pair_code" };
  const query = new URLSearchParams(text.slice(PREFIX.length));
  const host = `http://${query.get("h") ?? ""}`;
  if (!isPrivateHost(host)) return { ok: false, error: "not_private" };
  const token = query.get("t") ?? "";
  if (!TOKEN_RE.test(token)) return { ok: false, error: "bad_token" };
  return { ok: true, host, token, name: query.get("n")?.trim() || "Fusible" };
}

/** Trades a pairing token for a device token. Throws with the HTTP status when the studio refuses. */
export async function claim(host: string, token: string, deviceName: string): Promise<Claimed> {
  if (!isPrivateHost(host)) throw new Error("not_private");
  const res = await fetch(`${host}/api/pair/claim`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ token, name: deviceName.slice(0, 64) }),
  });
  if (!res.ok) throw new Error(`claim_${res.status}`);
  return (await res.json()) as Claimed;
}
