# `tm4c-hal`

> Common bits of HAL for the TM4C123x and TM4C129x family of microcontrollers

You probably don't need this crate, you need `tm4c123x-hal` or `tm4c129x-hal`
depending on your processor.

## Changelog

### Unreleased Changes ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/master/tm4c-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c-hal-0.4.2...master))

* Basic EEPROM Read, Write, Erase added
* Updated dependencies in `tm4c123x`, `tm4c129x`, `tm4c123x-hal`, and
`tm4c129x-hal` to use newer version of cortex-m (up to v0.7 as of this release).


### Unreleased Changes ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/master/tm4c-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c-hal-0.4.1...master))

* Implement use of sealed traits by downstream crates (i.e. `tm4c123x-hal` and `tm4c129x-hal`)

### v0.4.1 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c-hal-0.4.1/tm4c-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c-hal-0.4.1...tm4c-hal-0.4.0))

* I2C busy handling fixes

### v0.4.0 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c-hal-0.4.0/tm4c-hal) [Diff](https://github.com/rust-embedded-community/tm4c-hal/compare/tm4c-hal-0.4.0...tm4c-hal-0.3.0))

* I2C fixes

### v0.3.0 ([Source](https://github.com/rust-embedded-community/tm4c-hal/tree/tm4c-hal-0.3.0/tm4c-hal))

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
