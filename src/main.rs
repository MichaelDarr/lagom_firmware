#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![allow(internal_features)]
#![deny(unsafe_op_in_unsafe_fn)]

mod descriptor;
mod keymap;
mod std_stub;

use arduino_hal::{
    pac::PLL,
    port::{
        mode::{Input, Output, PullUp},
        Pin,
    },
};
use atmega_usbd::{SuspendNotifier, UsbBus};
use avr_device::{
    atmega32u4::TC1,
    interrupt,
    
};
use descriptor::{LayoutKey, COLUMN_COUNT, ROW_COUNT};
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
    // http://www.linux-usb.org/usb.ids
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x0738, 0x4588))
        .manufacturer("sthlm kb.")
        .product("Lagom")
        .build();

    
    let clock = TC1Clock::new(dp.TC1);
    let debounce_origin = clock.try_now();

    unsafe {
        USB_CTX = Some(UsbContext {
            usb_device,
            clock: clock,
            debounce_origin,
            hid_class,
            indicator: pins.led_rx.into_output().downgrade(),
            keyboard: keymap::KEYBOARD,
            mux0: pins.d6.into_output().downgrade(),
            mux1: pins.d7.into_output().downgrade(),
            mux2: pins.d8.into_output().downgrade(),
            mux3: pins.d9.into_output().downgrade(),
            rotarty_encoders: [
                RotaryEncoder::new(
                    pins.d4.into_pull_up_input().downgrade(),
                    pins.d5.into_pull_up_input().downgrade(),
                ),
                RotaryEncoder::new(
                    pins.a3.into_pull_up_input().downgrade(),
                    pins.a2.into_pull_up_input().downgrade(),
                ),
            ],
            rows: [
                pins.d10.into_pull_up_input().downgrade(),
                pins.d14.into_pull_up_input().downgrade(),
                pins.d16.into_pull_up_input().downgrade(),
                pins.d15.into_pull_up_input().downgrade(),
                pins.a0.into_pull_up_input().downgrade(),
            ],
            cur_report: BLANK_REPORT,
            next_report: BLANK_REPORT,
        });
    }

    unsafe { interrupt::enable() };

    loop {}
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
    debounce_origin: u16,
    clock: TC1Clock,
    hid_class: HIDClass<'static, UsbBus<S>>,
    indicator: Pin<Output>,
    keyboard: descriptor::Keyboard,
    mux0: Pin<Output>,
    mux1: Pin<Output>,
    mux2: Pin<Output>,
    mux3: Pin<Output>,
    rows: [Pin<Input<PullUp>>; ROW_COUNT],
    rotarty_encoders: [RotaryEncoder; 2],
    cur_report: KeyboardReport,
    next_report: KeyboardReport,
}

const BLANK_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0; 6],
};

const ROLLOVER_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [LayoutKey::URol as u8, 0, 0, 0, 0, 0],
};

const MUX_COLUMN_COUNT: usize = COLUMN_COUNT / 2;

impl<S: SuspendNotifier> UsbContext<S> {
    fn poll(&mut self) {
        let mut report = BLANK_REPORT;
        let mut report_keycode_idx: usize = 0;

        let encoder_right_res = self.rotarty_encoders[0].poll();
        if encoder_right_res.is_some() {
            let mut left_rotary_report = BLANK_REPORT;
            if encoder_right_res.unwrap() == RotaryEncoderDirection::Clockwise {
                left_rotary_report.keycodes[0] = self.keyboard.left_rotary_encoder.clockwise as u8;
            } else {
                left_rotary_report.keycodes[0] =
                    self.keyboard.left_rotary_encoder.counter_clockwise as u8;
            }
            self.hid_class.push_input(&left_rotary_report).ok();
        }

        let encoder_left_res = self.rotarty_encoders[1].poll();
        if encoder_left_res.is_some() {
            let mut right_rotary_report = BLANK_REPORT;
            if encoder_left_res.unwrap() == RotaryEncoderDirection::Clockwise {
                right_rotary_report.keycodes[0] =
                    self.keyboard.right_rotary_encoder.clockwise as u8;
            } else {
                right_rotary_report.keycodes[0] =
                    self.keyboard.right_rotary_encoder.counter_clockwise as u8;
            }
            self.hid_class.push_input(&right_rotary_report).ok();
        }

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
                if self.rows[row].is_low() && report_keycode_idx < 6 {
                    // If the key is a modifier, apply the appropriate bitmask instead of recording it as a keystroke.
                    // https://wiki.osdev.org/USB_Human_Interface_Devices#Report_format
                    match self.keyboard.layout[row][col] {
                        LayoutKey::CtrL => report.modifier |= 0b0000_0001,
                        LayoutKey::SftL => report.modifier |= 0b0000_0010,
                        LayoutKey::AltL => report.modifier |= 0b0000_0100,
                        LayoutKey::GuiL => report.modifier |= 0b0000_1000,
                        LayoutKey::CtrR => report.modifier |= 0b0001_0000,
                        LayoutKey::SftR => report.modifier |= 0b0010_0000,
                        LayoutKey::AltR => report.modifier |= 0b0100_0000,
                        LayoutKey::GuiR => report.modifier |= 0b1000_0000,
                        _ => {
                            report.keycodes[report_keycode_idx] =
                                self.keyboard.layout[row][col] as u8;
                            report_keycode_idx += 1;
                        }
                    }
                }
            }
        }

        if report_keycode_idx > 5 {
            report = ROLLOVER_REPORT;
        }

        // Keep sending the current report until the next report has remained stable for 5 ms, then begin sending that.
        let now = self.clock.try_now();

        let diff = if now >= self.debounce_origin {
            now - self.debounce_origin
        } else {
            (u16::MAX - self.debounce_origin) + now
        };

        // roughly 5ms
        if diff < 300 {
            self.hid_class.push_input(&self.cur_report).ok();
            if !reports_are_equal(self.next_report, report) {
                self.next_report = report;
                self.debounce_origin = now
            }
        } else {
            self.hid_class.push_input(&self.next_report).ok();
            self.cur_report = self.next_report;
            self.next_report = report;
            self.debounce_origin = now
        }

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

// This system returns `true` if the following are identical for the provided reports:
// * Modifier value
// * Keycode array (order-sensitive)
fn reports_are_equal(a: KeyboardReport, b: KeyboardReport) -> bool {
    if a.modifier != b.modifier {
        return false;
    }
    for i in 0..6 {
        if a.keycodes[i] != b.keycodes[i] {
            return false;
        }
    }
    true
}

struct RotaryEncoder {
    a: Pin<Input<PullUp>>,
    b: Pin<Input<PullUp>>,
    progress: RotaryEncoderProgress,
    state: [bool; 2],
}

#[derive(PartialEq, Eq)]
enum RotaryEncoderProgress {
    AwaitingA,
    AwaitingB,
    ReadyToFire,
    Idle,
}

#[derive(PartialEq, Eq)]
enum RotaryEncoderDirection {
    Clockwise,
    CounterClockwise,
}

impl RotaryEncoder {
    pub fn new(a: Pin<Input<PullUp>>, b: Pin<Input<PullUp>>) -> RotaryEncoder {
        let a_state = a.is_low();
        let b_state = b.is_low();
        RotaryEncoder {
            a,
            b,
            progress: RotaryEncoderProgress::Idle,
            state: [a_state, b_state],
        }
    }

    pub fn poll(&mut self) -> Option<RotaryEncoderDirection> {
        let a_state = self.a.is_low();
        let b_state = self.b.is_low();
        if a_state != self.state[0] || b_state != self.state[1] {
            if !a_state && !b_state {
                if self.progress == RotaryEncoderProgress::ReadyToFire {
                    self.progress = RotaryEncoderProgress::Idle;
                    let direction = if !self.state[0] {
                        RotaryEncoderDirection::CounterClockwise
                    } else {
                        RotaryEncoderDirection::Clockwise
                    };
                    self.state = [a_state, b_state];
                    return Some(direction);
                }
                self.progress = RotaryEncoderProgress::Idle;
            } else if !a_state {
                if self.progress == RotaryEncoderProgress::AwaitingA {
                    self.progress = RotaryEncoderProgress::ReadyToFire;
                } else if self.progress == RotaryEncoderProgress::ReadyToFire {
                    self.progress = RotaryEncoderProgress::AwaitingA;
                } else if self.progress == RotaryEncoderProgress::Idle {
                    self.progress = RotaryEncoderProgress::AwaitingB;
                }
            } else if !b_state {
                if self.progress == RotaryEncoderProgress::AwaitingB {
                    self.progress = RotaryEncoderProgress::ReadyToFire;
                } else if self.progress == RotaryEncoderProgress::ReadyToFire {
                    self.progress = RotaryEncoderProgress::AwaitingB;
                } else if self.progress == RotaryEncoderProgress::Idle {
                    self.progress = RotaryEncoderProgress::AwaitingA;
                }
            }

            self.state = [a_state, b_state];
        }
        None
    }
}

struct TC1Clock {
    tc1: TC1,
}

impl TC1Clock {
    pub fn new(tc1: TC1) -> TC1Clock {
        //          100 | Clock source       | clk/256 (prescaled)
        //       0_0    | Mode               | normal mode (go up to 0xFFFF, then roll around to 0)
        //      0       | n/a                | reserved
        //     0        | Input capture edge | falling edge used as trigger on input capture pin (ICPn)
        //    0         | Noise canceler     | disabled
        tc1.tccr1b.write(|w| unsafe { w.bits(
            0b0000_0100
        )});
        TC1Clock {
            tc1,
        }
    }
}

impl TC1Clock {
    fn try_now(&self) -> u16 {
        self.tc1.tcnt1.read().bits()
    }
}
