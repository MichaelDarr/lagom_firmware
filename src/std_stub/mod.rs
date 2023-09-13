//! Replacement for avr-std-stub with a custom panic handler.

use core::panic::PanicInfo;

use arduino_hal::{delay_ms, pins, Peripherals};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let dp = unsafe { Peripherals::steal() };
    let pins = pins!(dp);

    // Flash the LED to indicate a panic
    let mut led = pins.led_tx.into_output();
    loop {
        led.toggle();
        delay_ms(500);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}
