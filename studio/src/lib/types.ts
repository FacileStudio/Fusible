export type ViewName = "schematic" | "pcb" | "3d" | "mechanical" | "assembly" | "bom";

export type PaneName = "inspector" | "components" | "nets" | "checks" | "rules" | "bodies" | "runs" | "pins";

export interface Diagnostic {
  rule?: string;
  message?: string;
  help?: string;
  severity?: string;
  standard?: string;
  source_url?: string;
  elements?: string[];
  fix?: string;
}

export interface DevPayload {
  board_name?: string;
  board?: string;
  fuse_blown?: boolean;
  diagnostics?: Diagnostic[];
  circuit_json?: unknown;
  revision?: string;
}

export interface ProjectItem {
  name: string;
  board?: string;
  kind?: string;
  exists?: boolean;
}

export interface Project {
  name?: string;
  boards: ProjectItem[];
  firmware: ProjectItem[];
  bodies: ProjectItem[];
  assembly?: unknown;
}

export type RunStatus = "running" | "ok" | "fuse_blown" | "error";

export interface Run {
  id: number;
  label: string;
  status: RunStatus;
  lines: string[];
}

export interface Rule {
  id: string;
  standard: string;
  source_url: string;
  severity: string;
  enabled: boolean;
  summary: string;
}

export interface BodyInfo {
  name: string;
  kind: string;
  source?: string;
  material?: string;
  process?: string;
  mass_g?: number;
  cost?: number;
  bbox?: number[];
  glb?: string;
}

export interface PairStart {
  uri: string;
  expires_at: string;
}

export interface Comp {
  name: string;
  mpn?: string;
  ftype?: string;
  x: number;
  y: number;
  w: number;
  h: number;
  layer: string;
  id: string;
  sourceComponentId?: string;
  pcbComponentId?: string;
  schematicComponentId: string | null;
  pins: { pin: string; net: string }[];
}

export interface Graph {
  components: Record<string, Comp>;
  nets: Record<string, { name: string; count: number }>;
}

export interface Tip {
  x: number;
  y: number;
  name: string;
  type?: string;
  rows: [string, string][];
}

export interface BomLine {
  id: string;
  kind: "component" | "print" | "manual";
  description: string;
  mpn?: string;
  designators: string[];
  quantity: number;
  supplier?: string;
  url?: string;
  grams?: number;
  unit_price?: number | null;
  total?: number | null;
  picked: boolean;
  estimated?: boolean;
}

export interface BomReport {
  currency: string;
  estimate_currency: string;
  price_per_kg: number;
  totals: { electronics: number; printed: number; manual: number; total: number; unpriced: number; estimated: number };
  lines: BomLine[];
}

export interface BomFile {
  parts: Record<string, Record<string, unknown>>;
  prints: Record<string, Record<string, unknown>>;
  print: { price_per_kg?: number };
  currency?: string;
}

export interface BomData {
  report: BomReport;
  file: BomFile;
  editable: boolean;
  pending: number;
}

export interface FirmwarePin {
  pin: string;
  const: string;
  remote?: string;
  violation?: string;
  net: string;
  field: string;
  group: string;
}

export interface FirmwareData {
  board?: string;
  mcu: string;
  framework?: string;
  buildable: boolean;
  pins: FirmwarePin[];
  pins_rs?: string;
  generated?: boolean;
  violations: unknown[];
}

export interface KicadBoard {
  board?: string;
  state: string;
  message?: string;
  differences: string[];
}

export interface GitChange {
  kind: string;
  path: string;
}

export interface GitSave {
  message: string;
  author: string;
  when: number;
  files: string[];
}

export interface GitStatus {
  repo: boolean;
  available?: boolean;
  changes: GitChange[];
  conflicts: string[];
  conflict_authors: Record<string, string>;
  has_remote: boolean;
  upstream?: string;
  ahead: number;
  behind: number;
  fetch_error?: string;
  last_saves: GitSave[];
}
