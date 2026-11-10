//! Blinks the board LED. `board.rs` and `pins.rs` are generated from the board netlist
//! by `fusible build`: move the LED in `hardware/index.tsx` and this code follows.
//!
//! `board::take!(p)` moves the board's pins out of `p`; the timer group and the
//! software interrupt the scheduler needs stay in `p`.

#![no_std]
#![no_main]

mod board;
mod pins;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    let p = esp_hal::init(esp_hal::Config::default());
    let board = board::take!(p);
    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0, p.FROM_CPU_INTR0);
    let mut led = Output::new(board.led, Level::Low, OutputConfig::default());
    esp_println::println!("blinking the LED on GPIO{}", pins::gpio::LED);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
