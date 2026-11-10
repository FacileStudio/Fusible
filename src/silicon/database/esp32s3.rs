//! Espressif ESP32-S3 IO MUX, ADC, boot strapping, and reserved memory pin tables.
//!
//! ADC channels: ESP32-S3 datasheet, Table 2-8 "Analog Functions": GPIO1-10 are
//! ADC1_CH0-9, GPIO11-20 ADC2_CH0-9 (`AdcChannel(unit, channel)`).

use crate::silicon::model::PeripheralSignal::AdcChannel;
use crate::silicon::model::{
    MuxFunction, PeripheralSignal, PinCapabilities, PinSpec, SiliconTarget, StrappingRequirement,
};

const ESP_GPIO_CAP: PinCapabilities = PinCapabilities {
    is_gpio: true,
    is_analog: false,
    is_input_only: false,
    is_reserved_flash: false,
};

const ESP_ADC_CAP: PinCapabilities = PinCapabilities {
    is_analog: true,
    ..ESP_GPIO_CAP
};

const ESP_INPUT_ONLY: PinCapabilities = PinCapabilities {
    is_gpio: true,
    is_analog: false,
    is_input_only: true,
    is_reserved_flash: false,
};

const ESP_FLASH_CAP: PinCapabilities = PinCapabilities {
    is_gpio: false,
    is_analog: false,
    is_input_only: false,
    is_reserved_flash: true,
};

static GP0_FUNCS: &[MuxFunction] = &[];
static GP3_FUNCS: &[MuxFunction] = &[mux!(AdcChannel(1, 2), 0, "ADC1_CH2")];
static GP43_FUNCS: &[MuxFunction] = &[mux!(PeripheralSignal::UartTx(0), 0, "U0TXD")];
static GP44_FUNCS: &[MuxFunction] = &[mux!(PeripheralSignal::UartRx(0), 0, "U0RXD")];
static GP45_FUNCS: &[MuxFunction] = &[];
static GP46_FUNCS: &[MuxFunction] = &[];
static EMPTY_FUNCS: &[MuxFunction] = &[];

pub static ESP32S3_PINS: &[PinSpec] =
    &[
        gpio_pin!(
            0,
            "GPIO0",
            ESP_GPIO_CAP,
            GP0_FUNCS,
            StrappingRequirement::MustBeHigh {
                purpose: "SPI Flash Boot mode",
                violation_consequence: "holding GPIO0 LOW at reset forces ROM download bootloader",
            }
        ),
        gpio_pin!(
            1,
            "GPIO1",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 0), 0, "ADC1_CH0")]
        ),
        gpio_pin!(
            2,
            "GPIO2",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 1), 0, "ADC1_CH1")]
        ),
        gpio_pin!(
            3,
            "GPIO3",
            ESP_ADC_CAP,
            GP3_FUNCS,
            StrappingRequirement::MustBeLow {
                purpose: "JTAG signal selection",
                violation_consequence: "holding GPIO3 HIGH at reset alters JTAG routing",
            }
        ),
        gpio_pin!(
            4,
            "GPIO4",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 3), 0, "ADC1_CH3")]
        ),
        gpio_pin!(
            5,
            "GPIO5",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 4), 0, "ADC1_CH4")]
        ),
        gpio_pin!(
            6,
            "GPIO6",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 5), 0, "ADC1_CH5")]
        ),
        gpio_pin!(
            7,
            "GPIO7",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 6), 0, "ADC1_CH6")]
        ),
        gpio_pin!(
            8,
            "GPIO8",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 7), 0, "ADC1_CH7")]
        ),
        gpio_pin!(
            9,
            "GPIO9",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 8), 0, "ADC1_CH8")]
        ),
        gpio_pin!(
            10,
            "GPIO10",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(1, 9), 0, "ADC1_CH9")]
        ),
        gpio_pin!(
            11,
            "GPIO11",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 0), 0, "ADC2_CH0")]
        ),
        gpio_pin!(
            12,
            "GPIO12",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 1), 0, "ADC2_CH1")]
        ),
        gpio_pin!(
            13,
            "GPIO13",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 2), 0, "ADC2_CH2")]
        ),
        gpio_pin!(
            14,
            "GPIO14",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 3), 0, "ADC2_CH3")]
        ),
        gpio_pin!(
            15,
            "GPIO15",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 4), 0, "ADC2_CH4")]
        ),
        gpio_pin!(
            16,
            "GPIO16",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 5), 0, "ADC2_CH5")]
        ),
        gpio_pin!(
            17,
            "GPIO17",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 6), 0, "ADC2_CH6")]
        ),
        gpio_pin!(
            18,
            "GPIO18",
            ESP_ADC_CAP,
            &[mux!(AdcChannel(2, 7), 0, "ADC2_CH7")]
        ),
        gpio_pin!(
            19,
            "GPIO19",
            ESP_ADC_CAP,
            &[
                mux!(AdcChannel(2, 8), 0, "ADC2_CH8"),
                mux!(PeripheralSignal::UsbDm, 0, "USB_DM")
            ]
        ),
        gpio_pin!(
            20,
            "GPIO20",
            ESP_ADC_CAP,
            &[
                mux!(AdcChannel(2, 9), 0, "ADC2_CH9"),
                mux!(PeripheralSignal::UsbDp, 0, "USB_DP")
            ]
        ),
        gpio_pin!(21, "GPIO21", ESP_GPIO_CAP, EMPTY_FUNCS),
        // GPIO 26-32: Reserved for SPI Flash
        gpio_pin!(26, "GPIO26", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(27, "GPIO27", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(28, "GPIO28", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(29, "GPIO29", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(30, "GPIO30", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(31, "GPIO31", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(32, "GPIO32", ESP_FLASH_CAP, EMPTY_FUNCS),
        // GPIO 33-37: Reserved for Octal Flash / PSRAM
        gpio_pin!(33, "GPIO33", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(34, "GPIO34", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(35, "GPIO35", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(36, "GPIO36", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(37, "GPIO37", ESP_FLASH_CAP, EMPTY_FUNCS),
        gpio_pin!(38, "GPIO38", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(39, "GPIO39", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(40, "GPIO40", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(41, "GPIO41", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(42, "GPIO42", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(43, "GPIO43", ESP_GPIO_CAP, GP43_FUNCS),
        gpio_pin!(44, "GPIO44", ESP_GPIO_CAP, GP44_FUNCS),
        gpio_pin!(
            45,
            "GPIO45",
            ESP_GPIO_CAP,
            GP45_FUNCS,
            StrappingRequirement::MustBeLow {
                purpose: "VDD_SPI Flash voltage selection",
                violation_consequence:
                    "holding GPIO45 HIGH selects 1.8V flash and browns out standard 3.3V modules",
            }
        ),
        gpio_pin!(46, "GPIO46", ESP_INPUT_ONLY, GP46_FUNCS, StrappingRequirement::MustBeLow {
            purpose: "ROM boot log output",
            violation_consequence:
                "holding GPIO46 HIGH suppresses ROM boot messages; pin cannot be used as an output",
        }),
        gpio_pin!(47, "GPIO47", ESP_GPIO_CAP, EMPTY_FUNCS),
        gpio_pin!(48, "GPIO48", ESP_GPIO_CAP, EMPTY_FUNCS),
    ];

pub static ESP32S3: SiliconTarget = SiliconTarget {
    name: "esp32-s3",
    family: "Espressif",
    pins: ESP32S3_PINS,
};
