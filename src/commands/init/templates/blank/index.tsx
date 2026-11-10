export default () => (
  <board width="30mm" height="20mm">
    <schematicsheet />
    <pinheader
      name="J1"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={-8}
      pcbY={0}
      pinLabels={{ pin1: "V3V3", pin2: "GND" }}
      pinAttributes={{
        V3V3: { providesPower: true, providesVoltage: 3.3 },
        GND: { providesGround: true },
      }}
    />
    <resistor name="R1" resistance="1k" footprint="0402" pcbX={0} pcbY={2} />
    <led name="D1" color="green" footprint="0603" pcbX={5} pcbY={2} />

    <trace from=".J1 > .V3V3" to=".R1 > .pin1" />
    <trace from=".R1 > .pin2" to=".D1 > .anode" />
    <trace from=".D1 > .cathode" to=".J1 > .GND" />
  </board>
)
