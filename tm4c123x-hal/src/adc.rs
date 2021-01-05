//! Analog-to-Digital Converter
use core::marker::PhantomData;

use crate::{
    bb,
    gpio::gpiob::{PB4, PB5},
    gpio::gpiod::{PD0, PD1, PD2, PD3},
    gpio::gpioe::{PE0, PE1, PE2, PE3, PE4, PE5},
    hal::adc,
    sysctl,
};

use tm4c123x::{ADC0, ADC1};
use tm4c_hal::gpio::{AnalogInput, Input};
use SampleSeq::{SS0, SS1, SS2, SS3};

#[allow(non_snake_case)]
/// Sample sequencers available
pub mod SampleSeq {
    /// 8 Samples 8 FIFO depth
    pub struct SS0;
    /// 4 Samples 4 FIFO depth
    pub struct SS1;
    /// 4 Samples 4 FIFO depth
    pub struct SS2;
    /// 1 Samples 1 FIFO depth
    pub struct SS3;
}

macro_rules! ss_ctl {
    ($adc:ident, SS0) => {
        $adc.ssctl0
    };
    ($adc:ident, SS1) => {
        $adc.ssctl1
    };
    ($adc:ident, SS2) => {
        $adc.ssctl2
    };
    ($adc:ident, SS3) => {
        $adc.ssctl3
    };
}

macro_rules! ss_mux {
    ($adc:ident, SS0) => {
        $adc.ssmux0
    };
    ($adc:ident, SS1) => {
        $adc.ssmux1
    };
    ($adc:ident, SS2) => {
        $adc.ssmux2
    };
    ($adc:ident, SS3) => {
        $adc.ssmux3
    };
}

macro_rules! ss_num {
    (SS0) => {
        0
    };
    (SS1) => {
        1
    };
    (SS2) => {
        2
    };
    (SS3) => {
        3
    };
}

macro_rules! ss_fifo {
    ($adc:ident, SS0) => {
        $adc.ssfifo0
    };
    ($adc:ident, SS1) => {
        $adc.ssfifo1
    };
    ($adc:ident, SS2) => {
        $adc.ssfifo2
    };
    ($adc:ident, SS3) => {
        $adc.ssfifo3
    };
}

/// Example ADC
pub struct Adc<SS, ADC, PIN> {
    /// adc peripheral
    adc: ADC,

    // input pin
    #[allow(dead_code)]
    pin: PIN,

    // SampleSequencer being used
    _ss: PhantomData<SS>,
}

macro_rules! pin_hal {
    ($ ($PXN:ident: ($num:expr), )+) => {
        $(
            impl adc::Channel<ADC0> for $PXN<Input<AnalogInput>> {
                type ID = u8;

                fn channel() -> u8 {
                    $num
                }
            }

            impl adc::Channel<ADC1> for $PXN<Input<AnalogInput>> {
                type ID = u8;

                fn channel() -> u8 {
                    $num
                }
            }
        )+
    }
}

macro_rules! init_hal {
    ($($ADCX:ident: ($powerDomain:ident, $adcX:ident, $SSX:ident),)+) => {
        $(

            impl<PIN> Adc<$SSX, $ADCX, PIN>
            where
                PIN: adc::Channel<$ADCX>,
                PIN::ID: Into<u32>
            {
                /// Configures the ADC peripheral to operate in full duplex master mode
                pub fn $adcX(
                    adc: $ADCX,
                    pin: PIN,
                    pc: &sysctl::PowerControl,
                ) -> Self
                {
                    // power up
                    sysctl::control_power(
                        pc, sysctl::Domain::$powerDomain,
                        sysctl::RunMode::Run, sysctl::PowerState::On);
                    sysctl::reset(pc, sysctl::Domain::$powerDomain);

                    unsafe { // disable for config
                        bb::change_bit(&adc.actss, ss_num!($SSX), false);
                    }
                    adc.emux.modify(|r, w| unsafe { w.bits(r.bits() & !(0xF000)) }); // software trigger

                    ss_mux!(adc, $SSX).write(|w| unsafe { w.bits(PIN::channel().into()) });

                    unsafe {
                        bb::change_bit(&ss_ctl!(adc, $SSX), 1, true);
                        bb::change_bit(&ss_ctl!(adc, $SSX), 2, true);
                        bb::change_bit(&adc.actss, ss_num!($SSX), true);
                    }

                    Adc { _ss: PhantomData, pin, adc }
                }

                /// Configured channel
                pub fn channel(&self) -> PIN::ID {
                    PIN::channel()
                }

                /// Blocking read
                pub fn read_blocking(&self) -> u16 {
                    unsafe {
                        bb::change_bit(&self.adc.pssi, ss_num!($SSX), true); // Enable SS3 conversion or start sampling data from AN0
                    }
                    while (&self.adc.ris.read().bits() & 8) == 0 {
                        // cortex_m::asm::nop();
                    }
                    let ref _adc = self.adc;
                    let adc_value = ss_fifo!(_adc, $SSX).read().data().bits(); //clear coversion clear flag bit
                    &self.adc.isc.write(|w| unsafe { w.bits(8) });
                    adc_value
                }
            }
        )+
    }
}

init_hal! {
    ADC0: (Adc0, adc0, SS0),
    ADC0: (Adc0, adc0, SS1),
    ADC0: (Adc0, adc0, SS2),
    ADC0: (Adc0, adc0, SS3),
    ADC1: (Adc1, adc1, SS0),
    ADC1: (Adc1, adc1, SS1),
    ADC1: (Adc1, adc1, SS2),
    ADC1: (Adc1, adc1, SS3),
}

pin_hal! {
    PE3: (0),
    PE2: (1),
    PE1: (2),
    PE0: (3),
    PD3: (4),
    PD2: (5),
    PD1: (6),
    PD0: (7),
    PE5: (8),
    PE4: (9),
    PB4: (10),
    PB5: (11),
}
