import { mockCircuit } from "./fixture-circuit";

export const MOCK_ID = "mock";

const board = (name: string, blown: boolean, errors: number, warnings: number) => ({
  name,
  fuse_blown: blown,
  errors,
  warnings,
});

const summary = {
  project: "Desk lamp",
  revision: "mock0001",
  boards: [board("main", false, 0, 1), board("sensor", true, 2, 0)],
  bom_total: { amount: 12.34, currency: "EUR" },
  bodies: [
    { name: "bracket", kind: "code" },
    { name: "case", kind: "authored" },
  ],
  last_run: { id: 3, label: "check", status: "done", lines: [] },
};

const line = (id: string, des: string[], value: string, quantity: number, total: number) => ({
  id,
  kind: "component",
  description: value,
  value,
  designators: des,
  quantity,
  total,
});

const bom = {
  report: {
    currency: "EUR",
    lines: [
      line("r", ["R1", "R2"], "10k", 2, 0.04),
      line("c", ["C1"], "100nF", 1, 0.02),
      line("u", ["U1"], "STM32G431", 1, 3.1),
    ],
    totals: { total: 12.34 },
  },
};

const KICAD = "https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html";

const diagnostics = (blown: boolean) => [
  {
    rule: blown ? "led_current" : "decoupling",
    message: blown ? "LED1 draws 38 mA, above the 20 mA limit" : "U1 has no decoupling capacitor",
    help: blown ? "raise R1 to 330 ohm" : "add 100 nF close to VDD",
    severity: blown ? "error" : "warning",
    standard: blown ? "IPC-2152" : undefined,
    source_url: blown ? KICAD : undefined,
  },
];

/** Canned answers for `?mock=1`, so the UI can be shown without a studio. */
export function mockGet(path: string): unknown {
  if (path.startsWith("/api/mobile/summary")) return summary;
  if (path.startsWith("/api/bom")) return bom;
  if (path.startsWith("/api/circuit")) {
    const blown = path.includes("sensor");
    const circuit_json = mockCircuit;
    return { board_name: blown ? "sensor" : "main", fuse_blown: blown, diagnostics: diagnostics(blown), circuit_json };
  }
  throw new Error("mock_missing");
}

export const mockHost = {
  id: MOCK_ID,
  name: "Demo studio",
  url: "http://192.168.1.20:3020",
  token: "",
  paired_at: "2026-11-04T09:00:00Z",
};
