//! Inter-Integrated Circuit (I2C) bus

use cortex_m::asm::delay;
use tm4c123x::{I2C0, I2C1, I2C2, I2C3};

use crate::gpio::{gpioa, gpiob, gpiod, gpioe};
use crate::gpio::{AlternateFunction, Floating, OpenDrain, OutputMode, AF3};

use crate::sysctl::{self, Clocks};

use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::time::Hertz;

pub use tm4c_hal::i2c::Error;
pub use tm4c_hal::{i2c_busy_wait, i2c_hal, i2c_pins};

/// I2C peripheral operating in master mode
pub struct I2c<I2C, PINS> {
    /// Underlying I2C peripheral
    pub i2c: I2C,
    /// Underlying GPIO pins used by peripheral
    pub pins: PINS,
}

// FIXME these should be "closed" traits
/// SCL pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SclPin<I2C> {}

/// SDA pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SdaPin<I2C> {}

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
