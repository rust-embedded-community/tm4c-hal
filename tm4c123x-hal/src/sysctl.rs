//! # System Control
//!
//! The SYSCTL peripheral controls clocks and power.
//!
//! The TM4C123x can be clocked from the Main Oscillator or the PLL, through a
//! divider. The Main Oscillator can be either the internal 16 MHz precision
//! oscillator, a 4 MHz derivation of the same, or an external crystal.
//!
//! SYSCTL includes the following registers:
//!
//! * Device ID (class, major, minor, family, package, temperature range, )
//! * Brown-out reset control
//! * Brown-out/PLL interrupt control
//! * Reset cause
//! * Run-time clock configuration
//! * GPIO high-performance bus control
//! * System Properties
//! * Registers to indicate whether peripherals are present
//! * Registers to reset peripherals
//! * Registers to enable/disable clocking of peripherals
//!
//! See the LM4F120 datasheet, page 228 for a full list.

pub use tm4c_hal::sysctl::*;

use crate::{
    bb,
    time::{Hertz, U32Ext},
};
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

impl Into<Hertz> for CrystalFrequency {
    fn into(self) -> Hertz {
        Hertz(match self {
            CrystalFrequency::_4mhz => 4_000_000,
            CrystalFrequency::_4_09mhz => 4_090_000,
            CrystalFrequency::_4_91mhz => 4_910_000,
            CrystalFrequency::_5mhz => 5_000_000,
            CrystalFrequency::_5_12mhz => 5_120_000,
            CrystalFrequency::_6mhz => 6_000_000,
            CrystalFrequency::_6_14mhz => 6_140_000,
            CrystalFrequency::_7_37mhz => 7_370_000,
            CrystalFrequency::_8mhz => 8_000_000,
            CrystalFrequency::_8_19mhz => 8_190_000,
            CrystalFrequency::_10mhz => 10_000_000,
            CrystalFrequency::_12mhz => 12_000_000,
            CrystalFrequency::_12_2mhz => 12_200_000,
            CrystalFrequency::_13_5mhz => 13_500_000,
            CrystalFrequency::_14_3mhz => 14_300_000,
            CrystalFrequency::_16mhz => 16_000_000,
            CrystalFrequency::_16_3mhz => 16_300_000,
            CrystalFrequency::_18mhz => 18_000_000,
            CrystalFrequency::_20mhz => 20_000_000,
            CrystalFrequency::_24mhz => 24_000_000,
            CrystalFrequency::_25mhz => 25_000_000,
        })
    }
}

/// Selects what to divide the PLL's 400MHz down to.
#[derive(Clone, Copy)]
pub enum PllOutputFrequency {
    /// 80.00 MHz
    _80_00mhz = 0,
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

impl Into<Hertz> for PllOutputFrequency {
    fn into(self) -> Hertz {
        Hertz(match self {
            PllOutputFrequency::_80_00mhz => 80_000_000,
            PllOutputFrequency::_66_67mhz => 66_670_000,
            PllOutputFrequency::_50_00mhz => 50_000_000,
            PllOutputFrequency::_40_00mhz => 40_000_000,
            PllOutputFrequency::_33_33mhz => 33_330_000,
            PllOutputFrequency::_28_57mhz => 28_570_000,
            PllOutputFrequency::_25mhz => 25_000_000,
            PllOutputFrequency::_22_22mhz => 22_220_000,
            PllOutputFrequency::_20mhz => 20_000_000,
            PllOutputFrequency::_18_18mhz => 18_180_000,
            PllOutputFrequency::_16_67mhz => 16_670_000,
            PllOutputFrequency::_15_38mhz => 15_380_000,
            PllOutputFrequency::_14_29mhz => 14_290_000,
            PllOutputFrequency::_13_33mhz => 13_330_000,
            PllOutputFrequency::_12_5mhz => 12_500_000,
        })
    }
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

/// List of peripherals that can be enabled or disabled
#[derive(Copy, Clone)]
pub enum Domain {
    /// Watchdog 1
    Watchdog1,
    /// Watchdog 0
    Watchdog0,
    /// 32/16-bit Timer 5
    Timer5,
    /// 32/16-bit Timer 4
    Timer4,
    /// 32/16-bit Timer 3
    Timer3,
    /// 32/16-bit Timer 2
    Timer2,
    /// 32/16-bit Timer 1
    Timer1,
    /// 32/16-bit Timer 0
    Timer0,
    /// GPIO F
    GpioF,
    /// GPIO E
    GpioE,
    /// GPIO D
    GpioD,
    /// GPIO C
    GpioC,
    /// GPIO B
    GpioB,
    /// GPIO A
    GpioA,
    /// µDMA
    MicroDma,
    /// Hibernation
    Hibernation,
    /// UART 7
    Uart7,
    /// UART 6
    Uart6,
    /// UART 5
    Uart5,
    /// UART 4
    Uart4,
    /// UART 3
    Uart3,
    /// UART 2
    Uart2,
    /// UART 1
    Uart1,
    /// UART 0
    Uart0,
    /// SSI 3
    Ssi3,
    /// SSI 2
    Ssi2,
    /// SSI 1
    Ssi1,
    /// SSI 0
    Ssi0,
    /// I2C 3
    I2c3,
    /// I2C 2
    I2c2,
    /// I2C 1
    I2c1,
    /// I2C 0
    I2c0,
    /// USB
    Usb,
    /// CAN
    Can,
    /// ADC 1
    Adc1,
    /// ADC 0
    Adc0,
    /// Analog Comparator
    AnalogComparator,
    /// EEPROM
    Eeprom,
    /// 64/32-bit Timer 5
    WideTimer5,
    /// 64/32-bit Timer 4
    WideTimer4,
    /// 64/32-bit Timer 3
    WideTimer3,
    /// 64/32-bit Timer 2
    WideTimer2,
    /// 64/32-bit Timer 1
    WideTimer1,
    /// 64/32-bit Timer 0
    WideTimer0,
    /// PWM0
    Pwm0,
    /// PWM1
    Pwm1,
}

/// Reset a peripheral
pub fn reset(_lock: &PowerControl, pd: Domain) {
    // We use bit-banding to make an atomic write, so this is safe
    let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
    match pd {
        Domain::Watchdog1 => unsafe {
            bb::toggle_bit(&p.srwd, 1);
            bb::spin_bit(&p.prwd, 1);
        },
        Domain::Watchdog0 => unsafe {
            bb::toggle_bit(&p.srwd, 0);
            bb::spin_bit(&p.prwd, 0);
        },
        Domain::Timer5 => unsafe {
            bb::toggle_bit(&p.srtimer, 5);
            bb::spin_bit(&p.prtimer, 5);
        },
        Domain::Timer4 => unsafe {
            bb::toggle_bit(&p.srtimer, 4);
            bb::spin_bit(&p.prtimer, 4);
        },
        Domain::Timer3 => unsafe {
            bb::toggle_bit(&p.srtimer, 3);
            bb::spin_bit(&p.prtimer, 3);
        },
        Domain::Timer2 => unsafe {
            bb::toggle_bit(&p.srtimer, 2);
            bb::spin_bit(&p.prtimer, 2);
        },
        Domain::Timer1 => unsafe {
            bb::toggle_bit(&p.srtimer, 1);
            bb::spin_bit(&p.prtimer, 1);
        },
        Domain::Timer0 => unsafe {
            bb::toggle_bit(&p.srtimer, 0);
            bb::spin_bit(&p.prtimer, 0);
        },
        Domain::GpioF => unsafe {
            bb::toggle_bit(&p.srgpio, 5);
            bb::spin_bit(&p.prgpio, 5);
        },
        Domain::GpioE => unsafe {
            bb::toggle_bit(&p.srgpio, 4);
            bb::spin_bit(&p.prgpio, 4);
        },
        Domain::GpioD => unsafe {
            bb::toggle_bit(&p.srgpio, 3);
            bb::spin_bit(&p.prgpio, 3);
        },
        Domain::GpioC => unsafe {
            bb::toggle_bit(&p.srgpio, 2);
            bb::spin_bit(&p.prgpio, 2);
        },
        Domain::GpioB => unsafe {
            bb::toggle_bit(&p.srgpio, 1);
            bb::spin_bit(&p.prgpio, 1);
        },
        Domain::GpioA => unsafe {
            bb::toggle_bit(&p.srgpio, 0);
            bb::spin_bit(&p.prgpio, 0);
        },
        Domain::MicroDma => unsafe {
            bb::toggle_bit(&p.srdma, 0);
            bb::spin_bit(&p.prdma, 0);
        },
        Domain::Hibernation => unsafe {
            bb::toggle_bit(&p.srhib, 0);
            bb::spin_bit(&p.prhib, 0);
        },
        Domain::Uart7 => unsafe {
            bb::toggle_bit(&p.sruart, 7);
            bb::spin_bit(&p.pruart, 7);
        },
        Domain::Uart6 => unsafe {
            bb::toggle_bit(&p.sruart, 6);
            bb::spin_bit(&p.pruart, 6);
        },
        Domain::Uart5 => unsafe {
            bb::toggle_bit(&p.sruart, 5);
            bb::spin_bit(&p.pruart, 5);
        },
        Domain::Uart4 => unsafe {
            bb::toggle_bit(&p.sruart, 4);
            bb::spin_bit(&p.pruart, 4);
        },
        Domain::Uart3 => unsafe {
            bb::toggle_bit(&p.sruart, 3);
            bb::spin_bit(&p.pruart, 3);
        },
        Domain::Uart2 => unsafe {
            bb::toggle_bit(&p.sruart, 2);
            bb::spin_bit(&p.pruart, 2);
        },
        Domain::Uart1 => unsafe {
            bb::toggle_bit(&p.sruart, 1);
            bb::spin_bit(&p.pruart, 1);
        },
        Domain::Uart0 => unsafe {
            bb::toggle_bit(&p.sruart, 0);
            bb::spin_bit(&p.pruart, 0);
        },
        Domain::Ssi3 => unsafe {
            bb::toggle_bit(&p.srssi, 3);
            bb::spin_bit(&p.prssi, 3);
        },
        Domain::Ssi2 => unsafe {
            bb::toggle_bit(&p.srssi, 2);
            bb::spin_bit(&p.prssi, 2);
        },
        Domain::Ssi1 => unsafe {
            bb::toggle_bit(&p.srssi, 1);
            bb::spin_bit(&p.prssi, 1);
        },
        Domain::Ssi0 => unsafe {
            bb::toggle_bit(&p.srssi, 0);
            bb::spin_bit(&p.prssi, 0);
        },
        Domain::I2c3 => unsafe {
            bb::toggle_bit(&p.sri2c, 3);
            bb::spin_bit(&p.pri2c, 3);
        },
        Domain::I2c2 => unsafe {
            bb::toggle_bit(&p.sri2c, 2);
            bb::spin_bit(&p.pri2c, 2);
        },
        Domain::I2c1 => unsafe {
            bb::toggle_bit(&p.sri2c, 1);
            bb::spin_bit(&p.pri2c, 1);
        },
        Domain::I2c0 => unsafe {
            bb::toggle_bit(&p.sri2c, 0);
            bb::spin_bit(&p.pri2c, 0);
        },
        Domain::Usb => unsafe {
            bb::toggle_bit(&p.srusb, 0);
            bb::spin_bit(&p.prusb, 0);
        },
        Domain::Can => unsafe {
            bb::toggle_bit(&p.srcan, 0);
            bb::spin_bit(&p.prcan, 0);
        },
        Domain::Adc1 => unsafe {
            bb::toggle_bit(&p.sradc, 1);
            bb::spin_bit(&p.pradc, 1);
        },
        Domain::Adc0 => unsafe {
            bb::toggle_bit(&p.sradc, 0);
            bb::spin_bit(&p.pradc, 0);
        },
        Domain::AnalogComparator => unsafe {
            bb::toggle_bit(&p.sracmp, 0);
            bb::spin_bit(&p.pracmp, 0);
        },
        Domain::Eeprom => unsafe {
            bb::toggle_bit(&p.sreeprom, 0);
            bb::spin_bit(&p.preeprom, 0);
        },
        Domain::WideTimer5 => unsafe {
            bb::toggle_bit(&p.srwtimer, 5);
            bb::spin_bit(&p.prwtimer, 5);
        },
        Domain::WideTimer4 => unsafe {
            bb::toggle_bit(&p.srwtimer, 4);
            bb::spin_bit(&p.prwtimer, 4);
        },
        Domain::WideTimer3 => unsafe {
            bb::toggle_bit(&p.srwtimer, 3);
            bb::spin_bit(&p.prwtimer, 3);
        },
        Domain::WideTimer2 => unsafe {
            bb::toggle_bit(&p.srwtimer, 2);
            bb::spin_bit(&p.prwtimer, 2);
        },
        Domain::WideTimer1 => unsafe {
            bb::toggle_bit(&p.srwtimer, 1);
            bb::spin_bit(&p.prwtimer, 1);
        },
        Domain::WideTimer0 => unsafe {
            bb::toggle_bit(&p.srwtimer, 0);
            bb::spin_bit(&p.prwtimer, 0);
        },
        Domain::Pwm0 => unsafe {
            bb::toggle_bit(&p.srpwm, 0);
            bb::spin_bit(&p.prpwm, 0);
        },
        Domain::Pwm1 => unsafe {
            bb::toggle_bit(&p.srpwm, 1);
            bb::spin_bit(&p.prpwm, 1);
        },
    }
}

/// Activate or De-Activate clocks and power to the given peripheral in the
/// given run mode.
///
/// We take a reference to PowerControl as a permission check. We don't need
/// an &mut reference as we use atomic writes in the bit-banding area so it's
/// interrupt safe.
pub fn control_power(_lock: &PowerControl, pd: Domain, run_mode: RunMode, state: PowerState) {
    let on = match state {
        PowerState::On => true,
        PowerState::Off => false,
    };
    match run_mode {
        RunMode::Run => control_run_power(pd, on),
        RunMode::Sleep => control_sleep_power(pd, on),
        RunMode::DeepSleep => control_deep_sleep_power(pd, on),
    }
    // Section 5.2.6 - "There must be a delay of 3 system clocks after a
    // peripheral module clock is enabled in the RCGC register before any
    // module registers are accessed."
    nop();
    nop();
    nop();
}

fn control_run_power(pd: Domain, on: bool) {
    // We use bit-banding to make an atomic write, so this is safe
    let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
    match pd {
        Domain::Watchdog1 => unsafe { bb::change_bit(&p.rcgcwd, 1, on) },
        Domain::Watchdog0 => unsafe { bb::change_bit(&p.rcgcwd, 0, on) },
        Domain::Timer5 => unsafe { bb::change_bit(&p.rcgctimer, 5, on) },
        Domain::Timer4 => unsafe { bb::change_bit(&p.rcgctimer, 4, on) },
        Domain::Timer3 => unsafe { bb::change_bit(&p.rcgctimer, 3, on) },
        Domain::Timer2 => unsafe { bb::change_bit(&p.rcgctimer, 2, on) },
        Domain::Timer1 => unsafe { bb::change_bit(&p.rcgctimer, 1, on) },
        Domain::Timer0 => unsafe { bb::change_bit(&p.rcgctimer, 0, on) },
        Domain::GpioF => unsafe { bb::change_bit(&p.rcgcgpio, 5, on) },
        Domain::GpioE => unsafe { bb::change_bit(&p.rcgcgpio, 4, on) },
        Domain::GpioD => unsafe { bb::change_bit(&p.rcgcgpio, 3, on) },
        Domain::GpioC => unsafe { bb::change_bit(&p.rcgcgpio, 2, on) },
        Domain::GpioB => unsafe { bb::change_bit(&p.rcgcgpio, 1, on) },
        Domain::GpioA => unsafe { bb::change_bit(&p.rcgcgpio, 0, on) },
        Domain::MicroDma => unsafe { bb::change_bit(&p.rcgcdma, 0, on) },
        Domain::Hibernation => unsafe { bb::change_bit(&p.rcgchib, 0, on) },
        Domain::Uart7 => unsafe { bb::change_bit(&p.rcgcuart, 7, on) },
        Domain::Uart6 => unsafe { bb::change_bit(&p.rcgcuart, 6, on) },
        Domain::Uart5 => unsafe { bb::change_bit(&p.rcgcuart, 5, on) },
        Domain::Uart4 => unsafe { bb::change_bit(&p.rcgcuart, 4, on) },
        Domain::Uart3 => unsafe { bb::change_bit(&p.rcgcuart, 3, on) },
        Domain::Uart2 => unsafe { bb::change_bit(&p.rcgcuart, 2, on) },
        Domain::Uart1 => unsafe { bb::change_bit(&p.rcgcuart, 1, on) },
        Domain::Uart0 => unsafe { bb::change_bit(&p.rcgcuart, 0, on) },
        Domain::Ssi3 => unsafe { bb::change_bit(&p.rcgcssi, 3, on) },
        Domain::Ssi2 => unsafe { bb::change_bit(&p.rcgcssi, 2, on) },
        Domain::Ssi1 => unsafe { bb::change_bit(&p.rcgcssi, 1, on) },
        Domain::Ssi0 => unsafe { bb::change_bit(&p.rcgcssi, 0, on) },
        Domain::I2c3 => unsafe { bb::change_bit(&p.rcgci2c, 3, on) },
        Domain::I2c2 => unsafe { bb::change_bit(&p.rcgci2c, 2, on) },
        Domain::I2c1 => unsafe { bb::change_bit(&p.rcgci2c, 1, on) },
        Domain::I2c0 => unsafe { bb::change_bit(&p.rcgci2c, 0, on) },
        Domain::Usb => unsafe { bb::change_bit(&p.rcgcusb, 0, on) },
        Domain::Can => unsafe { bb::change_bit(&p.rcgccan, 0, on) },
        Domain::Adc1 => unsafe { bb::change_bit(&p.rcgcadc, 1, on) },
        Domain::Adc0 => unsafe { bb::change_bit(&p.rcgcadc, 0, on) },
        Domain::AnalogComparator => unsafe { bb::change_bit(&p.rcgcacmp, 0, on) },
        Domain::Eeprom => unsafe { bb::change_bit(&p.rcgceeprom, 0, on) },
        Domain::WideTimer5 => unsafe { bb::change_bit(&p.rcgcwtimer, 5, on) },
        Domain::WideTimer4 => unsafe { bb::change_bit(&p.rcgcwtimer, 4, on) },
        Domain::WideTimer3 => unsafe { bb::change_bit(&p.rcgcwtimer, 3, on) },
        Domain::WideTimer2 => unsafe { bb::change_bit(&p.rcgcwtimer, 2, on) },
        Domain::WideTimer1 => unsafe { bb::change_bit(&p.rcgcwtimer, 1, on) },
        Domain::WideTimer0 => unsafe { bb::change_bit(&p.rcgcwtimer, 0, on) },
        Domain::Pwm0 => unsafe { bb::change_bit(&p.rcgcpwm, 0, on) },
        Domain::Pwm1 => unsafe { bb::change_bit(&p.rcgcpwm, 1, on) },
    }
}

fn control_sleep_power(pd: Domain, on: bool) {
    // We use bit-banding to make an atomic write, so this is safe
    let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
    match pd {
        Domain::Watchdog1 => unsafe { bb::change_bit(&p.scgcwd, 1, on) },
        Domain::Watchdog0 => unsafe { bb::change_bit(&p.scgcwd, 0, on) },
        Domain::Timer5 => unsafe { bb::change_bit(&p.scgctimer, 5, on) },
        Domain::Timer4 => unsafe { bb::change_bit(&p.scgctimer, 4, on) },
        Domain::Timer3 => unsafe { bb::change_bit(&p.scgctimer, 3, on) },
        Domain::Timer2 => unsafe { bb::change_bit(&p.scgctimer, 2, on) },
        Domain::Timer1 => unsafe { bb::change_bit(&p.scgctimer, 1, on) },
        Domain::Timer0 => unsafe { bb::change_bit(&p.scgctimer, 0, on) },
        Domain::GpioF => unsafe { bb::change_bit(&p.scgcgpio, 5, on) },
        Domain::GpioE => unsafe { bb::change_bit(&p.scgcgpio, 4, on) },
        Domain::GpioD => unsafe { bb::change_bit(&p.scgcgpio, 3, on) },
        Domain::GpioC => unsafe { bb::change_bit(&p.scgcgpio, 2, on) },
        Domain::GpioB => unsafe { bb::change_bit(&p.scgcgpio, 1, on) },
        Domain::GpioA => unsafe { bb::change_bit(&p.scgcgpio, 0, on) },
        Domain::MicroDma => unsafe { bb::change_bit(&p.scgcdma, 0, on) },
        Domain::Hibernation => unsafe { bb::change_bit(&p.scgchib, 0, on) },
        Domain::Uart7 => unsafe { bb::change_bit(&p.scgcuart, 7, on) },
        Domain::Uart6 => unsafe { bb::change_bit(&p.scgcuart, 6, on) },
        Domain::Uart5 => unsafe { bb::change_bit(&p.scgcuart, 5, on) },
        Domain::Uart4 => unsafe { bb::change_bit(&p.scgcuart, 4, on) },
        Domain::Uart3 => unsafe { bb::change_bit(&p.scgcuart, 3, on) },
        Domain::Uart2 => unsafe { bb::change_bit(&p.scgcuart, 2, on) },
        Domain::Uart1 => unsafe { bb::change_bit(&p.scgcuart, 1, on) },
        Domain::Uart0 => unsafe { bb::change_bit(&p.scgcuart, 0, on) },
        Domain::Ssi3 => unsafe { bb::change_bit(&p.scgcssi, 3, on) },
        Domain::Ssi2 => unsafe { bb::change_bit(&p.scgcssi, 2, on) },
        Domain::Ssi1 => unsafe { bb::change_bit(&p.scgcssi, 1, on) },
        Domain::Ssi0 => unsafe { bb::change_bit(&p.scgcssi, 0, on) },
        Domain::I2c3 => unsafe { bb::change_bit(&p.scgci2c, 3, on) },
        Domain::I2c2 => unsafe { bb::change_bit(&p.scgci2c, 2, on) },
        Domain::I2c1 => unsafe { bb::change_bit(&p.scgci2c, 1, on) },
        Domain::I2c0 => unsafe { bb::change_bit(&p.scgci2c, 0, on) },
        Domain::Usb => unsafe { bb::change_bit(&p.scgcusb, 0, on) },
        Domain::Can => unsafe { bb::change_bit(&p.scgccan, 0, on) },
        Domain::Adc1 => unsafe { bb::change_bit(&p.scgcadc, 1, on) },
        Domain::Adc0 => unsafe { bb::change_bit(&p.scgcadc, 0, on) },
        Domain::AnalogComparator => unsafe { bb::change_bit(&p.scgcacmp, 0, on) },
        Domain::Eeprom => unsafe { bb::change_bit(&p.scgceeprom, 0, on) },
        Domain::WideTimer5 => unsafe { bb::change_bit(&p.scgcwtimer, 5, on) },
        Domain::WideTimer4 => unsafe { bb::change_bit(&p.scgcwtimer, 4, on) },
        Domain::WideTimer3 => unsafe { bb::change_bit(&p.scgcwtimer, 3, on) },
        Domain::WideTimer2 => unsafe { bb::change_bit(&p.scgcwtimer, 2, on) },
        Domain::WideTimer1 => unsafe { bb::change_bit(&p.scgcwtimer, 1, on) },
        Domain::WideTimer0 => unsafe { bb::change_bit(&p.scgcwtimer, 0, on) },
        Domain::Pwm0 => unsafe { bb::change_bit(&p.scgcpwm, 0, on) },
        Domain::Pwm1 => unsafe { bb::change_bit(&p.scgcpwm, 1, on) },
    }
}

fn control_deep_sleep_power(pd: Domain, on: bool) {
    // We use bit-banding to make an atomic write, so this is safe
    let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
    match pd {
        Domain::Watchdog1 => unsafe { bb::change_bit(&p.dcgcwd, 1, on) },
        Domain::Watchdog0 => unsafe { bb::change_bit(&p.dcgcwd, 0, on) },
        Domain::Timer5 => unsafe { bb::change_bit(&p.dcgctimer, 5, on) },
        Domain::Timer4 => unsafe { bb::change_bit(&p.dcgctimer, 4, on) },
        Domain::Timer3 => unsafe { bb::change_bit(&p.dcgctimer, 3, on) },
        Domain::Timer2 => unsafe { bb::change_bit(&p.dcgctimer, 2, on) },
        Domain::Timer1 => unsafe { bb::change_bit(&p.dcgctimer, 1, on) },
        Domain::Timer0 => unsafe { bb::change_bit(&p.dcgctimer, 0, on) },
        Domain::GpioF => unsafe { bb::change_bit(&p.dcgcgpio, 5, on) },
        Domain::GpioE => unsafe { bb::change_bit(&p.dcgcgpio, 4, on) },
        Domain::GpioD => unsafe { bb::change_bit(&p.dcgcgpio, 3, on) },
        Domain::GpioC => unsafe { bb::change_bit(&p.dcgcgpio, 2, on) },
        Domain::GpioB => unsafe { bb::change_bit(&p.dcgcgpio, 1, on) },
        Domain::GpioA => unsafe { bb::change_bit(&p.dcgcgpio, 0, on) },
        Domain::MicroDma => unsafe { bb::change_bit(&p.dcgcdma, 0, on) },
        Domain::Hibernation => unsafe { bb::change_bit(&p.dcgchib, 0, on) },
        Domain::Uart7 => unsafe { bb::change_bit(&p.dcgcuart, 7, on) },
        Domain::Uart6 => unsafe { bb::change_bit(&p.dcgcuart, 6, on) },
        Domain::Uart5 => unsafe { bb::change_bit(&p.dcgcuart, 5, on) },
        Domain::Uart4 => unsafe { bb::change_bit(&p.dcgcuart, 4, on) },
        Domain::Uart3 => unsafe { bb::change_bit(&p.dcgcuart, 3, on) },
        Domain::Uart2 => unsafe { bb::change_bit(&p.dcgcuart, 2, on) },
        Domain::Uart1 => unsafe { bb::change_bit(&p.dcgcuart, 1, on) },
        Domain::Uart0 => unsafe { bb::change_bit(&p.dcgcuart, 0, on) },
        Domain::Ssi3 => unsafe { bb::change_bit(&p.dcgcssi, 3, on) },
        Domain::Ssi2 => unsafe { bb::change_bit(&p.dcgcssi, 2, on) },
        Domain::Ssi1 => unsafe { bb::change_bit(&p.dcgcssi, 1, on) },
        Domain::Ssi0 => unsafe { bb::change_bit(&p.dcgcssi, 0, on) },
        Domain::I2c3 => unsafe { bb::change_bit(&p.dcgci2c, 3, on) },
        Domain::I2c2 => unsafe { bb::change_bit(&p.dcgci2c, 2, on) },
        Domain::I2c1 => unsafe { bb::change_bit(&p.dcgci2c, 1, on) },
        Domain::I2c0 => unsafe { bb::change_bit(&p.dcgci2c, 0, on) },
        Domain::Usb => unsafe { bb::change_bit(&p.dcgcusb, 0, on) },
        Domain::Can => unsafe { bb::change_bit(&p.dcgccan, 0, on) },
        Domain::Adc1 => unsafe { bb::change_bit(&p.dcgcadc, 1, on) },
        Domain::Adc0 => unsafe { bb::change_bit(&p.dcgcadc, 0, on) },
        Domain::AnalogComparator => unsafe { bb::change_bit(&p.dcgcacmp, 0, on) },
        Domain::Eeprom => unsafe { bb::change_bit(&p.dcgceeprom, 0, on) },
        Domain::WideTimer5 => unsafe { bb::change_bit(&p.dcgcwtimer, 5, on) },
        Domain::WideTimer4 => unsafe { bb::change_bit(&p.dcgcwtimer, 4, on) },
        Domain::WideTimer3 => unsafe { bb::change_bit(&p.dcgcwtimer, 3, on) },
        Domain::WideTimer2 => unsafe { bb::change_bit(&p.dcgcwtimer, 2, on) },
        Domain::WideTimer1 => unsafe { bb::change_bit(&p.dcgcwtimer, 1, on) },
        Domain::WideTimer0 => unsafe { bb::change_bit(&p.dcgcwtimer, 0, on) },
        Domain::Pwm0 => unsafe { bb::change_bit(&p.dcgcpwm, 0, on) },
        Domain::Pwm1 => unsafe { bb::change_bit(&p.dcgcpwm, 1, on) },
    }
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
                            w.sysdiv().bits(div as u8 - 1);
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
                            w.sysdiv().bits(div as u8 - 1);
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
                        w.sysdiv().bits(div as u8 - 1);
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
                        w.sysdiv().bits(div as u8 - 1);
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

                match f {
                    // We need to use RCC2 for this one
                    PllOutputFrequency::_80_00mhz => {
                        p.rcc2.write(|w| {
                            w.usercc2().set_bit();
                            // Divide 400 MHz not 200 MHz
                            w.div400().set_bit();
                            // div=2 with lsb=0 gives divide by 5, so 400 MHz => 80 MHz
                            w.sysdiv2lsb().clear_bit();
                            unsafe { w.sysdiv2().bits(2) };
                            w.bypass2().clear_bit();
                            w
                        });
                        sysclk = 400_000_000u32 / 5;
                    }
                    _ => {
                        // All the other frequencies can be done with legacy registers
                        p.rcc.modify(|_, w| {
                            unsafe { w.sysdiv().bits(f as u8) };
                            w.usesysdiv().set_bit();
                            w.bypass().clear_bit();
                            w
                        });
                        sysclk = 400_000_000u32 / (2 * ((f as u32) + 1));
                    }
                }
            }
            _ => {}
        }

        Clocks {
            osc: osc.hz(),
            sysclk: sysclk.hz(),
        }
    }
}

impl PowerControl {}

/// This module is all about identifying the physical chip we're running on.
pub mod chip_id {
    pub use tm4c_hal::sysctl::chip_id::*;

    /// Read DID0 and DID1 to discover what sort of
    /// TM4C123/LM4F120 this is.
    pub fn get() -> Result<ChipId, Error> {
        // This is safe as it's read only
        let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
        let did0 = p.did0.read();
        if did0.ver().bits() != 0x01 {
            return Err(Error::UnknownDid0Ver(did0.ver().bits()));
        }
        let device_class = match did0.class().bits() {
            0x05 => DeviceClass::StellarisBlizzard,
            0x0a => DeviceClass::Snowflake,
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
            e => PartNo::Unknown(e),
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
            3 => TempRange::IndustrialOrExtended,
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
