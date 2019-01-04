//! Inter-Integrated Circuit (I2C) bus

use tm4c123x::{I2C0, I2C1, I2C2, I2C3};
use cortex_m::asm::delay;

use crate::gpio::gpioa::{PA6, PA7};
use crate::gpio::gpiob::{PB2, PB3};
use crate::gpio::gpiod::{PD0, PD1};
use crate::gpio::gpioe::{PE4, PE5};

use crate::gpio::{AlternateFunction, Floating, OpenDrain, OutputMode, AF3};

use crate::sysctl::{self, Clocks};

use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::time::Hertz;

/// I2C error
#[derive(Debug)]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,

    /// Missing Data ACK
    DataAck,

    /// Missing Addrees ACK
    AdrAck,

    #[doc(hidden)]
    _Extensible,
}

// FIXME these should be "closed" traits
/// SCL pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SclPin<I2C> {}

/// SDA pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SdaPin<I2C> {}

unsafe impl<T> SclPin<I2C0> for PB2<AlternateFunction<AF3, T>> where T: OutputMode {}
unsafe impl SdaPin<I2C0> for PB3<AlternateFunction<AF3, OpenDrain<Floating>>> {}

unsafe impl<T> SclPin<I2C1> for PA6<AlternateFunction<AF3, T>> where T: OutputMode {}
unsafe impl SdaPin<I2C1> for PA7<AlternateFunction<AF3, OpenDrain<Floating>>> {}

unsafe impl<T> SclPin<I2C2> for PE4<AlternateFunction<AF3, T>> where T: OutputMode {}
unsafe impl SdaPin<I2C2> for PE5<AlternateFunction<AF3, OpenDrain<Floating>>> {}

unsafe impl<T> SclPin<I2C3> for PD0<AlternateFunction<AF3, T>> where T: OutputMode {}
unsafe impl SdaPin<I2C3> for PD1<AlternateFunction<AF3, OpenDrain<Floating>>> {}

/// I2C peripheral operating in master mode
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

macro_rules! busy_wait {
    ($i2c:expr, $flag:ident, $op:ident) => {
        // in 'release' builds, the time between setting the `run` bit and checking the `busy`
        // bit is too short and the `busy` bit is not reliably set by the time you get there,
        // it can take up to 8 clock cycles for the `run` to begin so this delay allows time
        // for that hardware synchronization
        delay(2);

        loop {
            let mcs = $i2c.mcs.read();

            if mcs.error().bit_is_set() {
                if mcs.adrack().bit_is_set() {
                    return Err(Error::AdrAck);
                } else if mcs.datack().bit_is_set() {
                    return Err(Error::DataAck);
                }
                return Err(Error::Bus);
            } else if mcs.arblst().bit_is_set() {
                return Err(Error::Arbitration);
            } else if mcs.$flag().$op() {
                break;
            } else {
                // try again
            }
        }
    };
}

macro_rules! hal {
    ($($I2CX:ident: ($powerDomain:ident, $i2cX:ident),)+) => {
        $(
            impl<SCL, SDA> I2c<$I2CX, (SCL, SDA)> {
                /// Configures the I2C peripheral to work in master mode
                pub fn $i2cX<F>(
                    i2c: $I2CX,
                    pins: (SCL, SDA),
                    freq: F,
                    clocks: &Clocks,
                    pc: &sysctl::PowerControl,
                ) -> Self where
                    F: Into<Hertz>,
                    SCL: SclPin<$I2CX>,
                    SDA: SdaPin<$I2CX>,
                {
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    // set Master Function Enable, and clear other bits.
                    i2c.mcr.write(|w| w.mfe().set_bit());

                    // Write TimerPeriod configuration and clear other bits.
                    let freq = freq.into().0;
                    let tpr = ((clocks.sysclk.0/(2*10*freq))-1) as u8;

                    i2c.mtpr.write(|w| unsafe {w.tpr().bits(tpr)});

                    I2c { i2c, pins }
                }

                /// Releases the I2C peripheral and associated pins
                pub fn free(self) -> ($I2CX, (SCL, SDA)) {
                    (self.i2c, self.pins)
                }
            }

            impl<PINS> Write for I2c<$I2CX, PINS> {
                type Error = Error;

                fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
                    // Write Slave address and clear Receive bit
                    self.i2c.msa.write(|w| unsafe {
                        w.sa().bits(addr)
                    });

                    // Put first byte in data register
                    self.i2c.mdr.write(|w| unsafe {
                        w.data().bits(bytes[0])
                    });

                    let sz = bytes.len();

                    busy_wait!(self.i2c, busbsy, bit_is_clear);

                    // Send START + RUN
                    // If single byte transfer, set STOP
                    self.i2c.mcs.write(|w| {
                        if sz == 1 {
                            w.stop().set_bit();
                        }
                        w.start().set_bit()
                            .run().set_bit()
                    });

                    for (i,byte) in (&bytes[1..]).iter().enumerate() {
                        busy_wait!(self.i2c, busy, bit_is_clear);

                        // Put next byte in data register
                        self.i2c.mdr.write(|w| unsafe {
                            w.data().bits(*byte)
                        });

                        // Send RUN command (Burst continue)
                        // Set STOP on last byte
                        self.i2c.mcs.write(|w| {
                            if (i+1) == (sz-1) {
                                w.stop().set_bit();
                            }
                            w.run().set_bit()
                        });
                    }

                    busy_wait!(self.i2c, busy, bit_is_clear);

                    Ok(())
                }
            }

            impl<PINS> Read for I2c<$I2CX, PINS> {
                type Error = Error;

                fn read(
                    &mut self,
                    addr: u8,
                    buffer: &mut [u8],
                ) -> Result<(), Error> {

                    // Write Slave address and set Receive bit
                    self.i2c.msa.write(|w| unsafe {
                        w.sa().bits(addr)
                            .rs().set_bit()
                    });

                    busy_wait!(self.i2c, busbsy, bit_is_clear);
                    let recv_sz = buffer.len();

                    if recv_sz == 1 {
                        // Single receive
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .start().set_bit()
                                .stop().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[0] = self.i2c.mdr.read().data().bits();
                    } else {
                        self.i2c.mcs.write(|w| {
                            w.start().set_bit()
                                .run().set_bit()
                                .ack().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[0] = self.i2c.mdr.read().data().bits();

                        for byte in &mut buffer[1..recv_sz-1] {
                            self.i2c.mcs.write(|w| {
                                w.run().set_bit()
                                    .ack().set_bit()
                            });
                            busy_wait!(self.i2c, busy, bit_is_clear);
                            *byte = self.i2c.mdr.read().data().bits();
                        }
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .stop().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[recv_sz-1] = self.i2c.mdr.read().data().bits();
                    }

                    Ok(())
                }
            }

            impl<PINS> WriteRead for I2c<$I2CX, PINS> {
                type Error = Error;

                fn write_read(
                    &mut self,
                    addr: u8,
                    bytes: &[u8],
                    buffer: &mut [u8],
                ) -> Result<(), Error> {

                    let write_len = bytes.len();

                    if buffer.len() == 0 {
                       return self.write(addr, bytes);
                    }

                    if bytes.len() == 0 {
                        return self.read(addr, buffer);
                    }

                    // Write Slave address and clear Receive bit
                    self.i2c.msa.write(|w| unsafe {
                        w.sa().bits(addr)
                    });

                    // send first byte
                    self.i2c.mdr.write(|w| unsafe {
                        w.data().bits(bytes[0])
                    });

                    busy_wait!(self.i2c, busbsy, bit_is_clear);

                    self.i2c.mcs.write(|w| {
                        w.start().set_bit()
                            .run().set_bit()
                    });

                    busy_wait!(self.i2c, busy, bit_is_clear);

                    for byte in (&bytes[1..write_len]).iter() {
                        self.i2c.mdr.write(|w| unsafe {
                            w.data().bits(*byte)
                        });

                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                    }

                    // Write Slave address and set Receive bit
                    self.i2c.msa.write(|w| unsafe {
                        w.sa().bits(addr)
                            .rs().set_bit()
                    });

                    let recv_sz = buffer.len();

                    if recv_sz == 1 {
                        // emit Repeated START and STOP for single receive
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .start().set_bit()
                                .stop().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[0] = self.i2c.mdr.read().data().bits();
                    } else {
                        // emit Repeated START
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .start().set_bit()
                                .ack().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[0] = self.i2c.mdr.read().data().bits();

                        for byte in &mut buffer[1..recv_sz-1] {
                            self.i2c.mcs.write(|w| {
                                w.run().set_bit()
                                    .ack().set_bit()
                            });
                            busy_wait!(self.i2c, busy, bit_is_clear);
                            *byte = self.i2c.mdr.read().data().bits();
                        }

                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .stop().set_bit()
                        });

                        busy_wait!(self.i2c, busy, bit_is_clear);
                        buffer[recv_sz-1] = self.i2c.mdr.read().data().bits();
                    }

                    Ok(())
                }
            }
        )+
    }
}

hal! {
    I2C0: (I2c0, i2c0),
    I2C1: (I2c1, i2c1),
    I2C2: (I2c2, i2c2),
    I2C3: (I2c3, i2c3),
}
