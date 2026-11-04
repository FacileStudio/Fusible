import { type Host, isPrivateHost, markUnpaired } from "./hosts";
import { MOCK_ID, mockGet } from "./fixtures";

const TIMEOUT_MS = 8000;

export class UnpairedError extends Error {}

async function send(host: Host, path: string): Promise<Response> {
  if (!isPrivateHost(host.url)) throw new Error("not_private");
  const control = new AbortController();
  const timer = setTimeout(() => control.abort(), TIMEOUT_MS);
  try {
    const headers = { Authorization: `Bearer ${host.token}` };
    const res = await fetch(`${host.url}${path}`, { headers, signal: control.signal });
    if (res.status === 401) {
      await markUnpaired(host.id);
      throw new UnpairedError("unpaired");
    }
    if (!res.ok) throw new Error(`http_${res.status}`);
    return res;
  } finally {
    clearTimeout(timer);
  }
}

/** GETs JSON from a paired studio. A 401 marks the host unpaired and throws {@link UnpairedError}. */
export async function get<T>(host: Host, path: string): Promise<T> {
  if (host.id === MOCK_ID) return mockGet(path) as T;
  return (await send(host, path)).json() as Promise<T>;
}

/** GETs a binary file such as a GLB from a paired studio. */
export async function getBuffer(host: Host, path: string): Promise<ArrayBuffer> {
  if (host.id === MOCK_ID) throw new Error("mock_no_binary");
  return (await send(host, path)).arrayBuffer();
}
