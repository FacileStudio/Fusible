const rail3v3 = { requiresPower: true, requiresVoltage: 3.3, requiredVoltageTolerance: 0.1 }

export default () => (
  <board width="60mm" height="56mm">
    <schematicsheet />
    <chip
      name="J1"
      manufacturerPartNumber="10118194-0001LF"
      footprint="usbmicro"
      pcbX={-27}
      pcbY={-10}
      pcbRotation={-90}
      pinLabels={{ pin1: "VBUS", pin2: "DM", pin3: "DP", pin4: "ID", pin5: "GND" }}
      pinAttributes={{
        VBUS: { providesPower: true, providesVoltage: 5 },
        GND: { providesGround: true },
        DM: { isBidirectional: true },
        DP: { isBidirectional: true },
        ID: { doNotConnect: true },
      }}
    />
    <chip
      name="U2"
      manufacturerPartNumber="AP2112K-3.3TRG1"
      footprint="sot23_5"
      pcbX={-18}
      pcbY={-10}
      pinLabels={{ pin1: "VIN", pin2: "GND", pin3: "EN", pin4: "NC", pin5: "VOUT" }}
      pinAttributes={{
        VIN: { requiresPower: true, requiresVoltage: 5, requiredVoltageTolerance: 0.2 },
        GND: { requiresGround: true },
        EN: { isInput: true },
        NC: { doNotConnect: true },
        VOUT: { providesPower: true, providesVoltage: 3.3 },
      }}
    />
    <capacitor name="C1" capacitance="1uF" footprint="0603" maxDecouplingTraceLength={30} pcbX={-21} pcbY={-15} />
    <capacitor name="C2" capacitance="22uF" footprint="0805" maxDecouplingTraceLength={30} pcbX={-15} pcbY={-15} />
    <chip
      name="U1"
      manufacturerPartNumber="ESP32-S3-WROOM-1-N8"
      footprint="kicad:RF_Module/ESP32-S3-WROOM-1"
      pcbX={6}
      pcbY={0.25}
      pinLabels={{
        pin1: "GND1",
        pin2: "3V3",
        pin3: "EN",
        pin13: "GPIO19",
        pin14: "GPIO20",
        pin25: "GPIO48",
        pin27: "GPIO0",
        pin36: "GPIO44",
        pin37: "GPIO43",
        pin40: "GND2",
        pin41: "GND3",
      }}
      pinAttributes={{
        GND1: { requiresGround: true },
        GND2: { requiresGround: true },
        GND3: { requiresGround: true },
        "3V3": {
          ...rail3v3,
          shouldHaveDecouplingCapacitor: true,
          recommendedDecouplingCapacitorCapacitance: "100nF",
        },
        EN: { isInput: true, needsExternalPullup: true },
        GPIO0: { isGpio: true, canUseInternalPullup: true, isUsingInternalPullup: true },
        GPIO19: { isBidirectional: true },
        GPIO20: { isBidirectional: true },
        GPIO43: { isOutput: true, isConfiguredForUartTx: true },
        GPIO44: { isInput: true, isConfiguredForUartRx: true },
        GPIO48: { isGpio: true, isOutput: true, isUsingPushPull: true },
      }}
    />
    <capacitor name="C3" capacitance="100nF" footprint="0402" maxDecouplingTraceLength={30} pcbX={-5} pcbY={4.24} />
    <resistor name="R1" resistance="10k" footprint="0402" pcbX={-7} pcbY={3} />
    <capacitor name="C4" capacitance="1uF" footprint="0402" pcbX={-7} pcbY={1} />
    <pinheader
      name="J2"
      pinCount={2}
      manufacturerPartNumber="PZ254V-11-02P"
      pcbX={23}
      pcbY={-12}
      pinLabels={{ pin1: "BOOT", pin2: "GND" }}
    />
    <resistor name="R2" resistance="330" footprint="0402" pcbX={12} pcbY={-17} />
    <led name="D1" color="green" footprint="0603" pcbX={12} pcbY={-21} />

    <trace from=".J1 > .VBUS" to="net.VBUS" />
    <trace from=".U2 > .VIN" to="net.VBUS" />
    <trace from=".U2 > .EN" to="net.VBUS" />
    <trace from=".C1 > .pin1" to="net.VBUS" />
    <trace from=".U2 > .VOUT" to="net.V3V3" />
    <trace from=".C2 > .pin1" to="net.V3V3" />
    <trace from=".U1 > .3V3" to="net.V3V3" />
    <trace from=".C3 > .pin1" to=".U1 > .3V3" />
    <trace from=".R1 > .pin1" to="net.V3V3" />

    <trace from=".J1 > .GND" to="net.GND" />
    <trace from=".U2 > .GND" to="net.GND" />
    <trace from=".C1 > .pin2" to="net.GND" />
    <trace from=".C2 > .pin2" to="net.GND" />
    <trace from=".C3 > .pin2" to="net.GND" />
    <trace from=".U1 > .GND1" to="net.GND" />
    <trace from=".U1 > .GND2" to="net.GND" />
    <trace from=".U1 > .GND3" to="net.GND" />

    <trace from=".R1 > .pin2" to="net.EN" />
    <trace from=".C4 > .pin1" to="net.EN" />
    <trace from=".U1 > .EN" to="net.EN" />
    <trace from=".C4 > .pin2" to="net.GND" />

    <trace from=".U1 > .GPIO0" to="net.BOOT" />
    <trace from=".J2 > .BOOT" to="net.BOOT" />
    <trace from=".J2 > .GND" to="net.GND" />

    <trace from=".U1 > .GPIO19" to="net.USB_DM" />
    <trace from=".J1 > .DM" to="net.USB_DM" />
    <trace from=".U1 > .GPIO20" to="net.USB_DP" />
    <trace from=".J1 > .DP" to="net.USB_DP" />

    <trace from=".U1 > .GPIO48" to="net.LED" />
    <trace from=".R2 > .pin1" to="net.LED" />
    <trace from=".R2 > .pin2" to=".D1 > .anode" />
    <trace from=".D1 > .cathode" to="net.GND" />
  </board>
)
