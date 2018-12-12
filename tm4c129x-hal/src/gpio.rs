//! General Purpose Input / Output
//!
//! This module makes heavy use of types to try and ensure you can't have a
//! pin in a mode you didn't expect.
//!
//! Most pins start in the `Tristate` state. You can call methods to convert
//! them to inputs, outputs or put them into Alternate Function mode (e.g. to
//! use with a UART).
//!
//! Some of the modes require extra information, and for that we use the so-
//! called 'Turbo Fish` syntax, which looks like `method::<TYPE>`.
//!
//! If the operation is non-atomic, then you need to pass a mut-reference to
//! the port's control structure. This ensures you can't change two pins in
//! two threads at the same time. If the operation is fully atomic (using the
//! chip's bit-banding feature) then this argument is not required.
//!
//! Here's an example:
//!
//! ```
//! # use tm4c129x_hal::*;
//! # use tm4c129x_hal::sysctl::SysctlExt;
//! # use tm4c129x_hal::gpio::GpioExt;
//! # fn foo() {
//! let p = Peripherals::take().unwrap();
//! let mut sc = p.SYSCTL.constrain();
//! let mut portb = p.GPIO_PORTB.split(&sc.power_control);
//! let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
//! let uart_tx_pin = portb
//!     .pb1
//!     .into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
//! let blue_led = portb.pb2.into_push_pull_output();
//! let button = portb.pb3.into_pull_up_input();
//! # }
//! ```

use crate::bb;
use crate::hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use crate::sysctl;
use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, power_control: &sysctl::PowerControl) -> Self::Parts;
}

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

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopd:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use super::*;
            use tm4c129x::$GPIOX;

            /// Provides mutual-exclusion for certain GPIO operations (such as
            /// selecting an alternate mode) that can't be done atomically.
            pub struct GpioControl {
                _0: (),
            }

            /// GPIO parts
            pub struct Parts {
                /// Pass an &mut reference to methods that require it.
                pub control: GpioControl,
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                /// Break this GPIO port into separate pins
                fn split(self, pc: &sysctl::PowerControl) -> Parts {
                    sysctl::control_power(
                        pc, sysctl::Domain::$iopd,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$iopd);

                    Parts {
                        control: GpioControl { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> StatefulOutputPin for $PXx<Output<MODE>> where MODE: OutputMode {
                fn is_set_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_set_low(&self) -> bool {
                    !self.is_set_high()
                }
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> where MODE: OutputMode {
                fn set_high(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, true); }
                }

                fn set_low(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, false); }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> where MODE: InputMode {
                fn is_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_low(&self) -> bool {
                    !self.is_high()
                }
            }

            impl<MODE> $PXx<Input<MODE>> where MODE: InputMode {
                /// Enables or disables interrupts on this GPIO pin.
                pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.im, self.i, false); }
                    match mode {
                        InterruptMode::LevelHigh => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS |= self.i;
                            unsafe { bb::change_bit(&p.is, self.i, true); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::LevelLow => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS |= self.i;
                            unsafe { bb::change_bit(&p.is, self.i, true); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV &= ~self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, false); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeRising => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeFalling => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE &= ~self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, false); }
                            // IEV &= ~self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, false); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::EdgeBoth => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                            // IS &= ~self.i;
                            unsafe { bb::change_bit(&p.is, self.i, false); }
                            // IBE |= self.i;
                            unsafe { bb::change_bit(&p.ibe, self.i, true); }
                            // IEV |= self.i;
                            unsafe { bb::change_bit(&p.iev, self.i, true); }
                            // IM |= self.i;
                            unsafe { bb::change_bit(&p.im, self.i, true); }
                        },
                        InterruptMode::Disabled => {
                            // IM &= ~self.i;
                            unsafe { bb::change_bit(&p.im, self.i, false); }
                        },
                    }
                }

                /// Marks the interrupt for this pin as handled. You should
                /// call this (or perform its functionality) from the ISR.
                pub fn clear_interrupt(&self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.icr, self.i, true); }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> where MODE: IsUnlocked {
                    /// Configures the pin to serve as alternate function 1 through 15.
                    /// Disables open-drain to make the output a push-pull.
                    pub fn into_af_push_pull<AF>(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AlternateFunction<AF, PushPull>> where AF: AlternateFunctionChoice {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = AF::number() << ($i * 4);
                        unsafe {
                            p.pctl.modify(|r, w| w.bits((r.bits() & !mask) | bits));
                        }
                        unsafe { bb::change_bit(&p.afsel, $i, true); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as alternate function 1 through 15.
                    /// Enables open-drain (useful for I2C SDA, for example).
                    pub fn into_af_open_drain<AF, ODM>(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AlternateFunction<AF, OpenDrain<ODM>>> where AF: AlternateFunctionChoice, ODM: OpenDrainMode {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = AF::number() << ($i * 4);
                        unsafe {
                            p.pctl.modify(|r, w| w.bits((r.bits() & !mask) | bits));
                        }
                        unsafe { bb::change_bit(&p.afsel, $i, true); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        unsafe { bb::change_bit(&p.pur, $i, ODM::pup()); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self
                    ) -> $PXi<Input<Floating>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self
                    ) -> $PXi<Input<PullDown>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, true); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self
                    ) -> $PXi<Input<PullUp>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, true); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output<ODM>(
                        self
                    ) -> $PXi<Output<OpenDrain<ODM>>> where ODM: OpenDrainMode {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        unsafe { bb::change_bit(&p.pur, $i, ODM::pup()); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self
                    ) -> $PXi<Output<PushPull>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin as tri-state
                    pub fn into_tri_state(
                        self
                    ) -> $PXi<Tristate> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.den, $i, false); }
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        $PXi { _mode: PhantomData }
                    }

                }

                impl<MODE> $PXi<MODE> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<MODE> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> where MODE: OutputMode {
                    fn is_set_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_set_low(&self) -> bool {
                        !self.is_set_high()
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> where MODE: OutputMode {
                    fn set_high(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, true); }
                    }

                    fn set_low(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, false); }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> where MODE: InputMode {
                    fn is_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }
                }

                impl<MODE> $PXi<Input<MODE>> where MODE: InputMode {
                    /// Enables or disables interrupts on this GPIO pin.
                    pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.im, $i, false); }
                        match mode {
                            InterruptMode::LevelHigh => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS |= $i;
                                unsafe { bb::change_bit(&p.is, $i, true); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::LevelLow => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS |= $i;
                                unsafe { bb::change_bit(&p.is, $i, true); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV &= ~$i;
                                unsafe { bb::change_bit(&p.iev, $i, false); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeRising => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeFalling => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE &= ~$i;
                                unsafe { bb::change_bit(&p.ibe, $i, false); }
                                // IEV &= ~$i;
                                unsafe { bb::change_bit(&p.iev, $i, false); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::EdgeBoth => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                                // IS &= ~$i;
                                unsafe { bb::change_bit(&p.is, $i, false); }
                                // IBE |= $i;
                                unsafe { bb::change_bit(&p.ibe, $i, true); }
                                // IEV |= $i;
                                unsafe { bb::change_bit(&p.iev, $i, true); }
                                // IM |= $i;
                                unsafe { bb::change_bit(&p.im, $i, true); }
                            },
                            InterruptMode::Disabled => {
                                // IM &= ~$i;
                                unsafe { bb::change_bit(&p.im, $i, false); }
                            },
                        }
                    }

                    /// Marks the interrupt for this pin as handled. You should
                    /// call this (or perform its functionality) from the ISR.
                    pub fn clear_interrupt(&self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.icr, $i, true); }
                    }
                }

                impl $PXi<Locked> {
                    /// Unlock a GPIO so that it can be used. This is required
                    /// on 'special' GPIOs that the manufacturer doesn't want
                    /// you to change by accident - like NMI and JTAG pins.
                    pub fn unlock(self, _gpio_control: &mut GpioControl) -> $PXi<Tristate> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        p.lock.write(|w| w.lock().key());
                        p.cr.modify(|_, w| unsafe { w.bits(1 << $i) });
                        p.lock.write(|w| w.lock().unlocked());
                        unsafe { bb::change_bit(&p.den, $i, false); }
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        $PXi { _mode: PhantomData }
                    }
                }
            )+
        }
    }
}

gpio!(GPIO_PORTA_AHB, gpioa, GpioA, PAx, [
    PA0: (pa0, 0, Tristate),
    PA1: (pa1, 1, Tristate),
    PA2: (pa2, 2, Tristate),
    PA3: (pa3, 3, Tristate),
    PA4: (pa4, 4, Tristate),
    PA5: (pa5, 5, Tristate),
    PA6: (pa6, 6, Tristate),
    PA7: (pa7, 7, Tristate),
]);

gpio!(GPIO_PORTB_AHB, gpiob, GpioB, PBx, [
    PB0: (pb0, 0, Tristate),
    PB1: (pb1, 1, Tristate),
    PB2: (pb2, 2, Tristate),
    PB3: (pb3, 3, Tristate),
    PB4: (pb4, 4, Tristate),
    PB5: (pb5, 5, Tristate),
    // PB6: (pb6, 6, Tristate),
    // PB7: (pb7, 7, Tristate),
]);

gpio!(GPIO_PORTC_AHB, gpioc, GpioC, PCx, [
    PC0: (pc0, 0, Locked), // JTAG/SWD pin
    PC1: (pc1, 1, Locked), // JTAG/SWD pin
    PC2: (pc2, 2, Locked), // JTAG/SWD pin
    PC3: (pc3, 3, Locked), // JTAG/SWD pin
    PC4: (pc4, 4, Tristate),
    PC5: (pc5, 5, Tristate),
    PC6: (pc6, 6, Tristate),
    PC7: (pc7, 7, Tristate),
]);

gpio!(GPIO_PORTD_AHB, gpiod, GpioD, PDx, [
    PD0: (pd0, 0, Tristate),
    PD1: (pd1, 1, Tristate),
    PD2: (pd2, 2, Tristate),
    PD3: (pd3, 3, Tristate),
    PD4: (pd4, 4, Tristate),
    PD5: (pd5, 5, Tristate),
    PD6: (pd6, 6, Tristate),
    PD7: (pd7, 7, Locked), // GPIO pin
]);

gpio!(GPIO_PORTE_AHB, gpioe, GpioE, PEx, [
    PE0: (pe0, 0, Tristate),
    PE1: (pe1, 1, Tristate),
    PE2: (pe2, 2, Tristate),
    PE3: (pe3, 3, Tristate),
    PE4: (pe4, 4, Tristate),
    PE5: (pe5, 5, Tristate),
    // PE6: (pe6, 6, Tristate),
    // PE7: (pe7, 7, Locked), // GPIO pin
]);

gpio!(GPIO_PORTF_AHB, gpiof, GpioF, PFx, [
    PF0: (pf0, 0, Tristate),
    PF1: (pf1, 1, Tristate),
    PF2: (pf2, 2, Tristate),
    PF3: (pf3, 3, Tristate),
    PF4: (pf4, 4, Tristate),
    // PF5: (pf5, 5, Tristate),
    // PF6: (pf6, 6, Tristate),
    // PF7: (pf7, 7, Tristate),
]);

gpio!(GPIO_PORTG_AHB, gpiog, GpioG, PGx, [
    PG0: (pg0, 0, Tristate),
    PG1: (pg1, 1, Tristate),
    // PG2: (pg2, 2, Tristate),
    // PG3: (pg3, 3, Tristate),
    // PG4: (pg4, 4, Tristate),
    // PG5: (pg5, 5, Tristate),
    // PG6: (pg6, 6, Tristate),
    // PG7: (pg7, 7, Tristate),
]);

gpio!(GPIO_PORTH_AHB, gpioh, GpioH, PHx, [
    PH0: (ph0, 0, Tristate),
    PH1: (ph1, 1, Tristate),
    PH2: (ph2, 2, Tristate),
    PH3: (ph3, 3, Tristate),
    // PH4: (ph4, 4, Tristate),
    // PH5: (ph5, 5, Tristate),
    // PH6: (ph6, 6, Tristate),
    // PH7: (ph7, 7, Tristate),
]);

gpio!(GPIO_PORTJ_AHB, gpioj, GpioJ, PJx, [
    PJ0: (pj0, 0, Tristate),
    PJ1: (pj1, 1, Tristate),
    // PJ2: (pj2, 2, Tristate),
    // PJ3: (pj3, 3, Tristate),
    // PJ4: (pj4, 4, Tristate),
    // PJ5: (pj5, 5, Tristate),
    // PJ6: (pj6, 6, Tristate),
    // PJ7: (pj7, 7, Tristate),
]);

gpio!(GPIO_PORTK, gpiok, GpioK, PKx, [
    PK0: (pk0, 0, Tristate),
    PK1: (pk1, 1, Tristate),
    PK2: (pk2, 2, Tristate),
    PK3: (pk3, 3, Tristate),
    PK4: (pk4, 4, Tristate),
    PK5: (pk5, 5, Tristate),
    PK6: (pk6, 6, Tristate),
    PK7: (pk7, 7, Tristate),
]);

gpio!(GPIO_PORTL, gpiol, GpioL, PNL, [
    PL0: (pl0, 0, Tristate),
    PL1: (pl1, 1, Tristate),
    PL2: (pl2, 2, Tristate),
    PL3: (pl3, 3, Tristate),
    PL4: (pl4, 4, Tristate),
    PL5: (pl5, 5, Tristate),
    PL6: (pl6, 6, Tristate),
    PL7: (pl7, 7, Tristate),
]);

gpio!(GPIO_PORTM, gpiom, GpioM, PMx, [
    PM0: (pm0, 0, Tristate),
    PM1: (pm1, 1, Tristate),
    PM2: (pm2, 2, Tristate),
    PM3: (pm3, 3, Tristate),
    PM4: (pm4, 4, Tristate),
    PM5: (pm5, 5, Tristate),
    PM6: (pm6, 6, Tristate),
    PM7: (pm7, 7, Tristate),
]);

gpio!(GPIO_PORTN, gpion, GpioN, PNx, [
    PN0: (pn0, 0, Tristate),
    PN1: (pn1, 1, Tristate),
    PN2: (pn2, 2, Tristate),
    PN3: (pn3, 3, Tristate),
    PN4: (pn4, 4, Tristate),
    PN5: (pn5, 5, Tristate),
    PN6: (pn6, 6, Tristate),
    PN7: (pn7, 7, Tristate),
]);

gpio!(GPIO_PORTP, gpiop, GpioP, PPx, [
    PP0: (pp0, 0, Tristate),
    PP1: (pp1, 1, Tristate),
    PP2: (pp2, 2, Tristate),
    PP3: (pp3, 3, Tristate),
    PP4: (pp4, 4, Tristate),
    PP5: (pp5, 5, Tristate),
    // PP6: (pp6, 6, Tristate),
    // PP7: (pp7, 7, Tristate),
]);

gpio!(GPIO_PORTQ, gpioq, GpioQ, PQx, [
    PQ0: (pq0, 0, Tristate),
    PQ1: (pq1, 1, Tristate),
    PQ2: (pq2, 2, Tristate),
    PQ3: (pq3, 3, Tristate),
    PQ4: (pq4, 4, Tristate),
    // PQ5: (pq5, 5, Tristate),
    // PQ6: (pq6, 6, Tristate),
    // PQ7: (pq7, 7, Tristate),
]);
