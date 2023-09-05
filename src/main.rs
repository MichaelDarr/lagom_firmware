#![no_std]
#![no_main]

const BLINK_DELAY: u16 = 500;

mod clock;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // Example code - blink LED
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.led_rx.into_output();

    loop {
        led.toggle();
        clock::Delay::new().delay_ms(BLINK_DELAY);
    }
}
