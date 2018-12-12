//! Serial

use core::fmt;
use core::marker::PhantomData;

use crate::hal::prelude::*;
use crate::hal::serial;
use nb::{self, block};
pub use tm4c129x::{UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7};
use void::Void;

use crate::gpio::*;
use crate::gpio::{AlternateFunction, OutputMode, AF1};
use crate::sysctl;
use crate::sysctl::Clocks;
use crate::time::Bps;

// FIXME these should be "closed" traits
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

/// CTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait CtsPin<UART> {
    /// Enables the CTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DCD pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DcdPin<UART> {
    /// Enables the DCD functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DSR pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DsrPin<UART> {
    /// Enables the DSR functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DTR pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DtrPin<UART> {
    /// Enables the DTR functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// RI pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RiPin<UART> {
    /// Enables the RI functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// RTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RtsPin<UART> {
    /// Enables the RTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

macro_rules! uart {
    ($UARTn:ident,
        cts: [() $(, ($($ctsgpio: ident)::*, $ctsaf: ident))*],
        // dcd: [() $(, ($($dcdgpio: ident)::*, $dcdaf: ident))*],
        // dsr: [() $(, ($($dsrgpio: ident)::*, $dsraf: ident))*],
        // dtr: [() $(, ($($dtrgpio: ident)::*, $dtraf: ident))*],
        // ri: [() $(, ($($rigpio: ident)::*, $riaf: ident))*],
        rts: [() $(, ($($rtsgpio: ident)::*, $rtsaf: ident))*],
        rx: [$(($($rxgpio: ident)::*, $rxaf: ident)),*],
        tx: [$(($($txgpio: ident)::*, $txaf: ident)),*],
    ) => {
        unsafe impl CtsPin<$UARTn> for () {
            fn enable(&mut self, _uart: &mut $UARTn) {}
        }

        $(
            unsafe impl<T> CtsPin<$UARTn> for $($ctsgpio)::*<AlternateFunction<$ctsaf, T>>
            where
                T: OutputMode,
            {
                fn enable(&mut self, uart: &mut $UARTn) {
                    uart.ctl.modify(|_, w| w.ctsen().set_bit());
                }
            }
        )*

        unsafe impl RtsPin<$UARTn> for () {
            fn enable(&mut self, _uart: &mut $UARTn) {}
        }

        $(
            unsafe impl<T> CtsPin<$UARTn> for $($rtsgpio)::*<AlternateFunction<$rtsaf, T>>
            where
                T: OutputMode,
            {
                fn enable(&mut self, uart: &mut $UARTn) {
                    uart.ctl.modify(|_, w| w.rtsen().set_bit());
                }
            }
        )*

        $(
            unsafe impl <T> RxPin<$UARTn> for $($rxgpio)::*<AlternateFunction<$rxaf, T>>
            where
                T: OutputMode,
            {}
        )*

        $(
            unsafe impl <T> TxPin<$UARTn> for $($txgpio)::*<AlternateFunction<$txaf, T>>
            where
                T: OutputMode,
            {}
        )*
    }
}

uart!(UART0,
    cts: [(), (gpioh::PH1, AF1), (gpiom::PM4, AF1), (gpiob::PB4, AF1)],
    // dcd: [(), (gpioh::PH2, AF1), (gpiom::PM5, AF1), (gpiop::PP3, AF2)],
    // dsr: [(), (gpioh::PH3, AF1), (gpiom::PM6, AF1), (gpiop::PP4, AF2)],
    // dtr: [(), (gpiop::PP2, AF2)],
    // ri: [(), (gpiok::PK7, AF1), (gpiom::PM7, AF1)],
    rts: [(), (gpioh::PH0, AF1), (gpiob::PB5, AF1)],
    rx: [(gpioa::PA0, AF1)],
    tx: [(gpioa::PA1, AF1)],
);

uart!(UART1,
    cts: [(), (gpiop::PP3, AF1), (gpion::PN1, AF1)],
    // dcd: [(), (gpioe::PE2, AF1), (gpion::PN2, AF1)],
    // dsr: [(), (gpioe::PE1, AF1), (gpion::PN3, AF1)],
    // dtr: [(), (gpioe::PE3, AF1), (gpion::PN3, AF1)],
    // ri: [(), (gpion::PN5, AF1), (gpion::PE4, AF1)],
    rts: [(), (gpioe::PE0, AF1), (gpion::PN0, AF1)],
    rx: [(gpiob::PB0, AF1), (gpioq::PQ4, AF1)],
    tx: [(gpiob::PB1, AF1)],
);

uart!(UART2,
    cts: [(), (gpion::PN3, AF2), (gpiod::PD7, AF1)],
    // dcd: [(), (gpion::PN2, AF2), (gpiod::PD6, AF1)],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [(gpioa::PA6, AF1), (gpiod::PD4, AF1)],
    tx: [(gpioa::PA7, AF1), (gpiod::PD5, AF1)],
);

uart!(UART3,
    cts: [()],
    // dcd: [()],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [],
    tx: [],
);

uart!(UART4,
    cts: [()],
    // dcd: [()],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [],
    tx: [],
);

uart!(UART5,
    cts: [()],
    // dcd: [()],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [],
    tx: [],
);

uart!(UART6,
    cts: [()],
    // dcd: [()],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [],
    tx: [],
);

uart!(UART7,
    cts: [()],
    // dcd: [()],
    // dsr: [()],
    // dtr: [()],
    // ri: [()],
    rts: [()],
    rx: [],
    tx: [],
);

/// Serial abstraction
pub struct Serial<UART, TX, RX, RTS, CTS> {
    uart: UART,
    tx_pin: TX,
    rx_pin: RX,
    rts_pin: RTS,
    cts_pin: CTS,
    nl_mode: NewlineMode,
}

/// writeln!() emits LF chars, so this is useful
/// if you're writing text with your UART
#[derive(PartialEq, Clone, Copy)]
pub enum NewlineMode {
    /// Emit octets as received
    Binary,
    /// Emit an extra CR before every LF
    SwapLFtoCRLF,
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

macro_rules! hal {
    ($(
        $UARTX:ident: ($powerDomain:ident, $uartX:ident),
    )+) => {
        $(
            impl<TX, RX, RTS, CTS> Serial<$UARTX, TX, RX, RTS, CTS> {
                /// Configures a UART peripheral to provide serial communication
                pub fn $uartX(
                    mut uart: $UARTX,
                    tx_pin: TX,
                    rx_pin: RX,
                    mut rts_pin: RTS,
                    mut cts_pin: CTS,
                    baud_rate: Bps,
                    nl_mode: NewlineMode,
                    clocks: &Clocks,
                    pc: &sysctl::PowerControl
                ) -> Self
                where
                    TX: TxPin<$UARTX>,
                    RX: RxPin<$UARTX>,
                    CTS: CtsPin<$UARTX>,
                    RTS: RtsPin<$UARTX>,
                {
                    // Enable UART peripheral clocks
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    // Reset UART
                    uart.ctl.reset();

                    // Calculate baud rate dividers
                    let baud_int: u32 = (((clocks.sysclk.0 * 8) / baud_rate.0) + 1) / 2;

                    // Set baud rate
                    uart.ibrd.write(|w|
                        w.divint().bits((baud_int / 64) as u16));
                    uart.fbrd.write(|w|
                        w.divfrac().bits((baud_int % 64) as u8));

                    // Set data bits / parity / stop bits / enable fifo
                    uart.lcrh.write(|w| w.wlen()._8().fen().bit(true));

                    // Activate flow control (if desired)
                    rts_pin.enable(&mut uart);
                    cts_pin.enable(&mut uart);

                    // Enable uart
                    uart.ctl.modify(|_, w| w.rxe().bit(true).txe().bit(true).uarten().bit(true));

                    Serial { uart, tx_pin, rx_pin, rts_pin, cts_pin, nl_mode }
                }

                /// Splits the `Serial` abstraction into a transmitter and a
                /// receiver half. If you do this you can transmit and receive
                /// in different threads.
                pub fn split(self) -> (Tx<$UARTX, TX, RTS>, Rx<$UARTX, RX, CTS>) {
                    (
                        Tx {
                            uart: self.uart,
                            pin: self.tx_pin,
                            nl_mode: self.nl_mode,
                            flow_pin: self.rts_pin,
                        },
                        Rx {
                            _uart: PhantomData,
                            pin: self.rx_pin,
                            flow_pin: self.cts_pin,
                        },
                    )
                }

                /// Write a complete string to the UART.
                /// If this returns `Ok(())`, all the data was sent.
                /// Otherwise you get number of octets sent and the error.
                pub fn write_all<I: ?Sized>(&mut self, data: &I)
                where
                    I: AsRef<[u8]>,
                {
                    for octet in data.as_ref().iter() {
                        block!(self.write(*octet)).unwrap(); // E = Void
                    }
                }

                /// Re-combine a split UART
                pub fn combine(tx: Tx<$UARTX, TX, RTS>, rx: Rx<$UARTX, RX, CTS>) -> Serial<$UARTX, TX, RX, RTS, CTS> {
                    Serial {
                        uart: tx.uart,
                        nl_mode: tx.nl_mode,
                        rx_pin: rx.pin,
                        tx_pin: tx.pin,
                        rts_pin: tx.flow_pin,
                        cts_pin: rx.flow_pin,
                    }
                }

                /// Releases the UART peripheral and associated pins
                pub fn free(self) -> ($UARTX, TX, RX, RTS, CTS) {
                    (self.uart, self.tx_pin, self.rx_pin, self.rts_pin, self.cts_pin)
                }
            }

            impl<TX, RTS> Tx<$UARTX, TX, RTS> {
                /// Write a complete string to the UART.
                /// If this returns `Ok(())`, all the data was sent.
                /// Otherwise you get number of octets sent and the error.
                pub fn write_all<I: ?Sized>(&mut self, data: &I)
                where
                    I: AsRef<[u8]>,
                {
                    for octet in data.as_ref().iter() {
                        block!(self.write(*octet)).unwrap(); // E = Void
                    }
                }
            }

            impl<TX, RX, RTS, CTS> serial::Read<u8> for Serial<$UARTX, TX, RX, RTS, CTS> {
                type Error = Void;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    if self.uart.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(self.uart.dr.read().data().bits())
                }
            }

            impl<RX, CTS> serial::Read<u8> for Rx<$UARTX, RX, CTS> {
                type Error = Void;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    // We're only doing RX operations here so this is safe.
                    let p = unsafe { &*$UARTX::ptr() };
                    if p.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(p.dr.read().data().bits())
                }
            }

            impl<TX, RX, RTS, CTS> serial::Write<u8> for Serial<$UARTX, TX, RX, RTS, CTS> {
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| w.data().bits(byte));
                    Ok(())
                }
            }

            impl<TX, RTS> serial::Write<u8> for Tx<$UARTX, TX, RTS> {
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| w.data().bits(byte));
                    Ok(())
                }
            }

            /// Allows the Uart to be passed to 'write!()' and friends.
            impl<TX, RX, RTS, CTS> fmt::Write for Serial<$UARTX, TX, RX, RTS, CTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D)).unwrap(); // E = Void
                                }
                                block!(self.write(byte)).unwrap(); // E = Void
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Allows the Tx to be passed to 'write!()' and friends.
            impl<TX, RTS> fmt::Write for Tx<$UARTX, TX, RTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D)).unwrap(); // E = Void
                                }
                                block!(self.write(byte)).unwrap(); // E = Void
                            }
                        }
                    }
                    Ok(())
                }
            }

        )+
    }
}

hal! {
    UART0: (Uart0, uart0),
    UART1: (Uart1, uart1),
    UART2: (Uart2, uart2),
    UART3: (Uart3, uart3),
    UART4: (Uart4, uart4),
    UART5: (Uart5, uart5),
    UART6: (Uart6, uart6),
    UART7: (Uart7, uart7),
}
