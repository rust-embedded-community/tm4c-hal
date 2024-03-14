//! Serial Peripheral Interface (SPI) bus

pub use crate::hal::spi::{Mode, MODE_0, MODE_1, MODE_2, MODE_3};

use crate::{
    gpio::{
        gpioa::{PA2, PA4, PA5},
        gpiob::{PB5},
        gpiod::{PD0, PD1, PD3},
        gpioe::{PE4, PE5},
        // gpiof::{PF0, PF1, PF3},
        gpioq::{PQ0, PQ2, PQ3},
        AlternateFunction, OutputMode, AF14, AF15,
    },
    hal::spi::{FullDuplex, Phase, Polarity},
    sysctl::{self, Clocks},
    time::Hertz,
    Sealed,
};

use nb;
use tm4c129x::{SSI0, SSI1, SSI2, SSI3};

/// SPI error
#[derive(Debug)]
pub enum Error {
    #[doc(hidden)]
    _Extensible,
}

/// SCK pin
pub trait SckPin<SPI>: Sealed {}

/// MISO pin
pub trait MisoPin<SPI>: Sealed {}

/// MOSI pin
pub unsafe trait MosiPin<SPI>: Sealed {}

// SSI0
impl<T> SckPin<SSI0> for PA2<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MisoPin<SSI0> for PA4<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MosiPin<SSI0> for PA5<AlternateFunction<AF15, T>> where T: OutputMode {}

// SSI1
impl<T> SckPin<SSI1> for PB5<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MisoPin<SSI1> for PE4<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MosiPin<SSI1> for PE5<AlternateFunction<AF15, T>> where T: OutputMode {}

// SSI2
impl<T> SckPin<SSI2> for PD3<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MisoPin<SSI2> for PD1<AlternateFunction<AF15, T>> where T: OutputMode {}
impl<T> MosiPin<SSI2> for PD0<AlternateFunction<AF15, T>> where T: OutputMode {}

// SSI3
impl<T> SckPin<SSI3> for PQ0<AlternateFunction<AF14, T>> where T: OutputMode {}
impl<T> MisoPin<SSI3> for PQ2<AlternateFunction<AF14, T>> where T: OutputMode {}
impl<T> MosiPin<SSI3> for PQ3<AlternateFunction<AF14, T>> where T: OutputMode {}

// SSI3 (alt)
// impl<T> SckPin<SSI3> for PF3<AlternateFunction<AF14, T>> where T: OutputMode {}
// impl<T> MisoPin<SSI3> for PF1<AlternateFunction<AF14, T>> where T: OutputMode {}
// impl<T> MosiPin<SSI3> for PF0<AlternateFunction<AF14, T>> where T: OutputMode {}

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
                            // FIXME: How to use FRFR::MOTO and DSS:: ?
                            .frf().bits(0)
                            .dss().bits(0x7)
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
