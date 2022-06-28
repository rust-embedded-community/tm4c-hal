//! Inter-Integrated Circuit (I2C) bus

use crate::{
    gpio::*,
    hal::blocking::i2c::{Read, Write, WriteRead},
    sysctl::{self, Clocks},
    time::Hertz,
    Sealed,
};

use cortex_m::asm::delay;
use tm4c123x::{I2C0, I2C1, I2C2, I2C3};

pub use tm4c_hal::i2c::Error;
pub use tm4c_hal::{i2c_busy_wait, i2c_hal, i2c_pins};

/// I2C peripheral operating in master mode
pub struct I2c<I2C, PINS> {
    /// Underlying I2C peripheral
    pub i2c: I2C,
    /// Underlying GPIO pins used by peripheral
    pub pins: PINS,
}

/// SCL pin
pub trait SclPin<I2C>: Sealed {}

/// SDA pin
pub trait SdaPin<I2C>: Sealed {}

i2c_pins!(I2C0, scl: [(gpiob::PB2, AF3)], sda: [(gpiob::PB3, AF3)],);
i2c_pins!(I2C1, scl: [(gpioa::PA6, AF3)], sda: [(gpioa::PA7, AF3)],);
i2c_pins!(I2C2, scl: [(gpioe::PE4, AF3)], sda: [(gpioe::PE5, AF3)],);
i2c_pins!(I2C3, scl: [(gpiod::PD0, AF3)], sda: [(gpiod::PD1, AF3)],);

i2c_hal! {
    I2C0: (I2c0, i2c0),
    I2C1: (I2c1, i2c1),
    I2C2: (I2c2, i2c2),
    I2C3: (I2c3, i2c3),
}
