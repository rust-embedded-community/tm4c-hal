# `tm4c129x-hal` and `tm4c123x-hal`

> An [Embedded HAL] (Hardware Abstraction Layer) for the [TM4C129x] and [TM4C123x] families of microcontrollers from Texas Instruments.

[Embedded HAL]: https://crates.io/crates/embedded-hal
[TM4C123x]: https://www.ti.com/product/TM4C123GH6PM
[TM4C129x]: https://www.ti.com/product/TM4C1294NCPDT

These microcontrollers are based on the Arm Cortex-M4F processor core and derived from the earlier TI/Luminary Micro LM4 and LM3 series MCUs. This HAL may work on an LM4F series MCU, but there are no guarantees. A full list of TM4C series microcontrollers is available from TI in [Document SPMT285D](https://www.ti.com/lit/sg/spmt285d/spmt285d.pdf).

## Crates

This repo comprises:

* `tm4c123x-hal` [![tm4c123x-hal version](https://img.shields.io/crates/v/tm4c123x-hal.svg)](https://crates.io/crates/tm4c123x-hal/) - a HAL for the TM4C123GH6PM and related microcontrollers
* `tm4c129x-hal` [![tm4c129x-hal version](https://img.shields.io/crates/v/tm4c129x-hal.svg)](https://crates.io/crates/tm4c129x-hal/) - a HAL for the TM4C1294NCPDT and related microcontrollers
* `tm4c-hal` [![tm4c-hal version](https://img.shields.io/crates/v/tm4c-hal.svg)](https://crates.io/crates/tm4c-hal/) - drivers and HAL implementation that is common to both the above MCU families

## Example Hardware

These crates are tested on the following Tiva-C Launchpad boards:

* Tiva-C Series TM4C123G Launchpad, [EK-TM4C123GXL](https://www.ti.com/tool/EK-TM4C123GXL)
* Tiva-C Series TM4C1294 Connected Launchpad, [EK-TM4C1294XL](https://www.ti.com/tool/EK-TM4C1294XL)
* Tiva-C Series TM4C129E Crypto Launchpad, [EK-TM4C129EXL](http://www.ti.com/tool/EK-TM4C129EXL)

## Example projects

The authors are aware of the following projects which use one or other (or both) of these crates:

* [Monotron](https://github.com/thejpster/monotron), a 1980s-style retro computer with VGA output

## Documentation

See https://docs.rs/tm4c129x-hal and https://docs.rs/tm4c123x-hal.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
