//! General Purpose Input / Output

// TODO the pins here currently correspond to the LQFP-100 package. There should be Cargo features
// that let you select different microcontroller packages

use core::marker::PhantomData;

use sysctl;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, power_control: &sysctl::PowerControl) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function 0 (type state)
pub struct AF0;

/// Alternate function 1 (type state)
pub struct AF1;

/// Alternate function 2 (type state)
pub struct AF2;

/// Alternate function 3 (type state)
pub struct AF3;

/// Alternate function 4 (type state)
pub struct AF4;

/// Alternate function 5 (type state)
pub struct AF5;

/// Alternate function 6 (type state)
pub struct AF6;

/// Alternate function 7 (type state)
pub struct AF7;

/// Alternate function 8 (type state)
pub struct AF8;

/// Alternate function 9 (type state)
pub struct AF9;

// 10 through 13 are not available on this chip.

/// Alternate function 14 (type state)
pub struct AF14;

/// Alternate function 15 (type state)
pub struct AF15;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopd:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;
            use tm4c123x::$GPIOX;
            use sysctl;
            use bb;

            use super::{
                Floating, GpioExt, Input, Output, PullUp, PullDown, OpenDrain, PushPull
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, pc: &sysctl::PowerControl) -> Parts {
                    sysctl::control_power(
                        pc, sysctl::PeripheralPowerDomain::$iopd,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::PeripheralPowerDomain::$iopd);

                    Parts {
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

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                //     /// Configures the pin to serve as alternate function 4 (AF4)
                //     pub fn into_af4(
                //         self,
                //         dir: &mut DIR,
                //         afsel: &mut AFSEL,
                //     ) -> $PXi<AF4> {
                //         let offset = 2 * $i;

                //         // alternate function mode
                //         let mode = 0b10;
                //         dir.dir().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                //         });

                //         let af = 4;
                //         let offset = 4 * ($i % 8);
                //         afsel.afsel().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                //         });

                //         $PXi { _mode: PhantomData }
                //     }

                //     /// Configures the pin to serve as alternate function 5 (AF5)
                //     pub fn into_af5(
                //         self,
                //         dir: &mut DIR,
                //         afsel: &mut AFSEL,
                //     ) -> $PXi<AF5> {
                //         let offset = 2 * $i;

                //         // alternate function mode
                //         let mode = 0b10;
                //         dir.dir().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                //         });

                //         let af = 5;
                //         let offset = 4 * ($i % 8);
                //         afsel.afsel().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                //         });

                //         $PXi { _mode: PhantomData }
                //     }

                //     /// Configures the pin to serve as alternate function 6 (AF6)
                //     pub fn into_af6(
                //         self,
                //         dir: &mut DIR,
                //         afsel: &mut AFSEL,
                //     ) -> $PXi<AF6> {
                //         let offset = 2 * $i;

                //         // alternate function mode
                //         let mode = 0b10;
                //         dir.dir().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                //         });

                //         let af = 6;
                //         let offset = 4 * ($i % 8);
                //         afsel.afsel().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                //         });

                //         $PXi { _mode: PhantomData }
                //     }

                //     /// Configures the pin to serve as alternate function 7 (AF7)
                //     pub fn into_af7(
                //         self,
                //         dir: &mut DIR,
                //         afsel: &mut AFSEL,
                //     ) -> $PXi<AF7> {
                //         let offset = 2 * $i;

                //         // alternate function mode
                //         let mode = 0b10;
                //         dir.dir().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                //         });

                //         let af = 7;
                //         let offset = 4 * ($i % 8);

                //         afsel.afsel().modify(|r, w| unsafe {
                //             w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                //         });

                //         $PXi { _mode: PhantomData }
                //     }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self
                    ) -> $PXi<Input<Floating>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // input mode
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        // no pull
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self
                    ) -> $PXi<Input<PullDown>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // input mode
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        // pull down
                        unsafe { bb::change_bit(&p.pur, $i, false); }
                        unsafe { bb::change_bit(&p.pdr, $i, true); }

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self
                    ) -> $PXi<Input<PullUp>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // input mode
                        unsafe { bb::change_bit(&p.dir, $i, false); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        // pull up
                        unsafe { bb::change_bit(&p.pur, $i, true); }
                        unsafe { bb::change_bit(&p.pdr, $i, false); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self
                    ) -> $PXi<Output<OpenDrain>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // output mode
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        // open-drain
                        unsafe { bb::change_bit(&p.odr, $i, true); }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self
                    ) -> $PXi<Output<PushPull>> {
                        let p = unsafe { &*$GPIOX::ptr() };
                        // output mode
                        unsafe { bb::change_bit(&p.dir, $i, true); }
                        unsafe { bb::change_bit(&p.den, $i, true); }
                        // open-drain off
                        unsafe { bb::change_bit(&p.odr, $i, false); }
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

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
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
            )+
        }
    }
}

gpio!(GPIO_PORTA, gpio_porta, GpioA, PAx, [
    PA0: (pa0, 0, Input<Floating>),
    PA1: (pa1, 1, Input<Floating>),
    PA2: (pa2, 2, Input<Floating>),
    PA3: (pa3, 3, Input<Floating>),
    PA4: (pa4, 4, Input<Floating>),
    PA5: (pa5, 5, Input<Floating>),
    PA6: (pa6, 6, Input<Floating>),
    PA7: (pa7, 7, Input<Floating>),
]);

gpio!(GPIO_PORTB, gpio_portb, GpioB, PBx, [
    PB0: (pb0, 0, Input<Floating>),
    PB1: (pb1, 1, Input<Floating>),
    PB2: (pb2, 2, Input<Floating>),
    PB3: (pb3, 3, Input<Floating>),
    PB4: (pb4, 4, Input<Floating>),
    PB5: (pb5, 5, Input<Floating>),
    PB6: (pb6, 6, Input<Floating>),
    PB7: (pb7, 7, Input<Floating>),
]);

gpio!(GPIO_PORTF, gpio_portf, GpioF, PFx, [
    PF0: (pf0, 0, Input<Floating>),
    PF1: (pf1, 1, Input<Floating>),
    PF2: (pf2, 2, Input<Floating>),
    PF3: (pf3, 3, Input<Floating>),
    PF4: (pf4, 4, Input<Floating>),
    PF5: (pf5, 5, Input<Floating>),
    PF6: (pf6, 6, Input<Floating>),
    PF7: (pf7, 7, Input<Floating>),
]);
