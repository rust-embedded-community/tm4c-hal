#![macro_use]
//! Sample sequencers for ADC

/// 8 Samples 8 FIFO depth
pub struct SS0;
impl SS0 {
    /// Sample sequencer number
    pub fn num() -> u8 {
        return 0;
    }
}
/// 4 Samples 4 FIFO depth
pub struct SS1;
impl SS1 {
    /// Sample sequencer number
    pub fn num() -> u8 {
        return 1;
    }
}
/// 4 Samples 4 FIFO depth
pub struct SS2;
impl SS2 {
    /// Sample sequencer number
    pub fn num() -> u8 {
        return 2;
    }
}
/// 1 Samples 1 FIFO depth
pub struct SS3;
impl SS3 {
    /// Sample sequencer number
    pub fn num() -> u8 {
        return 3;
    }
}

#[macro_export]
/// ssctl field accessor
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

#[macro_export]
/// ssmux field accessor
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

#[macro_export]
/// ssfifo field accessor
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
