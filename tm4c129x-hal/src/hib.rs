//! A wrapper for the HIB (Hibernation) peripheral

use crate::sysctl;

/// Which source to use for the HIB clock
pub enum Source {
    /// HIB clock is from an external crystal
    ExternalCrystal,
    /// HIB clock is from an external oscillator
    ExternalOscillator,
    /// HIB clock is from the internal low-frequency oscillator
    LowFrequencyInternalOscillator,
}

/// A wrapper around the HIB (Hibernation) peripheral
pub struct Hib {
    hib: tm4c129x::HIB,
}

impl Hib {
    /// Initialize the HIB peripheral, using a clock from `source`
    pub fn hib(hib: tm4c129x::HIB, source: Source, _pc: &sysctl::PowerControl) -> Self {
        hib.ctl.write(|w| {
            match source {
                Source::ExternalCrystal => w.oscsel().clear_bit().oscbyp().clear_bit(),
                Source::ExternalOscillator => w.oscsel().clear_bit().oscbyp().set_bit(),
                Source::LowFrequencyInternalOscillator => w.oscsel().set_bit().oscbyp().clear_bit(),
            };

            w.clk32en().set_bit();

            w
        });

        while hib.ctl.read().wrc().bit_is_clear() {}
        hib.ctl.write(|w| {
            match source {
                Source::ExternalCrystal => w.oscsel().clear_bit().oscbyp().clear_bit(),
                Source::ExternalOscillator => w.oscsel().clear_bit().oscbyp().set_bit(),
                Source::LowFrequencyInternalOscillator => w.oscsel().set_bit().oscbyp().clear_bit(),
            };

            w.oscdrv().set_bit();
            w.clk32en().set_bit();
            w.rtcen().set_bit();

            w
        });
        while hib.ctl.read().wrc().bit_is_clear() {}

        Hib { hib }
    }

    /// Get the current time, in units of (seconds, subseconds), where a
    /// subsecond is 1/32768 seconds
    pub fn get_time(&self) -> (u32, u16) {
        loop {
            let seconds = self.hib.rtcc.read().bits();
            let subsec = self.hib.rtcss.read().rtcssc().bits();

            if seconds == self.hib.rtcc.read().bits() {
                return (seconds, subsec);
            }
        }
    }

    /// Get the current time in milliseconds
    pub fn get_millis(&self) -> u64 {
        let (seconds, subsec) = self.get_time();
        let seconds: u64 = seconds.into();
        let subsec: u64 = subsec.into();

        let millis: u64 = subsec * 1000 / 32_768;
        seconds * 1000 + millis
    }
}
