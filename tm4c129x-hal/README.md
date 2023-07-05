# `tm4c129x-hal`

> HAL for the TM4C129x family of microcontrollers

[`embedded-hal`]: https://crates.io/crates/embedded-hal

## [Documentation](https://docs.rs/tm4c129x-hal)

## Changelog

* Update 0.9.3 - Updated the dependencies for the supporting crate tm4c129x to 
0.9.1 which supports newer version of cortex-m. This _should_ allow for running
newer version of RTIC / cortex-m, however, unlike the tm4c123 this hasn't been 
tested.

### Unreleased Changes ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/master/tm4c129x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c129x-hal-0.9.3...master))

* Use sealed traits for `*Pin` marker traits
* Do not reexport `tm4c-hal` macros

### v0.9.2 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c129x-hal-0.9.2/tm4c129x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c129x-hal-0.9.2...tm4c129x-hal-0.9.1))

* Updated to tm4c-hal 0.4.1

### v0.9.1 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c129x-hal-0.9.1/tm4c129x-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c129x-hal-0.9.1...tm4c129x-hal-0.9.0))

* Updated to tm4c-hal 0.4.0

### v0.9.0 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c129x-hal-0.9.0/tm4c129x-hal))

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
