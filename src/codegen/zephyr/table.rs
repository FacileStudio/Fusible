//! Parts whose Zephyr binding and bus address Fusible knows, by part number prefix.
//!
//! Every `compatible` is a binding of `dts/bindings/` in the Zephyr tree
//! (<https://github.com/zephyrproject-rtos/zephyr>), none of which marks a
//! property required beyond `reg` and, on SPI, `spi-max-frequency`. Addresses
//! come from the part datasheets: BME280 SDO (0x76 low, 0x77 high; SPI up to
//! 10 MHz), SHT3x-DIS ADDR (0x44, 0x45), MPU-6050 AD0 (0x68, 0x69), BH1750FVI
//! ADDR (0x23, 0x5C); VL53L0X (0x29) and PCF8563 (0x51) have one address.
//! A part missing here, or one whose strap pin the netlist does not tie, needs
//! `firmware.<name>.zephyr.<designator>` in `fusible.yml`.

use crate::codegen::view::Kind;

/// How a part's I2C address is set.
pub(super) enum Address {
    /// The part has no bus address (UART).
    None,
    Fixed(u16),
    /// A pin tied low or high picks one of two addresses.
    Strap {
        pins: &'static [&'static str],
        low: u16,
        high: u16,
    },
}

/// One known part.
pub(super) struct Entry {
    /// Part number prefixes, uppercase, without `-`, `_` or spaces.
    prefixes: &'static [&'static str],
    /// Generic devicetree node name.
    pub node: &'static str,
    pub compatible: &'static str,
    pub buses: &'static [Kind],
    pub address: Address,
    /// Highest SPI clock of the datasheet, in hertz.
    pub spi_hz: Option<u32>,
}

const fn strap(pin: &'static [&'static str], low: u16, high: u16) -> Address {
    Address::Strap {
        pins: pin,
        low,
        high,
    }
}

const fn i2c(
    prefixes: &'static [&'static str],
    node: &'static str,
    compatible: &'static str,
) -> Entry {
    Entry {
        prefixes,
        node,
        compatible,
        buses: &[Kind::I2c],
        address: Address::None,
        spi_hz: None,
    }
}

static TABLE: &[Entry] = &[
    Entry {
        buses: &[Kind::I2c, Kind::Spi],
        address: strap(&["SDO"], 0x76, 0x77),
        spi_hz: Some(10_000_000),
        ..i2c(&["BME280"], "bme280", "bosch,bme280")
    },
    Entry {
        address: strap(&["ADDR"], 0x44, 0x45),
        ..i2c(&["SHT30", "SHT31", "SHT35"], "sht3xd", "sensirion,sht3xd")
    },
    Entry {
        address: strap(&["AD0"], 0x68, 0x69),
        ..i2c(&["MPU6050"], "mpu6050", "invensense,mpu6050")
    },
    Entry {
        address: strap(&["ADDR"], 0x23, 0x5C),
        ..i2c(&["BH1750"], "bh1750", "rohm,bh1750")
    },
    Entry {
        address: Address::Fixed(0x29),
        ..i2c(&["VL53L0X"], "vl53l0x", "st,vl53l0x")
    },
    Entry {
        address: Address::Fixed(0x51),
        ..i2c(&["PCF8563"], "rtc", "nxp,pcf8563")
    },
    Entry {
        buses: &[Kind::Uart],
        ..i2c(&["MAXM10", "MIAM10"], "gnss", "u-blox,m10")
    },
    Entry {
        buses: &[Kind::Uart],
        ..i2c(&["NEOM8", "MAXM8", "SAMM8", "ZOEM8"], "gnss", "u-blox,m8")
    },
];

/// The entry whose prefix starts `mpn` and that sits on `bus`.
pub(super) fn find(mpn: &str, bus: Kind) -> Option<&'static Entry> {
    let clean: String = mpn
        .chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .map(|c| c.to_ascii_uppercase())
        .collect();
    TABLE
        .iter()
        .find(|e| e.buses.contains(&bus) && e.prefixes.iter().any(|p| clean.starts_with(p)))
}
