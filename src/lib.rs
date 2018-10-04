//! HAL for the tm4c123x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the tm4c123x family of
//! microcontrollers.
//!
//! [`embedded-hal`]: https://github.com/japaric/embedded-hal
//!
//! # Usage
//!
//! To build applications (binary crates) using this crate follow the [cortex-m-quickstart]
//! instructions and add this crate as a dependency in step number 5 and make sure you enable the
//! "rt" Cargo feature of this crate.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.2.3
//!
//! # Examples
//!
//! Examples of *using* these abstractions like these can be found in the documentation of the [`f3`] crate.
//!
//! [`f3`]: https://docs.rs/f3/~0.5.1

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub mod bb;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod prelude;
pub mod serial;
pub mod spi;
pub mod sysctl;
pub mod time;

use embedded_hal as hal;
pub use tm4c123x;
pub use tm4c123x::*;
