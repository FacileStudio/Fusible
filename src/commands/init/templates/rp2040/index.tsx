const io = {
  requiresPower: true,
  requiresVoltage: 3.3,
  requiredVoltageTolerance: 0.1,
  shouldHaveDecouplingCapacitor: true,
  recommendedDecouplingCapacitorCapacitance: "100nF",
}

const core = {
  requiresPower: true,
  requiresVoltage: 1.1,
  requiredVoltageTolerance: 0.1,
  shouldHaveDecouplingCapacitor: true,
  recommendedDecouplingCapacitorCapacitance: "100nF",
}

export default () => (
  <board width="50mm" height="40mm">
    <schematicsheet />
    <pinheader
      name="J1"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={-20}
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
      pcbX={-13}
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
    <capacitor name="C1" capacitance="1uF" footprint="0603" maxDecouplingTraceLength={40} pcbX={-15} pcbY={5} />
    <capacitor name="C2" capacitance="10uF" footprint="0805" maxDecouplingTraceLength={40} pcbX={-10} pcbY={5} />
    <chip
      name="U1"
      manufacturerPartNumber="RP2040"
      footprint="qfn56_thermalpad3.2x3.2mm"
      pcbX={4}
      pcbY={0}
      pinLabels={{
        pin1: "IOVDD1",
        pin2: "GPIO0",
        pin3: "GPIO1",
        pin10: "IOVDD2",
        pin19: "TESTEN",
        pin22: "IOVDD3",
        pin23: "DVDD1",
        pin24: "SWCLK",
        pin25: "SWDIO",
        pin26: "RUN",
        pin33: "IOVDD4",
        pin27: "GPIO16",
        pin42: "IOVDD5",
        pin43: "ADC_AVDD",
        pin44: "VREG_VIN",
        pin45: "VREG_VOUT",
        pin48: "USB_VDD",
        pin49: "IOVDD6",
        pin50: "DVDD2",
        pin57: "thermalpad",
      }}
      pinAttributes={{
        IOVDD1: io,
        IOVDD2: io,
        IOVDD3: io,
        IOVDD4: io,
        IOVDD5: io,
        IOVDD6: io,
        ADC_AVDD: io,
        USB_VDD: io,
        VREG_VIN: { requiresPower: true, requiresVoltage: 3.3, requiredVoltageTolerance: 0.1 },
        VREG_VOUT: { providesPower: true, providesVoltage: 1.1 },
        DVDD1: core,
        DVDD2: core,
        thermalpad: { requiresGround: true },
        TESTEN: { isInput: true },
        RUN: { isInput: true, needsExternalPullup: true },
        GPIO0: { isOutput: true, isConfiguredForUartTx: true },
        GPIO1: { isInput: true, isConfiguredForUartRx: true },
        GPIO16: { isGpio: true, isOutput: true, isUsingPushPull: true },
        SWCLK: { isInput: true },
        SWDIO: { isBidirectional: true },
      }}
    />
    <capacitor name="C3" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={40} pcbX={-2} pcbY={6} />
    <capacitor name="C4" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={40} pcbX={10} pcbY={6} />
    <capacitor name="C5" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={40} pcbX={10} pcbY={-6} />
    <capacitor name="C6" capacitance="1uF" footprint="0402" maxDecouplingTraceLength={40} pcbX={-2} pcbY={-6} />
    <resistor name="R1" resistance="10k" footprint="0402" pcbX={4} pcbY={-9} />
    <resistor name="R2" resistance="330" footprint="0402" pcbX={16} pcbY={0} />
    <led name="D1" color="green" footprint="0603" pcbX={19} pcbY={0} />
    <pinheader
      name="J2"
      pinCount={4}
      manufacturerPartNumber="PZ254V-11-04P"
      pcbX={6}
      pcbY={15}
      pinLabels={{ pin1: "V3V3", pin2: "SWDIO", pin3: "SWCLK", pin4: "GND" }}
    />

    <trace from=".U2 > .VIN" to=".U2 > .EN" />
    <trace from=".C1 > .pin1" to=".U2 > .VIN" />
    <trace from=".J1 > .VBUS" to=".U2 > .VIN" />
    <trace from=".U1 > .TESTEN" to=".U1 > .thermalpad" />
    <trace from=".J1 > .GND" to=".U2 > .GND" />
    <trace from=".C6 > .pin2" to=".U1 > .TESTEN" />
    <trace from=".C1 > .pin2" to=".U2 > .GND" />
    <trace from=".C1 > .pin2" to=".C2 > .pin2" />
    <trace from=".C2 > .pin2" to=".C3 > .pin2" />
    <trace from=".U1 > .TESTEN" to=".C5 > .pin2" />
    <trace from=".C3 > .pin2" to=".U1 > .thermalpad" />
    <trace from=".U1 > .thermalpad" to=".C4 > .pin2" />
    <trace from=".J2 > .GND" to=".C4 > .pin2" />
    <trace from=".C5 > .pin2" to=".D1 > .cathode" />
    <trace from=".U1 > .IOVDD6" to=".U1 > .USB_VDD" />
    <trace from=".U1 > .VREG_VIN" to=".U1 > .ADC_AVDD" />
    <trace from=".U1 > .ADC_AVDD" to=".U1 > .IOVDD5" />
    <trace from=".U1 > .USB_VDD" to=".U1 > .VREG_VIN" />
    <trace from=".U1 > .ADC_AVDD" to=".C4 > .pin1" />
    <trace from=".C3 > .pin1" to=".U1 > .IOVDD1" />
    <trace from=".U2 > .VOUT" to=".C2 > .pin1" />
    <trace from=".U1 > .IOVDD1" to=".U1 > .IOVDD6" />
    <trace from=".U1 > .IOVDD1" to=".U1 > .IOVDD2" />
    <trace from=".U1 > .IOVDD5" to=".U1 > .IOVDD4" />
    <trace from=".U1 > .IOVDD3" to=".U1 > .IOVDD4" />
    <trace from=".U1 > .IOVDD4" to=".C5 > .pin1" />
    <trace from=".R1 > .pin1" to=".U1 > .IOVDD3" />
    <trace from=".C2 > .pin1" to=".C3 > .pin1" />
    <trace from=".C3 > .pin1" to=".J2 > .V3V3" />
    <trace from=".U1 > .DVDD2" to=".U1 > .VREG_VOUT" />
    <trace from=".C6 > .pin1" to=".U1 > .DVDD1" />
    <trace from=".U1 > .DVDD2" to=".U1 > .DVDD1" />
    <trace from=".R1 > .pin2" to=".U1 > .RUN" />
    <trace from=".U1 > .GPIO16" to="net.LED" />
    <trace from=".R2 > .pin1" to="net.LED" />
    <trace from=".R2 > .pin2" to=".D1 > .anode" />
    <trace from=".J2 > .SWDIO" to=".U1 > .SWDIO" />
    <trace from=".J2 > .SWCLK" to=".U1 > .SWCLK" />
  </board>
)
