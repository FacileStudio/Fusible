//! Blinks the board LED. `pins.rs` is generated from the board netlist by `fusible build`:
//! move the LED in `hardware/index.tsx` and this code follows.

#![no_std]
#![no_main]

mod pins;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::gpio::{AnyPin, Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;

esp_bootloader_esp_idf::esp_app_desc!();

/// Takes the GPIO that `pins.rs` gives the `LED` net.
///
/// Safety: `pins.rs` gives every net its own GPIO, and this is the only place the LED pin
/// is taken; no typed `GPIOn` peripheral with the same number is used.
fn led_pin() -> AnyPin<'static> {
    unsafe { AnyPin::steal(pins::gpio::LED) }
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    let mut led = Output::new(led_pin(), Level::Low, OutputConfig::default());
    esp_println::println!("blinking the LED on GPIO{}", pins::gpio::LED);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
