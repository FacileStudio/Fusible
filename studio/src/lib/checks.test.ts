import { expect, test } from "bun:test";
import { fixLabel, groupByCategory, nearestPcbComponent, setPropFixes, tone } from "./checks";
import type { Diagnostic, Rule } from "./types";

const rule = (id: string, category: string): Rule => ({
  id,
  category,
  standard: "",
  source_url: "",
  severity: "error",
  enabled: true,
  summary: "",
});

test("info is neutral, warning is warning, the rest is danger", () => {
  expect(tone("info")).toBe("info");
  expect(tone("warning")).toBe("warning");
  expect(tone("error")).toBe("danger");
  expect(tone(undefined)).toBe("danger");
});

test("diagnostics group by rule category in a fixed order, unknown rules last", () => {
  const rules = [rule("trace_clearance", "drc"), rule("unconnected_net", "netlist")];
  const diags: Diagnostic[] = [{ rule: "trace_clearance" }, { rule: "made_up" }, { rule: "unconnected_net" }];
  const groups = groupByCategory(diags, rules);
  expect(groups.map((g) => g.category)).toEqual(["netlist", "drc", "other"]);
  expect(groups[0].items[0].index).toBe(2);
});

test("only set_prop fixes are applicable", () => {
  const diag: Diagnostic = {
    fixes: [{ note: "move it" }, { set_prop: { selector: ".C1", prop: "pcbX", value: 2 } }],
  };
  const fixes = setPropFixes(diag);
  expect(fixes).toHaveLength(1);
  expect(fixLabel(fixes[0])).toBe("Set pcbX = 2 on C1");
});

test("the nearest pcb component wins", () => {
  const circuit = [
    { type: "pcb_component", pcb_component_id: "a", center: { x: 0, y: 0 } },
    { type: "pcb_component", pcb_component_id: "b", center: { x: 5, y: 5 } },
    { type: "pcb_trace" },
  ];
  expect(nearestPcbComponent(circuit, 4, 4)).toBe("b");
  expect(nearestPcbComponent([], 0, 0)).toBeNull();
});
