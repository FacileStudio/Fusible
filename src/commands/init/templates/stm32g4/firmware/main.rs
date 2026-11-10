//! Blinks the board LED. `board.rs` and `pins.rs` are generated from the board netlist
//! by `fusible build`: move the LED in `hardware/index.tsx` and this code follows.

#![no_std]
#![no_main]

mod board;
mod pins;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = board::take!(p);
    let mut led = Output::new(board.led, Level::Low, Speed::Low);
    defmt::info!("blinking the LED on pin {}", pins::gpio::LED);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
