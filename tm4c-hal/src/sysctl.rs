use crate::time::Hertz;

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

#[derive(Copy, Clone)]
/// Select in which mode the peripheral should be affected
pub enum RunMode {
    /// Run mode
    Run,
    /// Sleep mode (i.e. WFI is being executed)
    Sleep,
    /// Deep-Sleep mode (i.e. WFI is being executed with SLEEP DEEP bit set)
    DeepSleep,
}

#[derive(Copy, Clone)]
/// Select whether the peripheral should be on or off
pub enum PowerState {
    /// Turn peripheral clocks/power off
    Off,
    /// Turn peripheral clocks/power on
    On,
}

impl Clocks {
    /// Returns the frequency of the oscillator.
    pub fn osc(self) -> Hertz {
        self.osc
    }

    /// Returns the system (core) frequency
    pub fn sysclk(self) -> Hertz {
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
        /// It's a "Tiva™ Snowflake-class microcontroller"
        Snowflake,
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
        /// It's a Commercial temperature range part (0°C - +70°C)
        Commercial,
        /// It's a Industrial temperature range part (-40°C - +85°C)
        Industrial,
        /// It's a Extended temperature range part (-40°C - +105°C)
        Extended,
        /// It's either Extended or Industrial depending on the exact part
        /// number
        IndustrialOrExtended,
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
        /// It's a TM4C1294NCPDT
        Tm4c1294ncpdt,
        /// It's a TM4C129ENCPDT
        Tm4c129encpdt,
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
}