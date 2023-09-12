#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

mod std_stub;

use arduino_hal::{
    entry,
    pac::PLL,
    pins,
    prelude::_void_ResultVoidExt,
    port::{
        mode::{Input, Output, PullUp},
        Pin,
    },
    Peripherals,
};
use atmega_usbd::{SuspendNotifier, UsbBus};
use avr_device::{asm::sleep, interrupt};
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::atmega32u4::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mux0 = pins.d6.into_output();
    let mut mux1 = pins.d7.into_output();
    let mut mux2 = pins.d8.into_output();
    let mut mux3 = pins.d9.into_output();

    let row0 = pins.d10.into_pull_up_input();
    let row1 = pins.d16.into_pull_up_input();
    let row2 = pins.d14.into_pull_up_input();
    let row3 = pins.d15.into_pull_up_input();
    let row4 = pins.a0.into_pull_up_input();

    // Select Y0 (col 0/8)
    mux0.set_low();
    mux1.set_low();
    mux2.set_low();

    // Select C2 (cols 0-7)
    mux3.set_low();

    loop {
        // let values = [
        //     row0.is_low(),
        //     row1.is_low(),
        //     row2.is_low(),
        //     row3.is_low(),
        //     row4.is_low(),
        // ];

        // for (i, v) in values.iter().enumerate() {
        //     ufmt::uwriteln!(&mut serial, "row{}: {}", i, v).void_unwrap()
        // }
        arduino_hal::delay_ms(1000)
    }
}
