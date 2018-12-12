//! Delays

use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::hal::blocking::delay::{DelayMs, DelayUs};
use crate::sysctl::Clocks;
use crate::time::Hertz;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    sysclk: Hertz,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay {
            syst,
            sysclk: clocks.sysclk,
        }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // Tricky to get this to not overflow
        let mut rvr = us * (self.sysclk.0 / 1_000_000);
        rvr += (us * ((self.sysclk.0 % 1_000_000) / 1_000)) / 1_000;
        rvr += (us * (self.sysclk.0 % 1_000)) / 1_000_000;

        while rvr >= 1 << 24 {
            self.syst.set_reload((1 << 24) - 1);
            self.syst.clear_current();
            self.syst.enable_counter();
            while !self.syst.has_wrapped() {}
            self.syst.disable_counter();
            rvr -= 1 << 24;
        }

        assert!(rvr < (1 << 24));
        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();
        while !self.syst.has_wrapped() {}
        self.syst.disable_counter();
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}
