//! Serial

use core::fmt;
use core::marker::PhantomData;

use hal::prelude::*;
use hal::serial;
use nb;
pub use tm4c123x::{UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7};
use void::Void;

use gpio::{gpioa, gpiob, gpioc, gpiod, gpioe, gpiof};
use gpio::{AF1, AF2, AF8, AlternateFunction, OutputMode};
use sysctl;
use sysctl::Clocks;
use time::Bps;

// FIXME these should be "closed" traits
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

/// RTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RtsPin<UART> {
    /// Enables the RTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART) {}
}

/// CTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait CtsPin<UART> {
    /// Enables the CTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART) {}
}

unsafe impl CtsPin<UART0> for () {}
unsafe impl RtsPin<UART0> for () {}
unsafe impl CtsPin<UART1> for () {}
unsafe impl RtsPin<UART1> for () {}
unsafe impl CtsPin<UART2> for () {}
unsafe impl RtsPin<UART2> for () {}
unsafe impl CtsPin<UART3> for () {}
unsafe impl RtsPin<UART3> for () {}
unsafe impl CtsPin<UART4> for () {}
unsafe impl RtsPin<UART4> for () {}
unsafe impl CtsPin<UART5> for () {}
unsafe impl RtsPin<UART5> for () {}
unsafe impl CtsPin<UART6> for () {}
unsafe impl RtsPin<UART6> for () {}
unsafe impl CtsPin<UART7> for () {}
unsafe impl RtsPin<UART7> for () {}

unsafe impl<T> CtsPin<UART1> for gpiof::PF1<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
    fn enable(&mut self, uart: &mut UART1) {
        uart.ctl.modify(|_, w| w.ctsen().set_bit());
    }
}
unsafe impl<T> CtsPin<UART1> for gpioc::PC5<AlternateFunction<AF8, T>>
where
    T: OutputMode,
{
    fn enable(&mut self, uart: &mut UART1) {
        uart.ctl.modify(|_, w| w.ctsen().set_bit());
    }
}
unsafe impl<T> RtsPin<UART1> for gpioc::PC4<AlternateFunction<AF8, T>>
where
    T: OutputMode,
{
    fn enable(&mut self, uart: &mut UART1) {
        uart.ctl.modify(|_, w| w.rtsen().set_bit());
    }
}
unsafe impl<T> RtsPin<UART1> for gpiof::PF0<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
    fn enable(&mut self, uart: &mut UART1) {
        uart.ctl.modify(|_, w| w.rtsen().set_bit());
    }
}

unsafe impl<T> RxPin<UART0> for gpioa::PA0<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART1> for gpiob::PB0<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART1> for gpioc::PC4<AlternateFunction<AF2, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART2> for gpiod::PD6<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART3> for gpioc::PC6<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART4> for gpioc::PC4<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART5> for gpioe::PE4<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART6> for gpiod::PD4<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> RxPin<UART7> for gpioe::PE0<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART0> for gpioa::PA1<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART1> for gpiob::PB1<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART1> for gpioc::PC5<AlternateFunction<AF2, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART2> for gpiod::PD7<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART3> for gpioc::PC7<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART4> for gpioc::PC5<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART5> for gpioe::PE5<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART6> for gpiod::PD5<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}
unsafe impl<T> TxPin<UART7> for gpioe::PE1<AlternateFunction<AF1, T>>
where
    T: OutputMode,
{
}

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
                        unsafe { w.divint().bits((baud_int / 64) as u16) });
                    uart.fbrd.write(|w|
                        unsafe { w.divfrac().bits((baud_int % 64) as u8) });

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
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
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
                    self.uart.dr.write(|w| unsafe { w.data().bits(byte) });
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
