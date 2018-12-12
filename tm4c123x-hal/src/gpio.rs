//! General Purpose Input / Output
//!
//! This module makes heavy use of types to try and ensure you can't have a
//! pin in a mode you didn't expect.
//!
//! Most pins start in the `Tristate` state. You can call methods to convert
//! them to inputs, outputs or put them into Alternate Function mode (e.g. to
//! use with a UART).
//!
//! Some of the modes require extra information, and for that we use the so-
//! called 'Turbo Fish` syntax, which looks like `method::<TYPE>`.
//!
//! If the operation is non-atomic, then you need to pass a mut-reference to
//! the port's control structure. This ensures you can't change two pins in
//! two threads at the same time. If the operation is fully atomic (using the
//! chip's bit-banding feature) then this argument is not required.
//!
//! Here's an example:
//!
//! ```
//! # use tm4c123x_hal::*;
//! # use tm4c123x_hal::sysctl::SysctlExt;
//! # use tm4c123x_hal::gpio::GpioExt;
//! # fn foo() {
//! let p = Peripherals::take().unwrap();
//! let mut sc = p.SYSCTL.constrain();
//! let mut portb = p.GPIO_PORTB.split(&sc.power_control);
//! let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
//! let uart_tx_pin = portb.pb1.into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
//! let blue_led = portb.pb2.into_push_pull_output();
//! let button = portb.pb3.into_pull_up_input();
//! # }
//! ```

pub use tm4c_hal::gpio::*;

use tm4c_hal::gpio_macro;
use core::marker::PhantomData;
use crate::bb;
use crate::hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use crate::sysctl;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, power_control: &sysctl::PowerControl) -> Self::Parts;
}

gpio_macro!(tm4c123x, GPIO_PORTA, gpioa, GpioA, PAx, [
    PA0: (pa0, 0, Tristate),
    PA1: (pa1, 1, Tristate),
    PA2: (pa2, 2, Tristate),
    PA3: (pa3, 3, Tristate),
    PA4: (pa4, 4, Tristate),
    PA5: (pa5, 5, Tristate),
    PA6: (pa6, 6, Tristate),
    PA7: (pa7, 7, Tristate),
]);

gpio_macro!(tm4c123x, GPIO_PORTB, gpiob, GpioB, PBx, [
    PB0: (pb0, 0, Tristate),
    PB1: (pb1, 1, Tristate),
    PB2: (pb2, 2, Tristate),
    PB3: (pb3, 3, Tristate),
    PB4: (pb4, 4, Tristate),
    PB5: (pb5, 5, Tristate),
    PB6: (pb6, 6, Tristate),
    PB7: (pb7, 7, Tristate),
]);

gpio_macro!(tm4c123x, GPIO_PORTC, gpioc, GpioC, PCx, [
    PC0: (pc0, 0, Locked), // JTAG/SWD pin
    PC1: (pc1, 1, Locked), // JTAG/SWD pin
    PC2: (pc2, 2, Locked), // JTAG/SWD pin
    PC3: (pc3, 3, Locked), // JTAG/SWD pin
    PC4: (pc4, 4, Tristate),
    PC5: (pc5, 5, Tristate),
    PC6: (pc6, 6, Tristate),
    PC7: (pc7, 7, Tristate),
]);

gpio_macro!(tm4c123x, GPIO_PORTD, gpiod, GpioD, PDx, [
    PD0: (pd0, 0, Tristate),
    PD1: (pd1, 1, Tristate),
    PD2: (pd2, 2, Tristate),
    PD3: (pd3, 3, Tristate),
    PD4: (pd4, 4, Tristate),
    PD5: (pd5, 5, Tristate),
    PD6: (pd6, 6, Tristate),
    PD7: (pd7, 7, Locked), // NMI pin
]);

gpio_macro!(tm4c123x, GPIO_PORTE, gpioe, GpioE, PEx, [
    PE0: (pe0, 0, Tristate),
    PE1: (pe1, 1, Tristate),
    PE2: (pe2, 2, Tristate),
    PE3: (pe3, 3, Tristate),
    PE4: (pe4, 4, Tristate),
    PE5: (pe5, 5, Tristate),
    PE6: (pe6, 6, Tristate),
    PE7: (pe7, 7, Tristate),
]);

gpio_macro!(tm4c123x, GPIO_PORTF, gpiof, GpioF, PFx, [
    PF0: (pf0, 0, Locked), // NMI pin
    PF1: (pf1, 1, Tristate),
    PF2: (pf2, 2, Tristate),
    PF3: (pf3, 3, Tristate),
    PF4: (pf4, 4, Tristate),
    PF5: (pf5, 5, Tristate),
    PF6: (pf6, 6, Tristate),
    PF7: (pf7, 7, Tristate),
]);
