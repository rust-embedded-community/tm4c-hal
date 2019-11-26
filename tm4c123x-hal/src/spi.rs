//! Serial Peripheral Interface (SPI) bus

pub use crate::hal::spi::{Mode, MODE_0, MODE_1, MODE_2, MODE_3};

use crate::gpio::gpioa::{PA2, PA4, PA5};
use crate::gpio::gpiob::{PB4, PB6, PB7};
use crate::gpio::gpiod::{PD0, PD2, PD3};
use crate::gpio::{AlternateFunction, OutputMode, AF1, AF2};
use crate::hal::spi::{FullDuplex, Phase, Polarity};
use crate::sysctl;
use crate::sysctl::Clocks;
use crate::time::Hertz;

use nb;
use tm4c123x::{SSI0, SSI1, SSI2, SSI3};

/// SPI error
#[derive(Debug)]
pub enum Error {
    #[doc(hidden)]
    _Extensible,
}

// FIXME these should be "closed" traits
/// SCK pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SckPin<SPI> {}

/// MISO pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MisoPin<SPI> {}

/// MOSI pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MosiPin<SPI> {}

// SSI0
unsafe impl<T> SckPin<SSI0> for PA2<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MisoPin<SSI0> for PA4<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MosiPin<SSI0> for PA5<AlternateFunction<AF2, T>> where T: OutputMode {}

// SSI1
unsafe impl<T> SckPin<SSI1> for PD0<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MisoPin<SSI1> for PD2<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MosiPin<SSI1> for PD3<AlternateFunction<AF2, T>> where T: OutputMode {}

// SSI2
unsafe impl<T> SckPin<SSI2> for PB4<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MisoPin<SSI2> for PB6<AlternateFunction<AF2, T>> where T: OutputMode {}
unsafe impl<T> MosiPin<SSI2> for PB7<AlternateFunction<AF2, T>> where T: OutputMode {}

// SSI3
unsafe impl<T> SckPin<SSI3> for PD0<AlternateFunction<AF1, T>> where T: OutputMode {}
unsafe impl<T> MisoPin<SSI3> for PD2<AlternateFunction<AF1, T>> where T: OutputMode {}
unsafe impl<T> MosiPin<SSI3> for PD3<AlternateFunction<AF1, T>> where T: OutputMode {}

/// SPI peripheral operating in full duplex master mode
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

macro_rules! busy_wait {
    ($spi:expr, $flag:ident, $op:ident) => {
        loop {
            let sr = $spi.sr.read();
            if sr.$flag().$op() {
                break;
            }
        }
    };
}

macro_rules! hal {
    ($($SPIX:ident: ($powerDomain:ident, $spiX:ident),)+) => {
        $(
            impl<SCK, MISO, MOSI> Spi<$SPIX, (SCK, MISO, MOSI)> {
                /// Configures the SPI peripheral to operate in full duplex master mode
                pub fn $spiX<F>(
                    spi: $SPIX,
                    pins: (SCK, MISO, MOSI),
                    mode: Mode,
                    freq: F,
                    clocks: &Clocks,
                    pc: &sysctl::PowerControl,
                ) -> Self
                where
                    F: Into<Hertz>,
                    SCK: SckPin<$SPIX>,
                    MISO: MisoPin<$SPIX>,
                    MOSI: MosiPin<$SPIX>,
                {
                    // power up
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    // write 0 (reset value) for master operation.
                    spi.cr1.write(|w| w);

                    // SSICC Clock setup
                    // set to reset value (0 = use system clock)
                    spi.cc.write(|w| w);

                    // Use Moto/SPI & 8bits data size
                    let scr: u8;
                    let mut cpsr = 2u32;
                    let target_bitrate : u32 = clocks.sysclk.0 / freq.into().0;

                    // Find solution for
                    // SSInClk = SysClk / (CPSDVSR * (1 + SCR))
                    // with:
                    //   CPSDVSR in [2,254]
                    //   SCR in [0,255]

                    loop {
                        let scr32 = (target_bitrate / cpsr) - 1;
                        if scr32 < 255 {
                            scr = scr32 as u8;
                            break;
                        }
                        cpsr += 2;
                        assert!(cpsr <= 254);
                    }

                    let cpsr = cpsr as u8;

                    spi.cpsr.write(|w| unsafe {
                        w.cpsdvsr().bits(cpsr)
                    });

                    spi.cr0.modify(|_,w| unsafe {
                        w.spo().bit(mode.polarity == Polarity::IdleHigh)
                            .sph().bit(mode.phase == Phase::CaptureOnSecondTransition)
                            .frf().moto()
                            .dss()._8()
                            .scr().bits(scr)
                    });

                    // Enable peripheral
                    spi.cr1.write(|w| w.sse().set_bit());

                    Spi { spi, pins }
                }

                /// Releases the SPI peripheral and associated pins
                pub fn free(self) -> ($SPIX, (SCK, MISO, MOSI)) {
                    (self.spi, self.pins)
                }

                /// Change the clock frequency of the SPI device.
                pub fn reclock<F>(&mut self, freq: F, clocks: &Clocks) where F: Into<Hertz> {
                    // Disable peripheral
                    self.spi.cr1.modify(|_, w| w.sse().clear_bit());

                    let scr: u8;
                    let mut cpsr = 2u32;
                    let target_bitrate : u32 = clocks.sysclk.0 / freq.into().0;

                    // Find solution for
                    // SSInClk = SysClk / (CPSDVSR * (1 + SCR))
                    // with:
                    //   CPSDVSR in [2,254]
                    //   SCR in [0,255]

                    loop {
                        let scr32 = (target_bitrate / cpsr) - 1;
                        if scr32 < 255 {
                            scr = scr32 as u8;
                            break;
                        }
                        cpsr += 2;
                        assert!(cpsr <= 254);
                    }

                    let cpsr = cpsr as u8;

                    self.spi.cpsr.write(|w| unsafe { w.cpsdvsr().bits(cpsr) });
                    self.spi.cr0.modify(|_,w| unsafe { w.scr().bits(scr) });

                    // Enable peripheral again
                    self.spi.cr1.modify(|_, w| w.sse().set_bit());
                }
            }

            impl<PINS> FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    // Receive FIFO Not Empty
                    if self.spi.sr.read().rne().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        let r = self.spi.dr.read().data().bits() as u8;
                        Ok(r)
                    }
                }

                fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
                    // Transmit FIFO Not Full
                    if self.spi.sr.read().tnf().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.spi.dr.write(|w| unsafe {
                            w.data().bits(byte.into())
                        });
                        busy_wait!(self.spi, bsy, bit_is_clear);
                        Ok(())
                    }
                }
            }

            impl<PINS> crate::hal::blocking::spi::transfer::Default<u8> for Spi<$SPIX, PINS> {}

            impl<PINS> crate::hal::blocking::spi::write::Default<u8> for Spi<$SPIX, PINS> {}
        )+
    }
}

hal! {
    SSI0: (Ssi0, spi0),
    SSI1: (Ssi1, spi1),
    SSI2: (Ssi2, spi2),
    SSI3: (Ssi3, spi3),
}
