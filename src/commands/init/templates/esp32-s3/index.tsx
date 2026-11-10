const rail3v3 = { requiresPower: true, requiresVoltage: 3.3, requiredVoltageTolerance: 0.1 }

export default () => (
  <board width="40mm" height="30mm">
    <schematicsheet />
    <pinheader
      name="J1"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={-15}
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
      pcbX={-7}
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
    <capacitor name="C1" capacitance="1uF" footprint="0603" maxDecouplingTraceLength={30} pcbX={-9} pcbY={3} />
    <capacitor name="C2" capacitance="10uF" footprint="0805" maxDecouplingTraceLength={30} pcbX={-4} pcbY={3} />
    <chip
      name="U1"
      manufacturerPartNumber="ESP32-S3-WROOM-1-N8"
      footprint="soic8"
      pcbX={6}
      pcbY={0}
      pinLabels={{
        pin1: "GND",
        pin2: "3V3",
        pin3: "EN",
        pin4: "GPIO0",
        pin5: "GPIO43",
        pin6: "GPIO44",
        pin7: "GPIO48",
        pin8: "GPIO1",
      }}
      pinAttributes={{
        GND: { requiresGround: true },
        "3V3": {
          ...rail3v3,
          shouldHaveDecouplingCapacitor: true,
          recommendedDecouplingCapacitorCapacitance: "100nF",
        },
        EN: { isInput: true, needsExternalPullup: true },
        GPIO0: { isGpio: true, canUseInternalPullup: true, isUsingInternalPullup: true },
        GPIO43: { isOutput: true, isConfiguredForUartTx: true },
        GPIO44: { isInput: true, isConfiguredForUartRx: true },
        GPIO48: { isGpio: true, isOutput: true, isUsingPushPull: true },
        GPIO1: { isGpio: true },
      }}
    />
    <capacitor name="C3" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={30} pcbX={2} pcbY={3.5} />
    <resistor name="R1" resistance="10k" footprint="0402" pcbX={3} pcbY={-4} />
    <resistor name="R2" resistance="330" footprint="0402" pcbX={12} pcbY={-5} />
    <led name="D1" color="green" footprint="0603" pcbX={16} pcbY={-5} />

    <trace from=".J1 > .VBUS" to=".U2 > .VIN" />
    <trace from=".U2 > .VIN" to=".U2 > .EN" />
    <trace from=".U2 > .VIN" to=".C1 > .pin1" />
    <trace from=".J1 > .GND" to=".U2 > .GND" />
    <trace from=".U2 > .GND" to=".C1 > .pin2" />
    <trace from=".C1 > .pin2" to=".C2 > .pin2" />
    <trace from=".C2 > .pin2" to=".C3 > .pin2" />
    <trace from=".C3 > .pin2" to=".U1 > .GND" />
    <trace from=".U2 > .VOUT" to=".C2 > .pin1" />
    <trace from=".C2 > .pin1" to=".C3 > .pin1" />
    <trace from=".U1 > .3V3" to=".C3 > .pin1" />
    <trace from=".U1 > .3V3" to=".R1 > .pin1" />
    <trace from=".R1 > .pin2" to=".U1 > .EN" />
    <trace from=".U1 > .GPIO48" to=".R2 > .pin1" />
    <trace from=".R2 > .pin2" to=".D1 > .anode" />
    <trace from=".D1 > .cathode" to=".U1 > .GND" />
  </board>
)
