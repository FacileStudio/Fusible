const pad = (id: string, x: number, y: number) => ({
  type: "pcb_smtpad",
  pcb_smtpad_id: id,
  shape: "rect",
  x,
  y,
  width: 1,
  height: 1.4,
  layer: "top",
  pcb_component_id: "pc1",
});

export const mockCircuit = [
  { type: "source_component", source_component_id: "sc1", name: "R1", ftype: "simple_resistor", resistance: 330 },
  { type: "pcb_board", pcb_board_id: "b1", center: { x: 0, y: 0 }, width: 24, height: 16, thickness: 1.6 },
  {
    type: "pcb_component",
    pcb_component_id: "pc1",
    source_component_id: "sc1",
    center: { x: 0, y: 0 },
    width: 4,
    height: 2,
    layer: "top",
    rotation: 0,
  },
  {
    type: "schematic_component",
    schematic_component_id: "sch1",
    source_component_id: "sc1",
    center: { x: 0, y: 0 },
    size: { width: 1.2, height: 0.5 },
  },
  pad("p1", -1.5, 0),
  pad("p2", 1.5, 0),
];
