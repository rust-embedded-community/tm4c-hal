# `tm4c123x-hal`

> HAL for the TM4C123x (and compatible LM4F120x) family of microcontrollers

[`embedded-hal`]: https://crates.io/crates/embedded-hal

## [Documentation](https://docs.rs/tm4c123x-hal)

## Changelog

### Unreleased Changes ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/master/tm4c123x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c123x-hal-0.10.3...master))

* Use sealed traits for `*Pin` marker traits
* Do not reexport `tm4c-hal` macros
* Updated the dependencies for the supporting crate `tm4c123x` to
`0.9.1` which supports newer version of `cortex-m`. This version can be used with newer
versions of RTIC and has been tested in hardware (Launchpad and custom PCB)
using RTIC `1.1.4`. Testing included SPI, ADC, Timers, EEPROM, GPIO, UART,
and multiple interrupts (UART, GPIO, TIMERS, ADC).

### v0.10.2 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c123x-hal-0.10.2/tm4c123x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c123x-hal-0.10.2...tm4c123x-hal-0.10.1))

* Updated to tm4c-hal 0.4.1

### v0.10.1 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c123x-hal-0.10.1/tm4c123x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c123x-hal-0.10.1...tm4c123x-hal-0.10.0))

* Updated to tm4c-hal 0.4.0

### v0.10.0 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c123x-hal-0.10.0/tm4c123x-hal))

* Changelog starts here

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
