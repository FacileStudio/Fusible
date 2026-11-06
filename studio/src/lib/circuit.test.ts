import { expect, test } from "bun:test";
import { parseCircuit } from "./circuit";
import { findComponent } from "./find";

const json = [
  { type: "source_component", source_component_id: "s1", name: "U1", manufacturer_part_number: "ESP32" },
  { type: "pcb_component", pcb_component_id: "p1", source_component_id: "s1", center: { x: 1, y: 2 } },
  {
    type: "schematic_component",
    schematic_component_id: "sc1",
    source_component_id: "s1",
    port_labels: { "1": "VBUS" },
  },
  { type: "pcb_trace", connection_name: "VBUS" },
];

test("parseCircuit indexes components, pins and nets", () => {
  const graph = parseCircuit(json);
  expect(graph.components.U1.x).toBe(1);
  expect(graph.components.U1.pins).toEqual([{ pin: "1", net: "VBUS" }]);
  expect(graph.nets.VBUS.count).toBe(1);
});

test("findComponent resolves any element id", () => {
  const graph = parseCircuit(json);
  expect(findComponent(graph.components, "sc1")?.name).toBe("U1");
  expect(findComponent(graph.components, "nope")).toBeNull();
});
