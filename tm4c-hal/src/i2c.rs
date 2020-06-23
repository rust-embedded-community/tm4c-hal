//! Common I2C code for TM4C123 and TM4C129

/// I2C error
#[derive(Debug)]
pub enum Error {
    /// Bus Busy
    BusBusy,

    /// Arbitration loss
    Arbitration,

    /// Missing Data ACK
    DataAck,

    /// Missing Address ACK
    AdrAck,

    /// I2C Timeout
    Timeout,

    #[doc(hidden)]
    _Extensible,
}

#[macro_export]
/// Implements the traits for an I2C peripheral
macro_rules! i2c_pins {
    ($UARTn:ident,
        scl: [$(($($sclgpio: ident)::*, $sclaf: ident)),*],
        sda: [$(($($sdagpio: ident)::*, $sdaaf: ident)),*],
    ) => {
        $(
            unsafe impl SclPin<$UARTn> for $($sclgpio)::*<AlternateFunction<$sclaf, OpenDrain<Floating>>>
            {}
        )*

        $(
            unsafe impl<T> SdaPin<$UARTn> for $($sdagpio)::*<AlternateFunction<$sdaaf, T>>
            where
                T: OutputMode,
            {}
        )*
    }
}

#[macro_export]
/// Spins on a given field on a TM4C I2C peripheral
macro_rules! i2c_busy_wait {
    ($i2c:expr) => {
        {
            // in 'release' builds, the time between setting the `run` bit and checking the `busy`
            // bit is too short and the `busy` bit is not reliably set by the time you get there,
            // it can take up to 8 clock cycles for the `run` to begin so this delay allows time
            // for that hardware synchronization
            delay(8);

            // Allow 1,000 clock cycles before we timeout. At 100 kHz, this is 10 ms.
            $i2c.mclkocnt.write(|w| unsafe { w.cntl().bits((1_000 >> 4) as u8) });

            let mcs = loop {
                let mcs = $i2c.mcs.read();

                if mcs.busy().bit_is_clear() {
                    break mcs;
                }
            };

            if mcs.clkto().bit_is_set() {
                Err(Error::Timeout)
            } else if mcs.busbsy().bit_is_set() {
                Err(Error::BusBusy)
            } else if mcs.arblst().bit_is_set() {
                Err(Error::Arbitration)
            } else if mcs.datack().bit_is_set() {
                Err(Error::DataAck)
            } else if mcs.adrack().bit_is_set() {
                Err(Error::AdrAck)
            } else {
                Ok(())
            }
        }
    };
}
#[macro_export]
/// Implements embedded-hal for an TM4C I2C peripheral
macro_rules! i2c_hal {
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

                    loop {
                        match i2c_busy_wait!(self.i2c) {
                            Ok(()) => break,
                            Err(Error::BusBusy) => continue,
                            e @ Err(_) => return e,
                        }
                    }

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
                        i2c_busy_wait!(self.i2c)?;

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

                    i2c_busy_wait!(self.i2c)?;

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

                    loop {
                        match i2c_busy_wait!(self.i2c) {
                            Ok(()) => break,
                            Err(Error::BusBusy) => continue,
                            e @ Err(_) => return e,
                        }
                    }

                    let recv_sz = buffer.len();

                    if recv_sz == 1 {
                        // Single receive
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .start().set_bit()
                                .stop().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
                        buffer[0] = self.i2c.mdr.read().data().bits();
                    } else {
                        self.i2c.mcs.write(|w| {
                            w.start().set_bit()
                                .run().set_bit()
                                .ack().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
                        buffer[0] = self.i2c.mdr.read().data().bits();

                        for byte in &mut buffer[1..recv_sz-1] {
                            self.i2c.mcs.write(|w| {
                                w.run().set_bit()
                                    .ack().set_bit()
                            });
                            i2c_busy_wait!(self.i2c)?;
                            *byte = self.i2c.mdr.read().data().bits();
                        }
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .stop().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
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

                    loop {
                        match i2c_busy_wait!(self.i2c) {
                            Ok(()) => break,
                            Err(Error::BusBusy) => continue,
                            e @ Err(_) => return e,
                        }
                    }

                    self.i2c.mcs.write(|w| {
                        w.start().set_bit()
                            .run().set_bit()
                    });

                    i2c_busy_wait!(self.i2c)?;

                    for byte in (&bytes[1..write_len]).iter() {
                        self.i2c.mdr.write(|w| unsafe {
                            w.data().bits(*byte)
                        });

                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
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

                        i2c_busy_wait!(self.i2c)?;
                        buffer[0] = self.i2c.mdr.read().data().bits();
                    } else {
                        // emit Repeated START
                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .start().set_bit()
                                .ack().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
                        buffer[0] = self.i2c.mdr.read().data().bits();

                        for byte in &mut buffer[1..recv_sz-1] {
                            self.i2c.mcs.write(|w| {
                                w.run().set_bit()
                                    .ack().set_bit()
                            });
                            i2c_busy_wait!(self.i2c)?;
                            *byte = self.i2c.mdr.read().data().bits();
                        }

                        self.i2c.mcs.write(|w| {
                            w.run().set_bit()
                                .stop().set_bit()
                        });

                        i2c_busy_wait!(self.i2c)?;
                        buffer[recv_sz-1] = self.i2c.mdr.read().data().bits();
                    }

                    Ok(())
                }
            }
        )+
    }
}
