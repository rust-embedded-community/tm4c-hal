//! Serial

use core::marker::PhantomData;
use core::fmt;

use hal::serial;
use hal::prelude::*;
use nb;
use tm4c123x::{UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7};

use gpio::{gpioa, gpiob, gpioc};
use gpio::{AF1, AF2};
use sysctl::Clocks;
use time::Bps;
use sysctl;

// FIXME these should be "closed" traits
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

// TODO: Add traits here for RTS and CTS?

unsafe impl RxPin<UART0> for gpioa::PA0<AF1> {}
unsafe impl TxPin<UART0> for gpioa::PA1<AF1> {}

unsafe impl RxPin<UART1> for gpiob::PB0<AF1> {}
unsafe impl TxPin<UART1> for gpiob::PB1<AF1> {}

unsafe impl RxPin<UART1> for gpioc::PC4<AF2> {}
unsafe impl TxPin<UART1> for gpioc::PC5<AF2> {}

unsafe impl RxPin<UART4> for gpioc::PC4<AF1> {}
unsafe impl TxPin<UART4> for gpioc::PC5<AF1> {}

unsafe impl RxPin<UART3> for gpioc::PC6<AF1> {}
unsafe impl TxPin<UART3> for gpioc::PC7<AF1> {}

/// Serial abstraction
pub struct Serial<UART, TX, RX> {
    uart: UART,
    tx_pin: TX,
    rx_pin: RX,
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
pub struct Rx<UART, RX> {
    _uart: PhantomData<UART>,
    pin: RX,
}

/// Serial transmitter
pub struct Tx<UART, TX> {
    uart: UART,
    pin: TX,
    nl_mode: NewlineMode,
}

macro_rules! hal {
    ($(
        $UARTX:ident: ($powerDomain:ident, $uartX:ident),
    )+) => {
        $(
            impl<TX, RX> Serial<$UARTX, TX, RX> {
                /// Configures a UART peripheral to provide serial communication
                pub fn $uartX(
                    uart: $UARTX,
                    tx_pin: TX,
                    rx_pin: RX,
                    baud_rate: Bps,
                    nl_mode: NewlineMode,
                    clocks: &Clocks,
                    pc: &sysctl::PowerControl
                ) -> Self
                where
                    TX: TxPin<$UARTX>,
                    RX: RxPin<$UARTX>,
                {
                    // Enable UART peripheral clocks
                    sysctl::control_power(
                        pc, sysctl::PeripheralPowerDomain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::PeripheralPowerDomain::$powerDomain);

                    // Reset UART
                    uart.ctl.reset();

                    // Calculate baud rate dividers
                    let baud_int: u32 = (((clocks.sysclk.0 * 8) / baud_rate.0) + 1) / 2;

                    // Set baud rate
                    uart.ibrd.write(|w|
                        unsafe { w.divint().bits((baud_int / 64) as u16) });
                    uart.fbrd.write(|w|
                        unsafe { w.divfrac().bits((baud_int % 64) as u8) });

                    // Set data bits / parity / stop bits / enable fifo
                    uart.lcrh.write(|w| w.wlen()._8().fen().bit(true));

                    // Enable uart
                    uart.ctl.write(|w| w.rxe().bit(true).txe().bit(true).uarten().bit(true));

                    Serial { uart, tx_pin, rx_pin, nl_mode }
                }

                /// Splits the `Serial` abstraction into a transmitter and a
                /// receiver half. If you do this you can transmit and receive
                /// in different threads.
                pub fn split(self) -> (Tx<$UARTX, TX>, Rx<$UARTX, RX>) {
                    (
                        Tx {
                            uart: self.uart,
                            pin: self.tx_pin,
                            nl_mode: self.nl_mode
                        },
                        Rx {
                            _uart: PhantomData,
                            pin: self.rx_pin
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
                        block!(self.write(*octet)).unwrap();
                    }
                }

                /// Re-combine a split UART
                pub fn combine(tx: Tx<$UARTX, TX>, rx: Rx<$UARTX, RX>) -> Serial<$UARTX, TX, RX> {
                    Serial {
                        uart: tx.uart,
                        nl_mode: tx.nl_mode,
                        rx_pin: rx.pin,
                        tx_pin: tx.pin
                    }
                }

                /// Releases the UART peripheral and associated pins
                pub fn free(self) -> ($UARTX, TX, RX) {
                    (self.uart, self.tx_pin, self.rx_pin)
                }
            }

            impl<TX> Tx<$UARTX, TX> {
                /// Write a complete string to the UART.
                /// If this returns `Ok(())`, all the data was sent.
                /// Otherwise you get number of octets sent and the error.
                pub fn write_all<I: ?Sized>(&mut self, data: &I)
                where
                    I: AsRef<[u8]>,
                {
                    for octet in data.as_ref().iter() {
                        block!(self.write(*octet)).unwrap();
                    }
                }
            }

            impl<TX, RX> serial::Read<u8> for Serial<$UARTX, TX, RX> {
                type Error = !;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    if self.uart.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(self.uart.dr.read().data().bits())
                }
            }

            impl<RX> serial::Read<u8> for Rx<$UARTX, RX> {
                type Error = !;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    // We're only doing RX operations here so this is safe.
                    let p = unsafe { &*$UARTX::ptr() };
                    if p.fr.read().rxfe().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(p.dr.read().data().bits())
                }
            }

            impl<TX, RX> serial::Write<u8> for Serial<$UARTX, TX, RX> {
                type Error = !;

                fn flush(&mut self) -> nb::Result<(), !> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), !> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
                    Ok(())
                }
            }

            impl<TX> serial::Write<u8> for Tx<$UARTX, TX> {
                type Error = !;

                fn flush(&mut self) -> nb::Result<(), !> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), !> {
                    if self.uart.fr.read().txff().bit() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
                    Ok(())
                }
            }

            /// Allows the Uart to be passed to 'write!()' and friends.
            impl<TX, RX> fmt::Write for Serial<$UARTX, TX, RX> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D))?;
                                }
                                block!(self.write(byte))?;
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Allows the Tx to be passed to 'write!()' and friends.
            impl<TX> fmt::Write for Tx<$UARTX, TX> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    match self.nl_mode {
                        NewlineMode::Binary => self.write_all(s),
                        NewlineMode::SwapLFtoCRLF => {
                            for byte in s.bytes() {
                                if byte == 0x0A {
                                    // Prefix every \n with a \r
                                    block!(self.write(0x0D))?;
                                }
                                block!(self.write(byte))?;
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
