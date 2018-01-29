//! # System Control
//!
//! The SYSCTL peripheral controls clocks and power.
//!
//! The TM4C123x can be clocked from the Main Oscillator or the PLL, through a
//! divider. The Main Oscillator can be either the internal 16 MHz precision
//! oscillator, a 4 MHz derivation of the same, or an external crystal.
//!
//! SYSCTL has the following registers:
//!
//! * Device ID (class, major, minor, family, package, temperature range, )
//! * Brown-out reset control
//! * Brown-out/PLL interrupt control
//! * Reset cause
//! * Run-time clock configuration

use tm4c123x;
use time::{Hertz, U32Ext};
use cortex_m::asm::nop;

/// Constrained SYSCTL peripheral.
pub struct Sysctl {
    /// Power control methods will require `&mut this.power_control` to
    /// prevent them from running concurrently.
    pub power_control: PowerControl,
    /// Clock configuration will consume this and give you `Clocks`.
    pub clock_setup: ClockSetup,
}

/// Used to gate access to the run-time power control features of the chip.
pub struct PowerControl {
    _0: (),
}

/// Used to configure the clock generators.
pub struct ClockSetup {
    /// The system oscillator configuration
    pub oscillator: Oscillator,
    // Make this type uncreatable
    _0: (),
}

/// Selects the system oscillator source
#[derive(Clone, Copy)]
pub enum Oscillator {
    /// Use the main oscillator (with the given crystal), into the PLL or a clock divider
    Main(CrystalFrequency, SystemClock),
    /// Use the 16 MHz precision internal oscillator, into the PLL or a clock divider
    PrecisionInternal(SystemClock),
    /// Use the 16 MHz precision internal oscillator, divided down to 4 MHz
    /// and then divided down again by the given value.
    PrecisionInternalDiv4(Divider),
    /// Use the 30 kHz internal oscillator, divided by the given value.
    LowFrequencyInternal(Divider),
}

/// Selects the source for the system clock
#[derive(Clone, Copy)]
pub enum SystemClock {
    /// Clock the system direct from the system oscillator
    UseOscillator(Divider),
    /// Clock the system from the PLL (which is driven by the system
    /// oscillator), divided down from 400MHz to the given frequency.
    UsePll(PllOutputFrequency),
}

/// Selects which crystal is fitted to the XOSC pins.
#[derive(Clone, Copy)]
pub enum CrystalFrequency {
    /// 4 MHz
    _4mhz,
    /// 4.096 MHz
    _4_09mhz,
    /// 4.9152 MHz
    _4_91mhz,
    /// 5 MHz
    _5mhz,
    /// 5.12 MHz
    _5_12mhz,
    /// 6 MHz
    _6mhz,
    /// 6.144 MHz
    _6_14mhz,
    /// 7.3728 MHz
    _7_37mhz,
    /// 8 MHz
    _8mhz,
    /// 8.192 MHz
    _8_19mhz,
    /// 10 MHz
    _10mhz,
    /// 12 MHz
    _12mhz,
    /// 12.288 MHz
    _12_2mhz,
    /// 13.56 MHz
    _13_5mhz,
    /// 14.31818 MHz
    _14_3mhz,
    /// 16 MHz
    _16mhz,
    /// 16.384 MHz
    _16_3mhz,
    /// 18.0 MHz (USB)
    _18mhz,
    /// 20.0 MHz (USB)
    _20mhz,
    /// 24.0 MHz (USB)
    _24mhz,
    /// 25.0 MHz (USB)
    _25mhz,
}

/// Selects what to divide the PLL's 400MHz down to.
#[derive(Clone, Copy)]
pub enum PllOutputFrequency {
    /// 66.67 MHz
    _66_67mhz = 2,
    /// 50 MHz
    _50_00mhz = 3,
    /// 40 MHz
    _40_00mhz = 4,
    /// 33.33 MHz
    _33_33mhz = 5,
    /// 28.57 MHz
    _28_57mhz = 6,
    /// 25 MHz
    _25mhz = 7,
    /// 22.22 MHz
    _22_22mhz = 8,
    /// 20 MHz
    _20mhz = 9,
    /// 18.18 MHz
    _18_18mhz = 10,
    /// 16.67 MHz
    _16_67mhz = 11,
    /// 15.38 MHz
    _15_38mhz = 12,
    /// 14.29 MHz
    _14_29mhz = 13,
    /// 13.33 MHz
    _13_33mhz = 14,
    /// 12.5 MHz
    _12_5mhz = 15,
}

/// Selects how much to divide the system oscillator down.
#[derive(Clone, Copy)]
pub enum Divider {
    /// Divide by 1
    _1 = 1,
    /// Divide by 2
    _2 = 2,
    /// Divide by 3
    _3 = 3,
    /// Divide by 4
    _4 = 4,
    /// Divide by 5
    _5 = 5,
    /// Divide by 6
    _6 = 6,
    /// Divide by 7
    _7 = 7,
    /// Divide by 8
    _8 = 8,
    /// Divide by 9
    _9 = 9,
    /// Divide by 10
    _10 = 10,
    /// Divide by 11
    _11 = 11,
    /// Divide by 12
    _12 = 12,
    /// Divide by 13
    _13 = 13,
    /// Divide by 14
    _14 = 14,
    /// Divide by 15
    _15 = 15,
    /// Divide by 16
    _16 = 16,
}

/// Extension trait that constrains the `SYSCTL` peripheral
pub trait SysctlExt {
    /// Constrains the `SYSCTL` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Sysctl;
}

impl SysctlExt for tm4c123x::SYSCTL {
    fn constrain(self) -> Sysctl {
        Sysctl {
            power_control: PowerControl { _0: () },
            clock_setup: ClockSetup {
                oscillator: Oscillator::PrecisionInternal(SystemClock::UseOscillator(Divider::_1)),
                _0: (),
            },
        }
    }
}

impl ClockSetup {
    /// Fix the clock configuration and produce a record of the configuration
    /// so that other modules can calibrate themselves (e.g. the UARTs).
    pub fn freeze(self) -> Clocks {
        // We own the SYSCTL at this point - no one else can be running.
        let p = unsafe { &*tm4c123x::SYSCTL::ptr() };

        let mut osc = 0u32;
        let mut sysclk = 0u32;

        match self.oscillator {
            Oscillator::Main(crystal_frequency, system_clock) => {
                p.rcc.write(|w| {
                    // BYPASS on
                    w.bypass().set_bit();
                    // OSCSRC = Main Oscillator
                    w.oscsrc().main();
                    // Main Oscillator not disabled
                    w.moscdis().clear_bit();
                    // SysDiv = 0x00
                    unsafe {
                        w.sysdiv().bits(0x00);
                    }
                    // Set crystal frequency
                    osc = match crystal_frequency {
                        CrystalFrequency::_4mhz => {
                            w.xtal()._4mhz();
                            4_000_000
                        }
                        CrystalFrequency::_4_09mhz => {
                            w.xtal()._4_09mhz();
                            4_090_000
                        }
                        CrystalFrequency::_4_91mhz => {
                            w.xtal()._4_91mhz();
                            4_910_000
                        }
                        CrystalFrequency::_5mhz => {
                            w.xtal()._5mhz();
                            5_000_000
                        }
                        CrystalFrequency::_5_12mhz => {
                            w.xtal()._5_12mhz();
                            5_120_000
                        }
                        CrystalFrequency::_6mhz => {
                            w.xtal()._6mhz();
                            6_000_000
                        }
                        CrystalFrequency::_6_14mhz => {
                            w.xtal()._6_14mhz();
                            6_140_000
                        }
                        CrystalFrequency::_7_37mhz => {
                            w.xtal()._7_37mhz();
                            7_370_000
                        }
                        CrystalFrequency::_8mhz => {
                            w.xtal()._8mhz();
                            8_000_000
                        }
                        CrystalFrequency::_8_19mhz => {
                            w.xtal()._8_19mhz();
                            8_190_000
                        }
                        CrystalFrequency::_10mhz => {
                            w.xtal()._10mhz();
                            10_000_000
                        }
                        CrystalFrequency::_12mhz => {
                            w.xtal()._12mhz();
                            12_000_000
                        }
                        CrystalFrequency::_12_2mhz => {
                            w.xtal()._12_2mhz();
                            12_200_000
                        }
                        CrystalFrequency::_13_5mhz => {
                            w.xtal()._13_5mhz();
                            13_500_000
                        }
                        CrystalFrequency::_14_3mhz => {
                            w.xtal()._14_3mhz();
                            14_300_000
                        }
                        CrystalFrequency::_16mhz => {
                            w.xtal()._16mhz();
                            16_000_000
                        }
                        CrystalFrequency::_16_3mhz => {
                            w.xtal()._16_3mhz();
                            16_300_000
                        }
                        CrystalFrequency::_18mhz => {
                            w.xtal()._18mhz();
                            18_000_000
                        }
                        CrystalFrequency::_20mhz => {
                            w.xtal()._20mhz();
                            20_000_000
                        }
                        CrystalFrequency::_24mhz => {
                            w.xtal()._24mhz();
                            24_000_000
                        }
                        CrystalFrequency::_25mhz => {
                            w.xtal()._25mhz();
                            25_000_000
                        }
                    };
                    if let SystemClock::UseOscillator(div) = system_clock {
                        w.usesysdiv().set_bit();
                        unsafe {
                            w.sysdiv().bits(div as u8);
                        }
                        sysclk = osc / (div as u32);
                    } else {
                        // Run 1:1 now, do PLL later
                        w.usesysdiv().clear_bit();
                        unsafe {
                            w.sysdiv().bits(0);
                        }
                        sysclk = osc;
                    }
                    w
                });
            }
            // The default
            Oscillator::PrecisionInternal(system_clock) => {
                osc = 16_000_000;
                p.rcc.write(|w| {
                    // BYPASS on
                    w.bypass().set_bit();
                    // OSCSRC = Internal Oscillator
                    w.oscsrc().int();
                    // Main Oscillator disabled
                    w.moscdis().set_bit();
                    // SysDiv = ?
                    if let SystemClock::UseOscillator(div) = system_clock {
                        w.usesysdiv().set_bit();
                        unsafe {
                            w.sysdiv().bits(div as u8);
                        }
                        sysclk = osc / (div as u32);
                    } else {
                        // Run 1:1 now, do PLL later
                        w.usesysdiv().clear_bit();
                        unsafe {
                            w.sysdiv().bits(0);
                        }
                        sysclk = osc;
                    }
                    w
                });
            }
            Oscillator::PrecisionInternalDiv4(div) => {
                osc = 4_000_000;
                p.rcc.write(|w| {
                    // BYPASS on
                    w.bypass().set_bit();
                    // OSCSRC = Internal Oscillator / 4
                    w.oscsrc().int4();
                    // Main Oscillator disabled
                    w.moscdis().set_bit();
                    w.usesysdiv().set_bit();
                    unsafe {
                        w.sysdiv().bits(div as u8);
                    }
                    sysclk = osc / (div as u32);
                    w
                });
            }
            Oscillator::LowFrequencyInternal(div) => {
                osc = 30_000;
                p.rcc.write(|w| {
                    // BYPASS on
                    w.bypass().set_bit();
                    // OSCSRC = Low Frequency internal (30 kHz)
                    w.oscsrc()._30();
                    // Main Oscillator disabled
                    w.moscdis().set_bit();
                    w.usesysdiv().set_bit();
                    unsafe {
                        w.sysdiv().bits(div as u8);
                    }
                    sysclk = osc / (div as u32);
                    w
                });
            }
        }

        match self.oscillator {
            Oscillator::PrecisionInternal(SystemClock::UsePll(f))
            | Oscillator::Main(_, SystemClock::UsePll(f)) => {
                // Configure 400MHz PLL with divider f

                // Set PLL bit in masked interrupt status to clear
                // PLL lock status
                p.misc.write(|w| w.plllmis().set_bit());

                // Enable the PLL
                p.rcc.modify(|_, w| w.pwrdn().clear_bit());

                while p.pllstat.read().lock().bit_is_clear() {
                    nop();
                }

                p.rcc.modify(|_, w| {
                    unsafe { w.sysdiv().bits(f as u8) };
                    w.usesysdiv().set_bit();
                    w.bypass().clear_bit();
                    w
                });

                sysclk = 400_000_000u32 / (2 * ((f as u32) + 1));
            }
            _ => {}
        }

        Clocks {
            osc: osc.hz(),
            sysclk: sysclk.hz(),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    /// System oscillator clock speed
    pub osc: Hertz,
    /// System clock speed
    pub sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the oscillator.
    pub fn osc(&self) -> Hertz {
        self.osc
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}

/// This module is all about identifying the physical chip we're running on.
pub mod chip_id {
    /// Possible errors we can get back when parsing the ID registers.
    #[derive(Debug)]
    pub enum Error {
        /// Unknown value in DID0
        UnknownDid0Ver(u8),
        /// Unknown value in DID1
        UnknownDid1Ver(u8),
    }

    /// What sort of device is this?
    #[derive(Debug)]
    pub enum DeviceClass {
        /// It's a Stellaris LM4F or a TM4C123 (they have the same value)
        StellarisBlizzard,
        /// I don't know what chip this is
        Unknown,
    }

    /// How many pins on this chip's package?
    #[derive(Debug)]
    pub enum PinCount {
        /// It's a 28 pin package
        _28,
        /// It's a 48 pin package
        _48,
        /// It's a 100 pin package
        _100,
        /// It's a 64 pin package
        _64,
        /// It's a 144 pin package
        _144,
        /// It's a 157 pin package
        _157,
        /// It's a 168 pin package (TM4C123 only)
        _168,
        /// I don't know what chip this is
        Unknown,
    }

    /// What temperature range does this chip support?
    #[derive(Debug)]
    pub enum TempRange {
        /// It's Commercial temperature range (0°C - +70°C)
        Commercial,
        /// It's Industrial temperature range (-40°C - +85°C)
        Industrial,
        /// It's Extended temperature range (-40°C - +105°C)
        Extended,
        /// I don't know what temperature range this is
        Unknown,
    }

    /// What package is this chip in?
    #[derive(Debug)]
    pub enum Package {
        /// It's a SOIC package
        Soic,
        /// It's a LQFP package
        Lqfp,
        /// It's a BGA package
        Bga,
        /// I don't know what package this is
        Unknown,
    }

    /// Is this an experimental chip or a production part?
    #[derive(Debug)]
    pub enum Qualification {
        /// It's a Engineering Sample chip
        EngineeringSample,
        /// It's a Pilot Production chip
        PilotProduction,
        /// It's a Fully Qualified chip
        FullyQualified,
        /// I don't know what qualification this is
        Unknown,
    }

    /// These values describe the part number
    #[derive(Debug)]
    pub enum PartNo {
        /// It's a TM4C123GH6PM
        Tm4c123gh6pm,
        /// It's a LM4F120H5QR
        Lm4f120h5qr,
        /// It's an unknown chip - please file a bug report
        Unknown(u8),
    }

    /// These values describe the physical LM4F/TM4C chip
    #[derive(Debug)]
    pub struct ChipId {
        /// The device class
        pub device_class: DeviceClass,
        /// The major revision
        pub major: u8,
        /// The minor revision
        pub minor: u8,
        /// The chip's pin count
        pub pin_count: PinCount,
        /// The chip's temperature range
        pub temp_range: TempRange,
        /// The chip's package
        pub package: Package,
        /// True if the chip is RoHS compliant
        pub rohs_compliant: bool,
        /// The chip's qualification
        pub qualification: Qualification,
        /// The chip's part number
        pub part_no: PartNo,
    }

    /// Read DID0 and DID1 to discover what sort of
    /// TM4C123/LM4F123 this is.
    pub fn get() -> Result<ChipId, Error> {
        use tm4c123x;
        // This is safe as it's read only
        let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
        let did0 = p.did0.read();
        if did0.ver().bits() != 0x01 {
            return Err(Error::UnknownDid0Ver(did0.ver().bits()));
        }
        let device_class = match did0.class().bits() {
            5 => DeviceClass::StellarisBlizzard,
            _ => DeviceClass::Unknown,
        };
        let major = did0.maj().bits();
        let minor = did0.min().bits();
        let did1 = p.did1.read();
        if did1.ver().bits() != 0x01 {
            // Stellaris LM3F (0x00) is not supported
            return Err(Error::UnknownDid1Ver(did1.ver().bits()));
        }
        let part_no = match did1.prtno().bits() {
            0x04 => PartNo::Lm4f120h5qr,
            0xA1 => PartNo::Tm4c123gh6pm,
            _ => PartNo::Unknown(did1.prtno().bits()),
        };
        let pin_count = match did1.pincnt().bits() {
            0 => PinCount::_28,
            1 => PinCount::_48,
            2 => PinCount::_100,
            3 => PinCount::_64,
            4 => PinCount::_144,
            5 => PinCount::_157,
            6 => PinCount::_168,
            _ => PinCount::Unknown,
        };
        let temp_range = match did1.temp().bits() {
            0 => TempRange::Commercial,
            1 => TempRange::Industrial,
            2 => TempRange::Extended,
            _ => TempRange::Unknown,
        };
        let package = match did1.pkg().bits() {
            0 => Package::Soic,
            1 => Package::Lqfp,
            2 => Package::Bga,
            _ => Package::Unknown,
        };
        let rohs_compliant = did1.rohs().bit_is_set();
        let qualification = match did1.qual().bits() {
            0 => Qualification::EngineeringSample,
            1 => Qualification::PilotProduction,
            2 => Qualification::FullyQualified,
            _ => Qualification::Unknown,
        };
        Ok(ChipId {
            device_class,
            major,
            minor,
            pin_count,
            temp_range,
            package,
            rohs_compliant,
            qualification,
            part_no,
        })
    }
}

// End of file
