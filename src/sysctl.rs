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
    /// The system clock configuration
    pub system_clock: SystemClock,
    // Make this type uncreatable
    _0: (),
}

/// Selects the system oscillator source
pub enum Oscillator {
    /// Use the main oscillator (with the given crystal)
    Main(CrystalFrequency),
    /// Use the 16 MHz precision internal oscillator
    PrecisionInternal,
    /// Use the 16 MHz precision internal oscillator, divided down to 4 MHz
    PrecisionInternalDiv4,
    /// Use the 30 kHz internal oscillator
    LowFrequencyInternal,
}

/// Selects the source for the system clock
pub enum SystemClock {
    /// Clock the system direct from the system oscillator
    UseOscillator(Divider),
    /// Clock the system from the PLL (which is driven by the system
    /// oscillator), divided down from 400MHz to the given frequency.
    UsePll(PllOutputFrequency),
}

/// Selects which crystal is fitted to the XOSC pins.
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
pub enum PllOutputFrequency {
    /// 66.67 MHz
    _66_67mhz,
    /// 50 MHz
    _50_00mhz,
    /// 40 MHz
    _40_00mhz,
    /// 33.33 MHz
    _33_33mhz,
    /// 28.57 MHz
    _28_57mhz,
    /// 25 MHz
    _25mhz,
    /// 22.22 MHz
    _22_22mhz,
    /// 20 MHz
    _20mhz,
    /// 18.18 MHz
    _18_18mhz,
    /// 16.67 MHz
    _16_67mhz,
    /// 15.38 MHz
    _15_38mhz,
    /// 14.29 MHz
    _14_29mhz,
    /// 13.33 MHz
    _13_33mhz,
    /// 12.5 MHz
    _12_5mhz,
}

/// Selects how much to divide the system oscillator down.
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
                oscillator: Oscillator::PrecisionInternal,
                system_clock: SystemClock::UseOscillator(Divider::_1),
                _0: (),
            },
        }
    }
}

impl ClockSetup {
    /// Fix the clock configuration and produce a record of the configuration
    /// so that other modules can calibrate themselves (e.g. the UARTs).
    pub fn freeze(self) -> Clocks {
        // TODO: Pay attention to the clock setup and don't ignore it. The
        // precision internal oscillator is 16 MHz and we run at 1:1 by
        // default with no PLL.

        // We own the SYSCTL at this point - no one else can be running.
        let _p = unsafe {
            &*tm4c123x::SYSCTL::ptr();
        };

        Clocks {
            osc: 16u32.mhz().into(),
            sysclk: 16u32.mhz().into(),
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
