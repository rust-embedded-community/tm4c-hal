//! Timers
//!
//! Timers on the LM4F120 come in two variants: normal (16-bit/32-bit) and wide (32-bit/64-bit).
//!
//! Each timer is comprised of two parts, A and B. They may be combined into
//! one single, wider, timer if desired.
//!
//! Timers have a number of modes:
//!
//! One-shot mode.
//! Periodic mode.
//! Real-time Clock mode.
//! Input Edge-count mode.
//! Input Edge-time mode.
//! Pulse-width Modulation (PWM) mode.

use sysctl;
use core::marker::PhantomData;
use tm4c123x::{self, TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5};

/// A 16-bit/32-bit Timer
#[must_use]
pub struct Timer<TIM, ModeA, ModeB> {
    tim: TIM,
    _mode_a: PhantomData<ModeA>,
    _mode_b: PhantomData<ModeB>,
}

/// Marker trait for Mode B states.
pub unsafe trait NotDisabled {}

/// Marker type for Timers in the reset state.
pub struct Reset;

// pub struct Periodic;
// pub struct OneShot;
// pub struct RealTimeClock;
// pub struct InputEdgeCount;
// pub struct InputEdgeTime;
/// Marker type for Timers in PWM mode.
pub struct PWM;

unsafe impl NotDisabled for PWM {}

/// Marker type for the B half when it's disable and A is double-width.
pub struct Disabled;

pub trait TimerExt {
    /// Makes a new timer
    fn create(self, pc: &sysctl::PowerControl) -> Timer<Self, Reset, Reset> where Self: Sized;
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $iopd:ident, $TIMA:ident, $TIMB:ident),)+) => {
        $(
            impl TimerExt for $TIM {
                /// Enables and resets a timer, but does little else.
                fn create(self, pc: &sysctl::PowerControl) -> Timer<Self, Reset, Reset> {
                    sysctl::control_power(
                        pc, sysctl::Domain::$iopd,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$iopd);

                    let timer = Timer {
                        tim: self,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    };
                    timer
                }
            }

            impl<ModeA, ModeB> Timer<$TIM, ModeA, ModeB> {

                /// Releases the TIM peripheral
                pub fn free(self) -> $TIM {
                    // return underlying object
                    self.tim
                }
            }

            impl Timer<$TIM, Reset, Reset> {
                /// Enter PWM mode.
                ///
                /// The total period of the PWM output is given in `total_period_ticks`. It is measured in clock ticks.
                /// The on period of the PWM output is given in `on_period_ticks`. It is measured in clock ticks.
                ///
                /// If `total_period_ticks` was, say, 1000 and `on_period_ticks` was, say, 333, and the clock
                /// frequency was 1 MHz, then the output would cycle at 1 kHz, one third on, two thirds off.
                pub fn make_wide(self) -> Timer<$TIM, Reset, Disabled> {
                    Timer {
                        tim: self.tim,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    }
                }
            }

            impl Timer<$TIM, Reset, Disabled> {
                /// Enter PWM mode in double-wide mode.
                ///
                /// The total period of the PWM output is given in `total_period_ticks`. It is measured in clock ticks.
                /// The on period of the PWM output is given in `on_period_ticks`. It is measured in clock ticks.
                ///
                /// If `total_period_ticks` was, say, 1000 and `on_period_ticks` was, say, 333, and the clock
                /// frequency was 1 MHz, then the output would cycle at 1 kHz, one third on, two thirds off.
                pub fn enter_pwm_mode_wide(
                        self,
                        _total_period_ticks: u32,
                        _on_period_ticks: u32,
                        _invert_signal: bool) -> Timer<$TIM, PWM, Disabled> {
                    Timer {
                        tim: self.tim,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    }
                }
            }

            impl<ModeA, ModeB> Timer<$TIM, ModeA, ModeB> where ModeA: NotDisabled {
                /// Stop Timer A
                pub fn stop_a(&mut self) {
                    self.tim.ctl.modify(|_, w| {
                        w.taen().clear_bit();
                        w
                    });
                }

                /// Start Timer A
                pub fn start_a(&mut self) {
                    self.tim.ctl.modify(|_, w| {
                        w.taen().set_bit();
                        w
                    });
                }
            }

            impl<ModeA, ModeB> Timer<$TIM, ModeA, ModeB> where ModeB: NotDisabled {
                /// Stop Timer B
                pub fn stop_b(&mut self) {
                    self.tim.ctl.modify(|_, w| {
                        w.tben().clear_bit();
                        w
                    });
                }

                /// Start Timer B
                pub fn start_b(&mut self) {
                    self.tim.ctl.modify(|_, w| {
                        w.tben().set_bit();
                        w
                    });
                }
            }


            impl<ModeB> Timer<$TIM, Reset, ModeB> where ModeB: NotDisabled {
                /// Enter PWM mode on the A half.
                ///
                /// The total period of the PWM output is given in `total_period_ticks`. It is measured in clock ticks.
                /// The on period of the PWM output is given in `on_period_ticks`. It is measured in clock ticks.
                ///
                /// If `total_period_ticks` was, say, 1000 and `on_period_ticks` was, say, 333, and the clock
                /// frequency was 1 MHz, then the output would cycle at 1 kHz, one third on, two thirds off.
                ///
                /// If you want to see a GPIO output, you need to put a pin
                /// into the appropriate alternate function state. We don't
                /// take a pin as an argument as you might not want a physical
                /// output, just a repeating timer.
                pub fn enter_pwm_mode_a(
                        self,
                        total_period_ticks: u16,
                        on_period_ticks: u16,
                        invert_signal: bool) -> Timer<$TIM, PWM, ModeB> {

                    let mut t = Timer {
                        tim: self.tim,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    };

                    t.stop_a();

                    // PWM = Alternate Mode Select (AMS), Capture Mode off and Mode = Periodic
                    t.tim.tamr.modify(|_, w| {
                        w.taams().set_bit();
                        w.tacmr().clear_bit();
                        w.tamr().period();
                        w
                    });

                    // Timer A interval load register
                    t.tim.tailr.write(|w| unsafe { w.bits(total_period_ticks as u32 - 1) });

                    // Timer A match register
                    t.tim.tamatchr.write(|w| unsafe { w.bits(on_period_ticks as u32 - 1) });

                    // GPTM Timer A PWM Output Level: 1 = inverted
                    if invert_signal {
                        t.tim.ctl.modify(|_, w| w.tapwml().set_bit());
                    }

                    t.start_a();

                    t
                }
            }

            impl Timer<$TIM, Reset, Disabled> {
                /// Enter PWM mode on the A half.
                ///
                /// The total period of the PWM output is given in `total_period_ticks`. It is measured in clock ticks.
                /// The on period of the PWM output is given in `on_period_ticks`. It is measured in clock ticks.
                ///
                /// If `total_period_ticks` was, say, 1000 and `on_period_ticks` was, say, 333, and the clock
                /// frequency was 1 MHz, then the output would cycle at 1 kHz, one third on, two thirds off.
                ///
                /// If you want to see a GPIO output, you need to put a pin
                /// into the appropriate alternate function state. We don't
                /// take a pin as an argument as you might not want a physical
                /// output, just a repeating timer.
                pub fn enter_pwm_mode_a(
                        self,
                        total_period_ticks: u32,
                        on_period_ticks: u32,
                        invert_signal: bool) -> Timer<$TIM, PWM, Disabled> {

                    let mut t = Timer {
                        tim: self.tim,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    };

                    t.stop_a();

                    // PWM = Alternate Mode Select (AMS), Capture Mode off and Mode = Periodic
                    t.tim.tamr.modify(|_, w| {
                        w.taams().set_bit();
                        w.tacmr().clear_bit();
                        w.tamr().period();
                        w
                    });

                    // Timer A interval load register
                    t.tim.tailr.write(|w| unsafe { w.bits(total_period_ticks - 1) });

                    // Timer A match register
                    t.tim.tamatchr.write(|w| unsafe { w.bits(on_period_ticks - 1) });

                    // GPTM Timer A PWM Output Level: 1 = inverted
                    if invert_signal {
                        t.tim.ctl.modify(|_, w| w.tapwml().set_bit());
                    }

                    t.start_a();

                    t
                }
            }

            impl<ModeB> Timer<$TIM, PWM, ModeB> {
                /// Enable interrupt on Timer A.
                /// If the PWM level is inverted, trigger on low to high, else
                /// trigger on high to low (i.e. after `on_period_ticks`).
                pub fn enable_interrupt_a(&mut self) {
                    self.tim.tamr.modify(|_, w| {
                        w.tapwmie().set_bit()
                    });
                    self.tim.imr.modify(|_, w| {
                        w.caeim().set_bit()
                    });
                }
            }

            impl<ModeA> Timer<$TIM, ModeA, PWM> {
                /// Enable interrupt on Timer B.
                /// If the PWM level is inverted, trigger on low to high, else
                /// trigger on high to low (i.e. after `on_period_ticks`).
                pub fn enable_interrupt_b(&mut self) {
                    self.tim.tbmr.modify(|_, w| {
                        w.tbpwmie().set_bit()
                    });
                    self.tim.imr.modify(|_, w| {
                        w.cbeim().set_bit()
                    });
                }
            }

            impl<ModeA> Timer<$TIM, ModeA, Reset> {
                /// Enter PWM mode on the B half.
                ///
                /// The total period of the PWM output is given in `total_period_ticks`. It is measured in clock ticks.
                /// The on period of the PWM output is given in `on_period_ticks`. It is measured in clock ticks.
                ///
                /// If `total_period_ticks` was, say, 1000 and `on_period_ticks` was, say, 333, and the clock
                /// frequency was 1 MHz, then the output would cycle at 1 kHz, one third on, two thirds off.
                pub fn enter_pwm_mode_b(
                        self,
                        total_period_ticks: u16,
                        on_period_ticks: u16,
                        invert_signal: bool) -> Timer<$TIM, ModeA, PWM> {
                    let mut t = Timer {
                        tim: self.tim,
                        _mode_a: PhantomData,
                        _mode_b: PhantomData,
                    };

                    t.stop_b();

                    // PWM = Alternate Mode Select (AMS), Capture Mode off and Mode = Periodic
                    t.tim.tbmr.modify(|_, w| {
                        w.tbams().set_bit();
                        w.tbcmr().clear_bit();
                        w.tbmr().period();
                        w
                    });

                    // Timer B interval load register
                    t.tim.tbilr.write(|w| unsafe { w.bits(total_period_ticks as u32) });

                    // Timer B match register
                    t.tim.tbmatchr.write(|w| unsafe { w.bits(on_period_ticks as u32 - 1) });

                    // GPTM Timer B PWM Output Level: 1 = inverted
                    if invert_signal {
                        t.tim.ctl.modify(|_, w| w.tbpwml().set_bit());
                    }

                    t.start_b();

                    t
                }
            }

        )+
    }
}

hal! {
    TIMER0: (timer0, Timer0, TIMER0A, TIMER0B),
    TIMER1: (timer1, Timer1, TIMER1A, TIMER1B),
    TIMER2: (timer2, Timer2, TIMER2A, TIMER2B),
    TIMER3: (timer3, Timer3, TIMER3A, TIMER3B),
    TIMER4: (timer4, Timer4, TIMER4A, TIMER4B),
    TIMER5: (timer5, Timer5, TIMER5A, TIMER5B),
}

/// Test code to check API
pub fn test() {
    use sysctl::SysctlExt;
    let p = unsafe { tm4c123x::Peripherals::steal() };
    let mut sc = p.SYSCTL.constrain();
    let mut t0 = p
        .TIMER0
        .create(&mut sc.power_control)
        .enter_pwm_mode_a(100, 50, true)
        .enter_pwm_mode_b(100, 50, true);
    t0.start_a();
    t0.start_b();
    t0.enable_interrupt_b();
    t0.enable_interrupt_a();
}