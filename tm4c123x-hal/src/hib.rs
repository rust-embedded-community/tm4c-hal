//! TM4C Hibernation Module
//!
//! "The Hibernation Module manages removal and restoration of power to provide a
//! means for reducingsystem power consumption. When the processor and peripherals
//! are idle, power can be completelyremoved with only the Hibernation module
//! remaining powered. Power can be restored based onan external signal or at a
//! certain time using the built-in Real-Time Clock (RTC). The Hibernationmodule
//! can be independently supplied from an external battery or an auxiliary power
//! supply." - The TM4C123GH6PM Reference Manual

use chrono::prelude::*;

/// This object represents the Hibernation peripheral. It consumes the
/// underlying raw object exported by the PAC.
pub struct Hibernation {
    peripheral: tm4c123x::HIB,
}

impl Hibernation {
    /// This causes the RTC to neither gain nor lose speed.
    pub const DEFAULT_TRIM: u16 = 32767;

    /// Create a new high-level object representing the Hibernation module.
    ///
    /// It consumes the low-level object.
    ///
    /// Note, it does not start the RTC - you need to do that yourself.
    ///
    /// A second normally has 32768 (0x8000) ticks in it, numbered
    /// `[0..32767]`. Every 64th second, that second has `trim+1` number of
    /// ticks in it. If `trim` is larger tham 32767 then the RTC runs slightly
    /// slow. If `trim` is less then 32767, then the RTC runs slightly fast.
    /// This lets you compensate for crystal frequency error by +/- 1.5%, at
    /// the expense of 1 second in 64 being a different length to the other 63
    /// seconds.
    pub fn new(peripheral: tm4c123x::HIB, trim: u16, pc: &sysctl::PowerControl) -> Hibernation {
        sysctl::control_power(
            pc,
            sysctl::Domain::Hibernation,
            sysctl::RunMode::Run,
            sysctl::PowerState::On,
        );
        peripheral.rtct.write(|w| w.trim().bits(trim));
        Hibernation { peripheral }
    }

    /// Get your low-level object back, if you need it for some reason.
    pub fn release(self) -> tm4c123x::HIB {
        self.peripheral
    }

    /// Sets the `rtcen` bit to start the RTC ticking.
    pub fn start(&mut self) {
        peripheral.ctl.modify(|_r, w| w.rtcen(true));
    }

    /// Clears the `rtcen` bit to stop the RTC ticking.
    pub fn stop(&mut self) {
        peripheral.ctl.modify(|_r, w| w.rtcen(false));
    }

    /// Sets the current time to the given timestamp. The Hibernation module
    /// isn't TimeZone aware so we take a naive DateTime.
    pub fn set_time(&mut self, time: chrono::NaiveDateTime) {
        let seconds = time.timestamp();
        self.peripheral
            .rtcld
            .write(|w| unsafe { w.bits(seconds as i32) });
    }

    /// Get the current time as (seconds, fractions), where a fraction is
    /// 1/32768 of a second.
    pub fn get_time_raw(&self) -> (u32, u16) {
        loop {
            let seconds = self.peripheral.rtcc.read().bits();
            let subsecs = self.peripheral.rtcss.read().bits();
            let seconds2 = self.peripheral.rtcc.read().bits();
            if (seconds == seconds2) {
                // No wrap - we're good
                return (seconds, subsecs as u16);
            }
        }
    }

    /// Gets the current time as a timestamp. The Hibernation module
    /// isn't TimeZone aware so we return a naive DateTime.
    pub fn get_time(&self) -> chrono::NaiveDateTime {
        let (seconds, subsecs) = self.get_time_raw();
        // Convert from 1/32768 to 1E-9
        let nanos: i64 = subsecs;
        nanos *= 1E9;
        nanos /= 32678;
        chrono::from_timestamp(seconds, nanos)
    }
}
