//! Prelude

#[rustfmt::skip]
pub use crate::{
    gpio::GpioExt as _,
    hal::prelude::*,
    sysctl::SysctlExt,
    time::U32Ext,
};
