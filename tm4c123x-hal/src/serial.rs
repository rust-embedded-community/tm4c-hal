//! Serial

// uart_hal_macro can be called with too-many arguments
#![allow(clippy::too_many_arguments)]

pub use tm4c123x::{UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7};
pub use tm4c_hal::serial::NewlineMode;
use tm4c_hal::{uart_hal_macro, uart_pin_macro};

#[rustfmt::skip]
use crate::{
    gpio::{
        gpioa, gpiob, gpioc, gpiod, gpioe, gpiof,
        AlternateFunction, OutputMode, AF1, AF2, AF8,
    },
    hal::{prelude::*, serial},
    sysctl,
    sysctl::Clocks,
    time::Bps,
};
use core::{fmt, marker::PhantomData};
use nb::{self, block};
use void::Void;

/// Serial abstraction
pub struct Serial<UART, TX, RX, RTS, CTS> {
    uart: UART,
    tx_pin: TX,
    rx_pin: RX,
    rts_pin: RTS,
    cts_pin: CTS,
    nl_mode: NewlineMode,
}

/// Serial receiver
pub struct Rx<UART, RX, CTS> {
    _uart: PhantomData<UART>,
    pin: RX,
    flow_pin: CTS,
}

/// Serial transmitter
pub struct Tx<UART, TX, RTS> {
    uart: UART,
    pin: TX,
    flow_pin: RTS,
    nl_mode: NewlineMode,
}

uart_pin_macro!(UART0,
    cts: [],
    rts: [],
    rx: [(gpioa::PA0, AF1)],
    tx: [(gpioa::PA1, AF1)],
);

uart_pin_macro!(UART1,
    cts: [(gpioc::PC5, AF8), (gpiof::PF1, AF1)],
    rts: [(gpioc::PC4, AF8), (gpiof::PF0, AF1)],
    rx: [(gpiob::PB0, AF1), (gpioc::PC4, AF2)],
    tx: [(gpiob::PB1, AF1), (gpioc::PC5, AF2)],
);

uart_pin_macro!(UART2,
    cts: [],
    rts: [],
    rx: [(gpiod::PD6, AF1)],
    tx: [(gpiod::PD7, AF1)],
);

uart_pin_macro!(UART3,
    cts: [],
    rts: [],
    rx: [(gpioc::PC6, AF1)],
    tx: [(gpioc::PC7, AF1)],
);

uart_pin_macro!(UART4,
    cts: [],
    rts: [],
    rx: [(gpioc::PC4, AF1)],
    tx: [(gpioc::PC5, AF1)],
);

uart_pin_macro!(UART5,
    cts: [],
    rts: [],
    rx: [(gpioe::PE4, AF1)],
    tx: [(gpioe::PE5, AF1)],
);

uart_pin_macro!(UART6,
    cts: [],
    rts: [],
    rx: [(gpiod::PD4, AF1)],
    tx: [(gpiod::PD5, AF1)],
);

uart_pin_macro!(UART7,
    cts: [],
    rts: [],
    rx: [(gpioe::PE0, AF1)],
    tx: [(gpioe::PE1, AF1)],
);

uart_hal_macro! {
    UART0: (Uart0, uart0),
    UART1: (Uart1, uart1),
    UART2: (Uart2, uart2),
    UART3: (Uart3, uart3),
    UART4: (Uart4, uart4),
    UART5: (Uart5, uart5),
    UART6: (Uart6, uart6),
    UART7: (Uart7, uart7),
}
