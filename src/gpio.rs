//! General Purpose Input / Output

// TODO the pins here currently correspond to the LQFP-100 package. There should be Cargo features
// that let you select different microcontroller packages

use core::marker::PhantomData;
use hal::digital::{InputPin, OutputPin};
use sysctl;
use bb;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, power_control: &sysctl::PowerControl) -> Self::Parts;
}

/// All unlocked pin modes implement this
pub trait IsUnlocked {}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}
impl<MODE> IsUnlocked for Input<MODE> {}

/// Sub-mode of Input: Floating input (type state)
pub struct Floating;

/// Sub-mode of Input: Pulled down input (type state)
pub struct PullDown;

/// Sub-mode of Input: Pulled up input (type state)
pub struct PullUp;

/// Tri-state
pub struct Tristate;
impl IsUnlocked for Tristate {}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
impl<MODE> IsUnlocked for Output<MODE> {}

/// Sub-mode of Output: Push pull output (type state)
pub struct PushPull;

/// Sub-mode of Output: Open drain output (type state)
pub struct OpenDrain;

/// Alternate function 1 (type state)
pub struct AF1;
impl IsUnlocked for AF1 {}

/// Alternate function 2 (type state)
pub struct AF2;
impl IsUnlocked for AF2 {}

/// Alternate function 3 (type state)
pub struct AF3;
impl IsUnlocked for AF3 {}

/// Alternate function 4 (type state)
pub struct AF4;
impl IsUnlocked for AF4 {}

/// Alternate function 5 (type state)
pub struct AF5;
impl IsUnlocked for AF5 {}

/// Alternate function 6 (type state)
pub struct AF6;
impl IsUnlocked for AF6 {}

/// Alternate function 7 (type state)
pub struct AF7;
impl IsUnlocked for AF7 {}

/// Alternate function 8 (type state)
pub struct AF8;
impl IsUnlocked for AF8 {}

/// Alternate function 9 (type state)
pub struct AF9;
impl IsUnlocked for AF9 {}

// 10 through 13 are not available on this chip.

/// Alternate function 14 (type state)
pub struct AF14;
impl IsUnlocked for AF14 {}

/// Alternate function 15 (type state)
pub struct AF15;
impl IsUnlocked for AF15 {}

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
    Disabled
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopd:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use super::*;
            use tm4c123x::$GPIOX;

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

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn is_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_low(&self) -> bool {
                    !self.is_high()
                }

                fn set_high(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, true); }
                }

                fn set_low(&mut self) {
                    let p = unsafe { &*$GPIOX::ptr() };
                    unsafe { bb::change_bit(&p.data, self.i, false); }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&self) -> bool {
                    let p = unsafe { &*$GPIOX::ptr() };
                    bb::read_bit(&p.data, self.i)
                }

                fn is_low(&self) -> bool {
                    !self.is_high()
                }
            }

            impl<MODE> $PXx<Input<MODE>> {
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
                    /// Configures the pin to serve as alternate function 1 (AF1)
                    pub fn into_af1(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF1> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x1 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 2 (AF2)
                    pub fn into_af2(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF2> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x2 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 3 (AF3)
                    pub fn into_af3(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF3> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x3 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 4 (AF4)
                    pub fn into_af4(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF4> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x4 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 5 (AF5)
                    pub fn into_af5(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF5> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x5 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 6 (AF6)
                    pub fn into_af6(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF6> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x6 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 7 (AF7)
                    pub fn into_af7(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF7> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x7 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 8 (AF8)
                    pub fn into_af8(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF8> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x8 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 9 (AF9)
                    pub fn into_af9(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF9> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0x9 << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 14 (AF14)
                    pub fn into_af14(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF14> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let mask = 0xF << ($i * 4);
                        let bits = 0xE << ($i * 4);
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

                    /// Configures the pin to serve as alternate function 15 (AF15)
                    pub fn into_af15(
                        self,
                        _gpio_control: &mut GpioControl,
                    ) -> $PXi<AF15> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        let bits = 0xF << ($i * 4);
                        unsafe {
                            p.pctl.modify(|r, w| w.bits(r.bits() | bits));
                        }
                        unsafe { bb::change_bit(&p.afsel, $i, true); }
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.odr, $i, false); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
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
                    pub fn into_open_drain_output(
                        self
                    ) -> $PXi<Output<OpenDrain>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.afsel, $i, false); }
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        unsafe { bb::change_bit(&p.pur, $i, false); }
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

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, on: bool) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // pull up
                        unsafe { bb::change_bit(&p.pur, $i, on); }
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

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }

                    fn set_high(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, true); }
                    }

                    fn set_low(&mut self) {
                        let p = unsafe { &*$GPIOX::ptr() };
                        unsafe { bb::change_bit(&p.data, $i, false); }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        let p = unsafe { &*$GPIOX::ptr() };
                        bb::read_bit(&p.data, $i)
                    }

                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
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

gpio!(GPIO_PORTA, gpioa, GpioA, PAx, [
    PA0: (pa0, 0, Tristate),
    PA1: (pa1, 1, Tristate),
    PA2: (pa2, 2, Tristate),
    PA3: (pa3, 3, Tristate),
    PA4: (pa4, 4, Tristate),
    PA5: (pa5, 5, Tristate),
    PA6: (pa6, 6, Tristate),
    PA7: (pa7, 7, Tristate),
]);

gpio!(GPIO_PORTB, gpiob, GpioB, PBx, [
    PB0: (pb0, 0, Tristate),
    PB1: (pb1, 1, Tristate),
    PB2: (pb2, 2, Tristate),
    PB3: (pb3, 3, Tristate),
    PB4: (pb4, 4, Tristate),
    PB5: (pb5, 5, Tristate),
    PB6: (pb6, 6, Tristate),
    PB7: (pb7, 7, Tristate),
]);

gpio!(GPIO_PORTC, gpioc, GpioC, PCx, [
    PC0: (pc0, 0, Locked), // JTAG/SWD pin
    PC1: (pc1, 1, Locked), // JTAG/SWD pin
    PC2: (pc2, 2, Locked), // JTAG/SWD pin
    PC3: (pc3, 3, Locked), // JTAG/SWD pin
    PC4: (pc4, 4, Tristate),
    PC5: (pc5, 5, Tristate),
    PC6: (pc6, 6, Tristate),
    PC7: (pc7, 7, Tristate),
]);

gpio!(GPIO_PORTD, gpiod, GpioD, PDx, [
    PD0: (pd0, 0, Tristate),
    PD1: (pd1, 1, Tristate),
    PD2: (pd2, 2, Tristate),
    PD3: (pd3, 3, Tristate),
    PD4: (pd4, 4, Tristate),
    PD5: (pd5, 5, Tristate),
    PD6: (pd6, 6, Tristate),
    PD7: (pd7, 7, Locked), // NMI pin
]);

gpio!(GPIO_PORTE, gpioe, GpioE, PEx, [
    PE0: (pe0, 0, Tristate),
    PE1: (pe1, 1, Tristate),
    PE2: (pe2, 2, Tristate),
    PE3: (pe3, 3, Tristate),
    PE4: (pe4, 4, Tristate),
    PE5: (pe5, 5, Tristate),
    PE6: (pe6, 6, Tristate),
    PE7: (pe7, 7, Tristate),
]);

gpio!(GPIO_PORTF, gpiof, GpioF, PFx, [
    PF0: (pf0, 0, Locked), // NMI pin
    PF1: (pf1, 1, Tristate),
    PF2: (pf2, 2, Tristate),
    PF3: (pf3, 3, Tristate),
    PF4: (pf4, 4, Tristate),
    PF5: (pf5, 5, Tristate),
    PF6: (pf6, 6, Tristate),
    PF7: (pf7, 7, Tristate),
]);
