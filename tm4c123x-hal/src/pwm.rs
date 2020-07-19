//! PWM abstractions

use crate::gpio::{gpiob, gpioc, gpiof, AlternateFunction, PushPull, AF7};

/// a timer
pub struct Timer<T> {
    timer: T,
}

/// Implemented for any pin that can be the Even CCP Pin (i.e. associated with timer A) of a timer
/// peripheral.
pub trait EvenPin<T> {}

/// Implemented for any pin that can be the Odd CCP Pin (i.e. associated with timer B) of a timer
/// peripheral.
pub trait OddPin<T> {}

/// PWM output from the even half (i.e. timer A) of a timer peripheral
pub struct EvenPWM<T> {
    timer: T,
}

/// PWM output from the odd half (i.e. timer B) of a timer peripheral
pub struct OddPWM<T> {
    timer: T,
}

macro_rules! into_one_half {
    ($Name:ident, $timer:path, $trait:path, $kind:ident, $mr:ident, $plo:ident, $mrsu:ident,
            $pwmie:ident, $cdir:ident, $ams:ident, $mr_module:ty) => {
        /// Create the PWM implementation for one half of a timer peripheral
        pub fn $Name<P: $trait>(self, _pin: P) -> $kind<$timer> {
            self.timer.$mr.write(|w| {
                w.$plo().set_bit();
                w.$mrsu().set_bit();
                w.$pwmie().clear_bit();
                w.$cdir().set_bit();
                w.$ams().set_bit();
                w.$mr().variant(<$mr_module>::PERIOD)
            });
            $kind { timer: self.timer }
        }
    };
}

macro_rules! impl_for_timer {
    ($Name:ident, $timer:path, $domain:expr, even: [$($($even_pins:ident)::+),+],
            odd: [$($($odd_pins:ident)::+),+]) => {
        impl_pwm!($timer);

        $(
            impl EvenPin<Timer<$timer>> for $($even_pins)::+<AlternateFunction<AF7, PushPull>> {}
        )+

        $(
            impl OddPin<Timer<$timer>> for $($odd_pins)::+<AlternateFunction<AF7, PushPull>> {}
        )+

        impl Timer<$timer> {
            /// Initialize timer for PWM usage
            pub fn $Name(power_control: &crate::sysctl::PowerControl, timer: $timer) -> Self {
                crate::sysctl::control_power(
                    power_control,
                    $domain,
                    crate::sysctl::RunMode::Run,
                    crate::sysctl::PowerState::On,
                );
                crate::sysctl::reset(power_control, $domain);
                timer
                    .cfg
                    .modify(|_r, w| w.cfg().variant(tm4c123x::timer0::cfg::CFG_A::_16_BIT));
                Timer { timer }
            }
            into_one_half!(
                into_even,
                $timer,
                EvenPin<Self>,
                EvenPWM,
                tamr,
                taplo,
                tamrsu,
                tapwmie,
                tacdir,
                taams,
                tm4c123x::timer0::tamr::TAMR_A
            );
            into_one_half!(
                into_odd,
                $timer,
                OddPin<Self>,
                OddPWM,
                tbmr,
                tbplo,
                tbmrsu,
                tbpwmie,
                tbcdir,
                tbams,
                tm4c123x::timer0::tbmr::TBMR_A
            );
            /// Create the PWM implementation for both halves of a timer peripheral
            pub fn into_both<E: EvenPin<Self>, O: OddPin<Self>>(
                self,
                even_pin: E,
                odd_pin: O,
            ) -> (EvenPWM<$timer>, OddPWM<$timer>) {
                // this is effectively cloning self, but the even and odd halves do not clobber the
                // same registers.  I think this will be safe as long as tm4c123x::TIMERn is a
                // zero-sized type, or otherwise Timer<T> remains safe to bitwise-copy.
                let mind: Self = unsafe { core::ptr::read(&self as *const _) };
                let even = mind.into_even(even_pin);
                let odd = self.into_odd(odd_pin);
                (even, odd)
            }
        }
    };
}

macro_rules! pwm_half {
    ($StructName:ident, $timer:path, $en_bit:expr, $ilr:ident, $matchr:ident) => {
        /// One half of a PWM timer
        impl embedded_hal::Pwm for $StructName<$timer> {
            type Channel = ();
            type Time = u32; // clock cycles, proper abstraction tbd
            type Duty = u32; // also clock cycles

            fn enable(&mut self, _: ()) {
                unsafe { crate::bb::change_bit(&self.timer.ctl, $en_bit, true) }
            }

            fn disable(&mut self, _: ()) {
                unsafe { crate::bb::change_bit(&self.timer.ctl, $en_bit, false) }
            }

            fn get_period(&self) -> Self::Time {
                self.timer.$ilr.read().bits()
            }

            fn set_period<P: Into<Self::Time>>(&mut self, period: P) {
                self.timer.$ilr.write(|w| unsafe { w.bits(period.into()) });
            }

            fn get_duty(&self, _: ()) -> Self::Duty {
                let thresh = self.timer.$matchr.read().bits();
                let period = self.get_period();
                period - thresh
            }

            fn get_max_duty(&self) -> Self::Duty {
                self.get_period()
            }

            fn set_duty(&mut self, _: (), duty: Self::Duty) {
                self.timer
                    .$matchr
                    .write(|w| unsafe { w.bits(self.get_period() - duty) });
            }
        }
    };
}

macro_rules! impl_pwm {
    ($timer:path) => {
        pwm_half!(EvenPWM, $timer, 0, tailr, tamatchr);
        pwm_half!(OddPWM, $timer, 8, tbilr, tbmatchr);
    };
}

impl_for_timer!(
    timer0,
    tm4c123x::TIMER0,
    crate::sysctl::Domain::Timer0,
    even: [gpiob::PB6, gpiof::PF0],
    odd: [gpiob::PB7, gpiof::PF1]
);

impl_for_timer!(
    timer1,
    tm4c123x::TIMER1,
    crate::sysctl::Domain::Timer1,
    even: [gpiof::PF2, gpiob::PB4],
    odd: [gpiof::PF3, gpiob::PB5]
);

impl_for_timer!(
    timer2,
    tm4c123x::TIMER2,
    crate::sysctl::Domain::Timer2,
    even: [gpiof::PF4, gpiob::PB0],
    odd: [gpiob::PB1]
);

impl_for_timer!(
    timer3,
    tm4c123x::TIMER3,
    crate::sysctl::Domain::Timer3,
    even: [gpiob::PB2],
    odd: [gpiob::PB3]
);

impl_for_timer!(
    timer4,
    tm4c123x::TIMER4,
    crate::sysctl::Domain::Timer4,
    even: [gpioc::PC0],
    odd: [gpioc::PC1]
);

impl_for_timer!(
    timer5,
    tm4c123x::TIMER5,
    crate::sysctl::Domain::Timer5,
    even: [gpioc::PC2],
    odd: [gpioc::PC3]
);
