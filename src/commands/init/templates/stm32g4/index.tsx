const supply = {
  requiresPower: true,
  requiresVoltage: 3.3,
  requiredVoltageTolerance: 0.1,
  shouldHaveDecouplingCapacitor: true,
  recommendedDecouplingCapacitorCapacitance: "100nF",
}

export default () => (
  <board width="50mm" height="36mm">
    <schematicsheet />
    <pinheader
      name="J1"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={-16}
      pcbY={0}
      pinLabels={{ pin1: "VBUS", pin2: "GND" }}
      pinAttributes={{
        VBUS: { providesPower: true, providesVoltage: 5 },
        GND: { providesGround: true },
      }}
    />
    <chip
      name="U2"
      manufacturerPartNumber="AP2112K-3.3TRG1"
      footprint="sot23_5"
      pcbX={-9}
      pcbY={0}
      pinLabels={{ pin1: "VIN", pin2: "GND", pin3: "EN", pin4: "NC", pin5: "VOUT" }}
      pinAttributes={{
        VIN: { requiresPower: true, requiresVoltage: 5, requiredVoltageTolerance: 0.2 },
        GND: { requiresGround: true },
        EN: { isInput: true },
        NC: { doNotConnect: true },
        VOUT: { providesPower: true, providesVoltage: 3.3 },
      }}
    />
    <capacitor name="C1" capacitance="1uF" footprint="0603" maxDecouplingTraceLength={30} pcbX={-11} pcbY={4} />
    <capacitor name="C2" capacitance="10uF" footprint="0805" maxDecouplingTraceLength={30} pcbX={-6} pcbY={4} />
    <chip
      name="U1"
      manufacturerPartNumber="STM32G431KBT6"
      footprint="lqfp32"
      pcbX={5}
      pcbY={0}
      pinLabels={{
        pin1: "VDD1",
        pin4: "NRST",
        pin7: "PA2",
        pin8: "PA3",
        pin10: "PA5",
        pin14: "VSSA",
        pin15: "VDDA",
        pin16: "VSS1",
        pin17: "VDD2",
        pin23: "PA13",
        pin24: "PA14",
        pin31: "PB8",
        pin32: "VSS2",
      }}
      pinAttributes={{
        VDD1: supply,
        VDD2: supply,
        VDDA: supply,
        VSS1: { requiresGround: true },
        VSS2: { requiresGround: true },
        VSSA: { requiresGround: true },
        NRST: { isInput: true, canUseInternalPullup: true, isUsingInternalPullup: true },
        PA2: { isOutput: true, isConfiguredForUartTx: true },
        PA3: { isInput: true, isConfiguredForUartRx: true },
        PA5: { isGpio: true, isOutput: true, isUsingPushPull: true },
        PA13: { isBidirectional: true },
        PA14: { isInput: true },
        PB8: { isInput: true, needsExternalPulldown: true },
      }}
    />
    <capacitor name="C3" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={30} pcbX={1} pcbY={5} />
    <capacitor name="C4" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={30} pcbX={9} pcbY={-5} />
    <capacitor name="C5" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={30} pcbX={9} pcbY={5} />
    <capacitor name="C6" capacitance="100nF" footprint="0402" pcbX={-2} pcbY={2} />
    <capacitor name="C7" capacitance="1uF" footprint="0402" maxDecouplingTraceLength={30} pcbX={12} pcbY={-2} />
    <resistor name="R1" resistance="10k" footprint="0402" pcbX={1} pcbY={-5} />
    <resistor name="R2" resistance="330" footprint="0402" pcbX={14} pcbY={-5} />
    <led name="D1" color="green" footprint="0603" pcbX={17} pcbY={-5} />
    <pinheader
      name="J2"
      pinCount={4}
      manufacturerPartNumber="PZ254V-11-04P"
      pcbX={10}
      pcbY={12}
      pinLabels={{ pin1: "V3V3", pin2: "SWDIO", pin3: "SWCLK", pin4: "GND" }}
    />

    <trace from=".U1 > .PA14" to="net.SWCLK" />
    <trace from=".J2 > .SWCLK" to="net.SWCLK" />
    <trace from=".U1 > .PA13" to="net.SWDIO" />
    <trace from=".J2 > .SWDIO" to="net.SWDIO" />
    <trace from=".R2 > .pin2" to=".D1 > .anode" />
    <trace from=".U1 > .PA5" to="net.LED" />
    <trace from=".R2 > .pin1" to="net.LED" />
    <trace from=".U1 > .PB8" to="net.BOOT0" />
    <trace from=".R1 > .pin1" to="net.BOOT0" />
    <trace from=".U1 > .VDDA" to=".C4 > .pin1" />
    <trace from=".U1 > .VDDA" to=".U1 > .VDD2" />
    <trace from=".U2 > .VOUT" to=".C2 > .pin1" />
    <trace from=".C3 > .pin1" to=".U1 > .VDD1" />
    <trace from=".C5 > .pin1" to=".U1 > .VDD2" />
    <trace from=".U1 > .VDD1" to=".U1 > .VDDA" />
    <trace from=".J2 > .V3V3" to=".C5 > .pin1" />
    <trace from=".C2 > .pin1" to=".C3 > .pin1" />
    <trace from=".U1 > .VSSA" to=".U1 > .VSS1" />
    <trace from=".C3 > .pin2" to=".U1 > .VSS2" />
    <trace from=".U1 > .VSS1" to=".C4 > .pin2" />
    <trace from=".C1 > .pin2" to=".U2 > .GND" />
    <trace from=".R1 > .pin2" to=".U1 > .VSSA" />
    <trace from=".J1 > .GND" to=".U2 > .GND" />
    <trace from=".C1 > .pin2" to=".C2 > .pin2" />
    <trace from=".U1 > .VSS2" to=".C5 > .pin2" />
    <trace from=".C2 > .pin2" to=".C3 > .pin2" />
    <trace from=".U1 > .VSS2" to=".U1 > .VSSA" />
    <trace from=".C5 > .pin2" to=".J2 > .GND" />
    <trace from=".C4 > .pin2" to=".D1 > .cathode" />
    <trace from=".U2 > .VIN" to=".U2 > .EN" />
    <trace from=".C1 > .pin1" to=".U2 > .VIN" />
    <trace from=".J1 > .VBUS" to=".C1 > .pin1" />
    <trace from=".C6 > .pin1" to=".U1 > .NRST" />
    <trace from=".C6 > .pin2" to=".U2 > .GND" />
    <trace from=".C7 > .pin1" to=".U1 > .VDDA" />
    <trace from=".C7 > .pin2" to=".U1 > .VSSA" />
  </board>
)
