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

const decoupling = { footprint: "0402", maxDecouplingTraceLength: 40 }

export default () => (
  <board width="60mm" height="45mm">
    <schematicsheet />
    <pinheader
      name="J1"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={-25}
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
      pcbX={-18}
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
    <capacitor name="C1" capacitance="1uF" footprint="0603" maxDecouplingTraceLength={40} pcbX={-20} pcbY={5} />
    <capacitor name="C2" capacitance="10uF" footprint="0805" maxDecouplingTraceLength={40} pcbX={-15} pcbY={5} />
    <chip
      name="U1"
      manufacturerPartNumber="RP2040"
      footprint="qfn56_thermalpad3.2x3.2mm"
      pcbX={0}
      pcbY={0}
      pinLabels={{
        pin1: "IOVDD1",
        pin2: "GPIO0",
        pin3: "GPIO1",
        pin10: "IOVDD2",
        pin19: "TESTEN",
        pin20: "XIN",
        pin21: "XOUT",
        pin22: "IOVDD3",
        pin23: "DVDD1",
        pin24: "SWCLK",
        pin25: "SWDIO",
        pin26: "RUN",
        pin27: "GPIO16",
        pin33: "IOVDD4",
        pin42: "IOVDD5",
        pin43: "ADC_AVDD",
        pin44: "VREG_VIN",
        pin45: "VREG_VOUT",
        pin48: "USB_VDD",
        pin49: "IOVDD6",
        pin50: "DVDD2",
        pin51: "QSPI_SD3",
        pin52: "QSPI_SCLK",
        pin53: "QSPI_SD0",
        pin54: "QSPI_SD2",
        pin55: "QSPI_SD1",
        pin56: "QSPI_SS",
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
        XIN: { isInput: true },
        XOUT: { isOutput: true },
        GPIO0: { isOutput: true, isConfiguredForUartTx: true },
        GPIO1: { isInput: true, isConfiguredForUartRx: true },
        GPIO16: { isGpio: true, isOutput: true, isUsingPushPull: true },
        SWCLK: { isInput: true },
        SWDIO: { isBidirectional: true },
      }}
    />
    <chip
      name="U3"
      manufacturerPartNumber="W25Q16JVSNIQ"
      footprint="soic8"
      pcbX={-4}
      pcbY={11}
      pinLabels={{ pin1: "CS", pin2: "DO", pin3: "IO2", pin4: "GND", pin5: "DI", pin6: "CLK", pin7: "IO3", pin8: "VCC" }}
      pinAttributes={{
        VCC: io,
        GND: { requiresGround: true },
        CS: { isInput: true },
        CLK: { isInput: true },
      }}
    />
    <chip
      name="X1"
      manufacturerPartNumber="ABM8-272-T3"
      footprint="crystal"
      pcbX={-2}
      pcbY={-9}
      pinLabels={{ pin1: "X1", pin2: "GND1", pin3: "X2", pin4: "GND2" }}
      pinAttributes={{
        X1: { isBidirectional: true },
        X2: { isBidirectional: true },
        GND1: { requiresGround: true },
        GND2: { requiresGround: true },
      }}
    />
    <capacitor name="C3" capacitance="15pF" footprint="0402" pcbX={-6} pcbY={-9} />
    <capacitor name="C4" capacitance="15pF" footprint="0402" pcbX={2} pcbY={-11} />
    <resistor name="R3" resistance="1k" footprint="0402" pcbX={2} pcbY={-8} />
    <capacitor name="C5" capacitance="100nF" {...decoupling} pcbX={-7} pcbY={3} />
    <capacitor name="C6" capacitance="100nF" {...decoupling} pcbX={-7} pcbY={-2} />
    <capacitor name="C7" capacitance="100nF" {...decoupling} pcbX={3} pcbY={-5.5} />
    <capacitor name="C8" capacitance="100nF" {...decoupling} pcbX={7} pcbY={-2} />
    <capacitor name="C9" capacitance="100nF" {...decoupling} pcbX={7} pcbY={3} />
    <capacitor name="C10" capacitance="100nF" {...decoupling} pcbX={6} pcbY={6} />
    <capacitor name="C11" capacitance="1uF" {...decoupling} pcbX={4} pcbY={7} />
    <capacitor name="C12" capacitance="1uF" {...decoupling} pcbX={2} pcbY={6.5} />
    <capacitor name="C13" capacitance="100nF" {...decoupling} pcbX={0} pcbY={7} />
    <capacitor name="C14" capacitance="100nF" {...decoupling} pcbX={-1} pcbY={-6} />
    <capacitor name="C15" capacitance="100nF" {...decoupling} pcbX={0} pcbY={11} />
    <resistor name="R1" resistance="10k" footprint="0402" pcbX={7} pcbY={-7} />
    <resistor name="R2" resistance="330" footprint="0402" pcbX={14} pcbY={0} />
    <led name="D1" color="green" footprint="0603" pcbX={18} pcbY={0} />
    <pinheader
      name="J2"
      pinCount={4}
      manufacturerPartNumber="PZ254V-11-04P"
      pcbX={12}
      pcbY={-14}
      pinLabels={{ pin1: "V3V3", pin2: "SWDIO", pin3: "SWCLK", pin4: "GND" }}
    />

    <trace from=".J1 > .VBUS" to="net.VBUS" />
    <trace from=".U2 > .VIN" to="net.VBUS" />
    <trace from=".U2 > .EN" to="net.VBUS" />
    <trace from=".C1 > .pin1" to="net.VBUS" />
    <trace from=".U2 > .VOUT" to="net.V3V3" />
    <trace from=".C2 > .pin1" to="net.V3V3" />

    <trace from=".J1 > .GND" to="net.GND" />
    <trace from=".U2 > .GND" to="net.GND" />
    <trace from=".C1 > .pin2" to="net.GND" />
    <trace from=".C2 > .pin2" to="net.GND" />
    <trace from=".U1 > .thermalpad" to="net.GND" />
    <trace from=".U1 > .TESTEN" to="net.GND" />

    <trace from=".U1 > .IOVDD1" to="net.V3V3" />
    <trace from=".U1 > .IOVDD2" to="net.V3V3" />
    <trace from=".U1 > .IOVDD3" to="net.V3V3" />
    <trace from=".U1 > .IOVDD4" to="net.V3V3" />
    <trace from=".U1 > .IOVDD5" to="net.V3V3" />
    <trace from=".U1 > .IOVDD6" to="net.V3V3" />
    <trace from=".U1 > .USB_VDD" to="net.V3V3" />
    <trace from=".U1 > .ADC_AVDD" to="net.V3V3" />
    <trace from=".U1 > .VREG_VIN" to="net.V3V3" />
    <trace from=".U1 > .VREG_VOUT" to="net.V1V1" />
    <trace from=".U1 > .DVDD1" to="net.V1V1" />
    <trace from=".U1 > .DVDD2" to="net.V1V1" />

    <trace from=".C5 > .pin1" to=".U1 > .IOVDD1" />
    <trace from=".C6 > .pin1" to=".U1 > .IOVDD2" />
    <trace from=".C7 > .pin1" to=".U1 > .IOVDD3" />
    <trace from=".C14 > .pin1" to=".U1 > .DVDD1" />
    <trace from=".C8 > .pin1" to=".U1 > .IOVDD4" />
    <trace from=".C9 > .pin1" to=".U1 > .IOVDD5" />
    <trace from=".C10 > .pin1" to=".U1 > .ADC_AVDD" />
    <trace from=".C11 > .pin1" to=".U1 > .VREG_VIN" />
    <trace from=".C12 > .pin1" to=".U1 > .VREG_VOUT" />
    <trace from=".C13 > .pin1" to=".U1 > .IOVDD6" />
    <trace from=".C5 > .pin2" to="net.GND" />
    <trace from=".C6 > .pin2" to="net.GND" />
    <trace from=".C7 > .pin2" to="net.GND" />
    <trace from=".C8 > .pin2" to="net.GND" />
    <trace from=".C9 > .pin2" to="net.GND" />
    <trace from=".C10 > .pin2" to="net.GND" />
    <trace from=".C11 > .pin2" to="net.GND" />
    <trace from=".C12 > .pin2" to="net.GND" />
    <trace from=".C13 > .pin2" to="net.GND" />
    <trace from=".C14 > .pin2" to="net.GND" />

    <trace from=".U1 > .QSPI_SS" to=".U3 > .CS" />
    <trace from=".U1 > .QSPI_SD1" to=".U3 > .DO" />
    <trace from=".U1 > .QSPI_SD2" to=".U3 > .IO2" />
    <trace from=".U1 > .QSPI_SD0" to=".U3 > .DI" />
    <trace from=".U1 > .QSPI_SCLK" to=".U3 > .CLK" />
    <trace from=".U1 > .QSPI_SD3" to=".U3 > .IO3" />
    <trace from=".U3 > .VCC" to="net.V3V3" />
    <trace from=".U3 > .GND" to="net.GND" />
    <trace from=".C15 > .pin1" to=".U3 > .VCC" />
    <trace from=".C15 > .pin2" to="net.GND" />

    <trace from=".U1 > .XIN" to=".X1 > .X1" />
    <trace from=".U1 > .XOUT" to=".R3 > .pin1" />
    <trace from=".R3 > .pin2" to=".X1 > .X2" />
    <trace from=".C3 > .pin1" to=".X1 > .X1" />
    <trace from=".C4 > .pin1" to=".X1 > .X2" />
    <trace from=".C3 > .pin2" to="net.GND" />
    <trace from=".C4 > .pin2" to="net.GND" />
    <trace from=".X1 > .GND1" to="net.GND" />
    <trace from=".X1 > .GND2" to="net.GND" />

    <trace from=".R1 > .pin1" to="net.V3V3" />
    <trace from=".R1 > .pin2" to=".U1 > .RUN" />
    <trace from=".U1 > .SWCLK" to="net.SWCLK" />
    <trace from=".J2 > .SWCLK" to="net.SWCLK" />
    <trace from=".U1 > .SWDIO" to="net.SWDIO" />
    <trace from=".J2 > .SWDIO" to="net.SWDIO" />
    <trace from=".J2 > .V3V3" to="net.V3V3" />
    <trace from=".J2 > .GND" to="net.GND" />

    <trace from=".U1 > .GPIO16" to="net.LED" />
    <trace from=".R2 > .pin1" to="net.LED" />
    <trace from=".R2 > .pin2" to=".D1 > .anode" />
    <trace from=".D1 > .cathode" to="net.GND" />
  </board>
)
