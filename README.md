# Experimental Keyboard Firmware

Lagom + ProMicro (ATmega32U4) + Rust

## Usage

1. Install the [`avr-hal` prerequisites](https://github.com/Rahix/avr-hal#quickstart).
2. Build firmware: `cargo build`
3. Flash to a connected board: `cargo run` (after flashing, `ravedude` automatically opens a UART console session)

## About

This project's foundation is [`avr-hal`](https://github.com/Rahix/avr-hal). It bootstrapped using [`avr-hal-template`](https://github.com/Rahix/avr-hal-template)(recommended by the [official docs](https://github.com/Rahix/avr-hal#starting-your-own-project)).
