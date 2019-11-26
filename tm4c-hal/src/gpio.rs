//! Code for GPIO pins

use core::marker::PhantomData;

/// All unlocked pin modes implement this
pub trait IsUnlocked {}

/// All input modes implement this
pub trait InputMode {}

/// All output modes implement this
pub trait OutputMode {}

/// OpenDrain modes implement this
pub trait OpenDrainMode {
    /// Is pull-up enabled
    fn pup() -> bool;
}

/// All the different Alternate Functions you can choose implement this
pub trait AlternateFunctionChoice {
    /// Which Alternate Function (numbered 1 through 15) is this?
    fn number() -> u32;
}

/// Input mode (type state)
pub struct Input<MODE>
where
    MODE: InputMode,
{
    _mode: PhantomData<MODE>,
}
impl<MODE> IsUnlocked for Input<MODE> where MODE: InputMode {}

/// Sub-mode of Input: Floating input (type state)
pub struct Floating;
impl InputMode for Floating {}
impl OpenDrainMode for Floating {
    /// Pull-up is not enabled
    fn pup() -> bool {
        false
    }
}

/// Sub-mode of Input: Pulled down input (type state)
pub struct PullDown;
impl InputMode for PullDown {}

/// Sub-mode of Input: Pulled up input (type state)
pub struct PullUp;
impl InputMode for PullUp {}
impl OpenDrainMode for PullUp {
    /// Pull-up is enabled
    fn pup() -> bool {
        true
    }
}

/// Tri-state
pub struct Tristate;
impl IsUnlocked for Tristate {}

/// Output mode (type state)
pub struct Output<MODE>
where
    MODE: OutputMode,
{
    _mode: PhantomData<MODE>,
}
impl<MODE> IsUnlocked for Output<MODE> where MODE: OutputMode {}

/// AlternateFunction mode (type state for a GPIO pin)
pub struct AlternateFunction<AF, MODE>
where
    AF: AlternateFunctionChoice,
    MODE: OutputMode,
{
    _func: PhantomData<AF>,
    _mode: PhantomData<MODE>,
}
impl<AF, MODE> IsUnlocked for AlternateFunction<AF, MODE>
where
    AF: AlternateFunctionChoice,
    MODE: OutputMode,
{
}

/// Sub-mode of Output/AlternateFunction: Push pull output (type state for
/// Output)
pub struct PushPull;
impl OutputMode for PushPull {}
impl OutputMode for PullDown {}
impl OutputMode for PullUp {}

/// Sub-mode of Output/AlternateFunction: Open drain output (type state for
/// Output)
pub struct OpenDrain<ODM>
where
    ODM: OpenDrainMode,
{
    _pull: PhantomData<ODM>,
}
impl<ODM> OutputMode for OpenDrain<ODM> where ODM: OpenDrainMode {}

/// Alternate function 1 (type state)
pub struct AF1;
impl AlternateFunctionChoice for AF1 {
    fn number() -> u32 {
        1
    }
}

/// Alternate function 2 (type state)
pub struct AF2;
impl AlternateFunctionChoice for AF2 {
    fn number() -> u32 {
        2
    }
}

/// Alternate function 3 (type state)
pub struct AF3;
impl AlternateFunctionChoice for AF3 {
    fn number() -> u32 {
        3
    }
}

/// Alternate function 4 (type state)
pub struct AF4;
impl AlternateFunctionChoice for AF4 {
    fn number() -> u32 {
        4
    }
}

/// Alternate function 5 (type state)
pub struct AF5;
impl AlternateFunctionChoice for AF5 {
    fn number() -> u32 {
        5
    }
}

/// Alternate function 6 (type state)
pub struct AF6;
impl AlternateFunctionChoice for AF6 {
    fn number() -> u32 {
        6
    }
}

/// Alternate function 7 (type state)
pub struct AF7;
impl AlternateFunctionChoice for AF7 {
    fn number() -> u32 {
        7
    }
}

/// Alternate function 8 (type state)
pub struct AF8;
impl AlternateFunctionChoice for AF8 {
    fn number() -> u32 {
        8
    }
}

/// Alternate function 9 (type state)
pub struct AF9;
impl AlternateFunctionChoice for AF9 {
    fn number() -> u32 {
        9
    }
}

// 10 through 13 are not available on this chip.

/// Alternate function 14 (type state)
pub struct AF14;
impl AlternateFunctionChoice for AF14 {
    fn number() -> u32 {
        14
    }
}

/// Pin is locked through the GPIOCR register
pub struct Locked;

/// Sets when a GPIO pin triggers an interrupt.
pub enum InterruptMode {
    /// Interrupt when level is low
    LevelLow,
    /// Interrupt when level is high
    LevelHigh,
    /// Interrupt on rising edge
    EdgeRising,
    /// Interrupt on falling edge
    EdgeFalling,
    /// Interrupt on both rising and falling edges
    EdgeBoth,
    /// Disable interrupts on this pin
    Disabled,
}

// End of file
