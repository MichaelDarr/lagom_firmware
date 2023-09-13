#![no_std]
#![no_main]

#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

#![allow(internal_features)]
#![deny(unsafe_op_in_unsafe_fn)]

mod std_stub;
mod keymap;
mod descriptor;

use arduino_hal::{
    pac::PLL,
    port::{
        mode::{Input, Output, PullUp},
        Pin,
    },
};
use atmega_usbd::{SuspendNotifier, UsbBus};
use avr_device::{asm::sleep, interrupt};
use descriptor::{COLUMN_COUNT, LayoutKey, ROW_COUNT};
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

    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());
    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());
    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus<PLL>>> = None;
        &*USB_BUS.insert(UsbBus::with_suspend_notifier(usb, pll))
    };
    let hid_class = HIDClass::new(usb_bus, KeyboardReport::desc(), 1);
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("sthlm kb.")
        .product("Lagom")
        .build();

    unsafe {
        USB_CTX = Some(UsbContext {
            usb_device,
            hid_class,
            indicator: pins.led_rx.into_output().downgrade(),
            layout: keymap::LAYOUT,
            mux0: pins.d6.into_output().downgrade(),
            mux1: pins.d7.into_output().downgrade(),
            mux2: pins.d8.into_output().downgrade(),
            mux3: pins.d9.into_output().downgrade(),
            rows: [
                pins.d10.into_pull_up_input().downgrade(),
                pins.d14.into_pull_up_input().downgrade(),
                pins.d16.into_pull_up_input().downgrade(),
                pins.d15.into_pull_up_input().downgrade(),
                pins.a0.into_pull_up_input().downgrade(),
            ],
        });
    }

    unsafe { interrupt::enable() };

    loop {
        sleep();
    }
}

static mut USB_CTX: Option<UsbContext<PLL>> = None;

#[interrupt(atmega32u4)]
fn USB_GEN() {
    unsafe { poll_usb() };
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    unsafe { poll_usb() };
}

/// # Safety
///
/// This function assumes that it is being called within an
/// interrupt context.
unsafe fn poll_usb() {
    // Safety: There must be no other overlapping borrows of USB_CTX.
    // - By the safety contract of this function, we are in an interrupt
    //   context.
    // - The main thread is not borrowing USB_CTX. The only access is the
    //   assignment during initialization. It cannot overlap because it is
    //   before the call to `interrupt::enable()`.
    // - No other interrupts are accessing USB_CTX, because no other interrupts
    //   are in the middle of execution. GIE is automatically cleared for the
    //   duration of the interrupt, and is not re-enabled within any ISRs.
    let ctx = unsafe { USB_CTX.as_mut().unwrap() };
    ctx.poll();
}

struct UsbContext<S: SuspendNotifier> {
    usb_device: UsbDevice<'static, UsbBus<S>>,
    hid_class: HIDClass<'static, UsbBus<S>>,
    indicator: Pin<Output>,
    layout: descriptor::LayoutGrid,
    mux0: Pin<Output>,
    mux1: Pin<Output>,
    mux2: Pin<Output>,
    mux3: Pin<Output>,
    rows: [Pin<Input<PullUp>>; ROW_COUNT],
}

const BLANK_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0; 6],
};

const MUX_COLUMN_COUNT: usize = COLUMN_COUNT / 2;

impl<S: SuspendNotifier> UsbContext<S> {
    fn poll(&mut self) {
        let mut report = BLANK_REPORT;
        let mut report_keycode_idx: usize = 0;

        // Start with demultiplexer C2 (cols 0-7)
        self.mux3.set_low();

        // Iterate over each matrix column
        for col in 0..COLUMN_COUNT {
            // After reading the first set, switch to the demultiplexer C1 (cols 8-15)
            if col == MUX_COLUMN_COUNT {
                self.mux3.set_high();
            }

            // Target the column's position by passing its binary representation to the demultiplexer 
            let pos = col % MUX_COLUMN_COUNT;
            if (pos & 1) == 1 {
                self.mux0.set_high();
            } else {
                self.mux0.set_low();
            }
            if ((pos >> 1) & 1) == 1 {
                self.mux1.set_high();
            } else {
                self.mux1.set_low();
            }
            if ((pos >> 2) & 1) == 1 {
                self.mux2.set_high();
            } else {
                self.mux2.set_low();
            }

            // Check for active rows within the column
            for row in 0..ROW_COUNT {
                if self.rows[row].is_low() {
                    // If the key is a modifier, apply the appropriate bitmask instead of recording it as a keystroke.
                    // https://wiki.osdev.org/USB_Human_Interface_Devices#Report_format
                    match self.layout[row][col] {
                        LayoutKey::CtrL => report.modifier |= 0b0000_0001,
                        LayoutKey::SftL => report.modifier |= 0b0000_0010,
                        LayoutKey::AltL => report.modifier |= 0b0000_0100,
                        LayoutKey::GuiL => report.modifier |= 0b0000_1000,
                        LayoutKey::CtrR => report.modifier |= 0b0001_0000,
                        LayoutKey::SftR => report.modifier |= 0b0010_0000,
                        LayoutKey::AltR => report.modifier |= 0b0100_0000,
                        LayoutKey::GuiR => report.modifier |= 0b1000_0000,
                        _ => {
                            report.keycodes[report_keycode_idx] = self.layout[row][col] as u8;
                            report_keycode_idx += 1;
                        }
                    }
                }
            }
        }

        self.hid_class.push_input(&report).ok();

        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            let mut report_buf = [0u8; 1];

            if self.hid_class.pull_raw_output(&mut report_buf).is_ok() {
                if report_buf[0] & 2 != 0 {
                    self.indicator.set_high();
                } else {
                    self.indicator.set_low();
                }
            }
        }
    }
}
