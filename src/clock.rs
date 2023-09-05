pub use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub type DefaultClock = avr_hal_generic::clock::MHz16;
pub type Delay = avr_hal_generic::delay::Delay<DefaultClock>;
