//! HAL for the tm4c129x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the tm4c129x
//! family of microcontrollers.
//!
//! [`embedded-hal`]: https://github.com/japaric/embedded-hal
//!
//! # Usage
//!
//! To build applications (binary crates) using this crate follow the
//! [cortex-m-quickstart] instructions and add this crate as a dependency in
//! step number 5 and make sure you enable the "rt" Cargo feature of this crate.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.2.3
//!
//! # Examples
//!
//! Examples of *using* these abstractions like these can be found in the
//! documentation of the [`f3`] crate.
//!
//! [`f3`]: https://docs.rs/f3/~0.5.1

#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(deprecated)]

pub use tm4c129x::{self, CorePeripherals, Peripherals};
pub use tm4c_hal::{bb, delay, time};

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::tm4c129x::interrupt;

use sealed::Sealed;
mod sealed {
    // To prevent implementation of `*Pin` traits on arbitrary types
    pub trait Sealed {}

    impl Sealed for () {}
}

pub mod gpio;
pub mod hib;
pub mod i2c;
pub mod prelude;
pub mod serial;
// pub mod spi;
pub mod sysctl;

use embedded_hal as hal;
