//! Timers

extern crate embedded_hal as hal;

use tm4c_hal::time::Hertz;

use crate::sysctl;
use hal::timer::{CountDown, Periodic};
use nb;
use tm4c123x::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5};
use tm4c123x::{WTIMER0, WTIMER1, WTIMER2, WTIMER3, WTIMER4, WTIMER5};

use crate::sysctl::Clocks;
use void::Void;

/// Hardware timers
pub struct Timer<TIM> {
    tim: TIM,
    clocks: Clocks,
    timeout: Hertz,
}

/// Interrupt events
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $powerDomain:ident),)+) => {
        $(
            impl Periodic for Timer<$TIM> {}

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                #[allow(unused_unsafe)]
                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    // Disable timer
                    self.tim.ctl.modify(|_, w|
					w.taen().clear_bit()
					.tben().clear_bit()
                    );
                    self.timeout = timeout.into();

                    let frequency = self.timeout.0;
                    let ticks = self.clocks.sysclk.0 / frequency;

                    self.tim.tav.write(|w| unsafe { w.bits(ticks) });
                    self.tim.tailr.write(|w| unsafe { w.bits(ticks) });

                    // // start counter
                    self.tim.ctl.modify(|_, w|
                        w.taen().set_bit()
                    );
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.ris.read().tatoris().bit_is_clear () {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.icr.write(|w| w.tatocint().set_bit());
                        Ok(())
                    }
                }
            }

            impl Timer<$TIM> {
                // XXX(why not name this `new`?) bummer: constructors need to have different names
                // even if the `$TIM` are non overlapping (compare to the `free` function below
                // which just works)
                /// Configures a TIM peripheral as a periodic count down timer
                pub fn $tim<T>(tim: $TIM, timeout: T,
                               pc: &sysctl::PowerControl,
                               clocks: &Clocks,
                ) -> Self
                where
                    T: Into<Hertz>,
                {
                    // power up
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    // Stop Timers
                    tim.ctl.write(|w|
                                  w.taen().clear_bit()
                                  .tben().clear_bit()
                                  .tastall().set_bit()
                    );

                    // GPTMCFG = 0x0 (chained - 2x16 = 32bits) This
                    // will not force 32bits wide timer, this will
                    // really force the wider range to be used (32 for
                    // 16/32bits timers, 64 for 32/64).
                    tim.cfg.write(|w| w.cfg()._32_bit_timer());

                    tim.tamr.write(|w| w.tamr().period());

                    let mut timer = Timer {
                        tim:tim,
                        clocks: *clocks,
                        timeout: Hertz(0),
                    };
                    timer.start(timeout);

                    timer
                }

                /// Starts listening for an `event`
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.imr.modify(|_,w|  w.tatoim().set_bit());
                        }
                    }
                }

                /// Stops listening for an `event`
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.imr.modify(|_,w| w.tatoim().clear_bit());
                        }
                    }
                }

                /// Releases the TIM peripheral
                pub fn free(self) -> $TIM {
                    // pause counter
                    self.tim.ctl.write(|w|
                                  w.taen().clear_bit()
                                  .tben().clear_bit());
                    self.tim
                }
            }
        )+
    }
}

hal! {
    TIMER0: (timer0, Timer0),
    TIMER1: (timer1, Timer1),
    TIMER2: (timer2, Timer2),
    TIMER3: (timer3, Timer3),
    TIMER4: (timer4, Timer4),
    TIMER5: (timer5, Timer5),

    WTIMER0: (wtimer0, WideTimer0),
    WTIMER1: (wtimer1, WideTimer1),
    WTIMER2: (wtimer2, WideTimer2),
    WTIMER3: (wtimer3, WideTimer3),
    WTIMER4: (wtimer4, WideTimer4),
    WTIMER5: (wtimer5, WideTimer5),
}
