import type {
  BodyInfo,
  BomData,
  BomFile,
  DevPayload,
  SetProp,
  FirmwareData,
  GitStatus,
  KicadBoard,
  PairStart,
  Project,
  Rule,
  Run,
} from "./types";

/** An HTTP failure that keeps the status so callers can tell a missing route from a broken one. */
export class ApiError extends Error {
  status: number;
  body: { details?: string };

  constructor(message: string, status: number, body: { details?: string } = {}) {
    super(message);
    this.status = status;
    this.body = body;
  }
}

/** Builds the circuit route for a board. */
export function circuitUrl(board: string): string {
  return `/api/circuit?board=${encodeURIComponent(board)}`;
}

/** Builds the firmware route for a firmware name. */
export function firmwareUrl(name: string): string {
  return `/api/firmware?name=${encodeURIComponent(name)}`;
}

/** Fetches `url` and parses the JSON reply; a non-2xx reply throws an ApiError carrying the server message. */
export async function getJson<T>(url: string, init: RequestInit = {}): Promise<T> {
  const response = await fetch(url, { cache: "no-store", ...init });
  const body = await response.json().catch(() => ({}));
  if (!response.ok) throw new ApiError(body.error || `HTTP ${response.status}`, response.status, body);
  return body as T;
}

/** Sends a JSON body with `method` and parses the JSON reply. */
export function sendJson<T>(method: string, url: string, body?: unknown): Promise<T> {
  const init = { method, headers: { "Content-Type": "application/json" }, body: JSON.stringify(body ?? {}) };
  return getJson<T>(url, init);
}

interface GitReply {
  status?: GitStatus;
  message?: string;
  error?: string;
  details?: string;
}

/** One typed helper per studio route. */
export const api = {
  getProject: () => getJson<Project>("/api/project"),
  getCircuit: (board: string) => getJson<DevPayload>(circuitUrl(board)),
  getBom: () => getJson<BomData>("/api/bom"),
  saveBom: (entries: BomFile) => sendJson<BomData>("POST", "/api/bom", entries),
  getRuns: () => getJson<Run[]>("/api/runs"),
  startRun: (action: string, target?: string) =>
    sendJson<{ run: number }>("POST", "/api/run", target ? { action, target } : { action }),
  getFirmware: (name: string) => getJson<FirmwareData>(firmwareUrl(name)),
  getKicad: () => getJson<KicadBoard[]>("/api/kicad"),
  getRules: () => getJson<Rule[]>("/api/rules"),
  applyFix: (board: string, fix: SetProp) =>
    sendJson<{ applied: boolean }>("POST", "/api/fix", { board: board || undefined, fix: { set_prop: fix } }),
  getBodies: () => getJson<BodyInfo[]>("/api/bodies"),
  startPair: () => sendJson<PairStart>("POST", "/api/pair/start"),
  getGit: (check: boolean) => getJson<GitStatus>(`/api/git${check ? "?check=1" : ""}`),
  postGit: (action: string, body?: unknown) => sendJson<GitReply>("POST", `/api/git/${action}`, body),
};
