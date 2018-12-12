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
//! # use tm4c129x_hal::*;
//! # use tm4c129x_hal::sysctl::SysctlExt;
//! # use tm4c129x_hal::gpio::GpioExt;
//! # fn foo() {
//! let p = Peripherals::take().unwrap();
//! let mut sc = p.SYSCTL.constrain();
//! let mut portb = p.GPIO_PORTB.split(&sc.power_control);
//! let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
//! let uart_tx_pin = portb
//!     .pb1
//!     .into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
//! let blue_led = portb.pb2.into_push_pull_output();
//! let button = portb.pb3.into_pull_up_input();
//! # }
//! ```

pub use tm4c_hal::gpio::*;

use tm4c_hal::gpio_macro;
use crate::bb;
use crate::hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use crate::sysctl;
use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, power_control: &sysctl::PowerControl) -> Self::Parts;
}

gpio_macro!(tm4c129x, GPIO_PORTA_AHB, gpioa, GpioA, PAx, [
    PA0: (pa0, 0, Tristate),
    PA1: (pa1, 1, Tristate),
    PA2: (pa2, 2, Tristate),
    PA3: (pa3, 3, Tristate),
    PA4: (pa4, 4, Tristate),
    PA5: (pa5, 5, Tristate),
    PA6: (pa6, 6, Tristate),
    PA7: (pa7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTB_AHB, gpiob, GpioB, PBx, [
    PB0: (pb0, 0, Tristate),
    PB1: (pb1, 1, Tristate),
    PB2: (pb2, 2, Tristate),
    PB3: (pb3, 3, Tristate),
    PB4: (pb4, 4, Tristate),
    PB5: (pb5, 5, Tristate),
    // PB6 and PB7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTC_AHB, gpioc, GpioC, PCx, [
    PC0: (pc0, 0, Locked), // JTAG/SWD pin
    PC1: (pc1, 1, Locked), // JTAG/SWD pin
    PC2: (pc2, 2, Locked), // JTAG/SWD pin
    PC3: (pc3, 3, Locked), // JTAG/SWD pin
    PC4: (pc4, 4, Tristate),
    PC5: (pc5, 5, Tristate),
    PC6: (pc6, 6, Tristate),
    PC7: (pc7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTD_AHB, gpiod, GpioD, PDx, [
    PD0: (pd0, 0, Tristate),
    PD1: (pd1, 1, Tristate),
    PD2: (pd2, 2, Tristate),
    PD3: (pd3, 3, Tristate),
    PD4: (pd4, 4, Tristate),
    PD5: (pd5, 5, Tristate),
    PD6: (pd6, 6, Tristate),
    PD7: (pd7, 7, Locked), // GPIO pin
]);

gpio_macro!(tm4c129x, GPIO_PORTE_AHB, gpioe, GpioE, PEx, [
    PE0: (pe0, 0, Tristate),
    PE1: (pe1, 1, Tristate),
    PE2: (pe2, 2, Tristate),
    PE3: (pe3, 3, Tristate),
    PE4: (pe4, 4, Tristate),
    PE5: (pe5, 5, Tristate),
    // PE6 and PE7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTF_AHB, gpiof, GpioF, PFx, [
    PF0: (pf0, 0, Tristate),
    PF1: (pf1, 1, Tristate),
    PF2: (pf2, 2, Tristate),
    PF3: (pf3, 3, Tristate),
    PF4: (pf4, 4, Tristate),
    // PF5, PF6 and PF7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTG_AHB, gpiog, GpioG, PGx, [
    PG0: (pg0, 0, Tristate),
    PG1: (pg1, 1, Tristate),
    // PG2 through PG7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTH_AHB, gpioh, GpioH, PHx, [
    PH0: (ph0, 0, Tristate),
    PH1: (ph1, 1, Tristate),
    PH2: (ph2, 2, Tristate),
    PH3: (ph3, 3, Tristate),
    // PH4 through PG7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTJ_AHB, gpioj, GpioJ, PJx, [
    PJ0: (pj0, 0, Tristate),
    PJ1: (pj1, 1, Tristate),
    // PJ2 through PJ7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTK, gpiok, GpioK, PKx, [
    PK0: (pk0, 0, Tristate),
    PK1: (pk1, 1, Tristate),
    PK2: (pk2, 2, Tristate),
    PK3: (pk3, 3, Tristate),
    PK4: (pk4, 4, Tristate),
    PK5: (pk5, 5, Tristate),
    PK6: (pk6, 6, Tristate),
    PK7: (pk7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTL, gpiol, GpioL, PNL, [
    PL0: (pl0, 0, Tristate),
    PL1: (pl1, 1, Tristate),
    PL2: (pl2, 2, Tristate),
    PL3: (pl3, 3, Tristate),
    PL4: (pl4, 4, Tristate),
    PL5: (pl5, 5, Tristate),
    PL6: (pl6, 6, Tristate),
    PL7: (pl7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTM, gpiom, GpioM, PMx, [
    PM0: (pm0, 0, Tristate),
    PM1: (pm1, 1, Tristate),
    PM2: (pm2, 2, Tristate),
    PM3: (pm3, 3, Tristate),
    PM4: (pm4, 4, Tristate),
    PM5: (pm5, 5, Tristate),
    PM6: (pm6, 6, Tristate),
    PM7: (pm7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTN, gpion, GpioN, PNx, [
    PN0: (pn0, 0, Tristate),
    PN1: (pn1, 1, Tristate),
    PN2: (pn2, 2, Tristate),
    PN3: (pn3, 3, Tristate),
    PN4: (pn4, 4, Tristate),
    PN5: (pn5, 5, Tristate),
    PN6: (pn6, 6, Tristate),
    PN7: (pn7, 7, Tristate),
]);

gpio_macro!(tm4c129x, GPIO_PORTP, gpiop, GpioP, PPx, [
    PP0: (pp0, 0, Tristate),
    PP1: (pp1, 1, Tristate),
    PP2: (pp2, 2, Tristate),
    PP3: (pp3, 3, Tristate),
    PP4: (pp4, 4, Tristate),
    PP5: (pp5, 5, Tristate),
    // PP6 and PP7 don't exist
]);

gpio_macro!(tm4c129x, GPIO_PORTQ, gpioq, GpioQ, PQx, [
    PQ0: (pq0, 0, Tristate),
    PQ1: (pq1, 1, Tristate),
    PQ2: (pq2, 2, Tristate),
    PQ3: (pq3, 3, Tristate),
    PQ4: (pq4, 4, Tristate),
    // PQ5, PQ6 and PQ7 don't exist
]);
