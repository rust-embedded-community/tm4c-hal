//! USB Host, Device and OTG support

use crate::gpio::{InputMode, Analog};
use crate::gpio::gpiod::{PD4, PD5};

/// Puts the on-board USB controller into USB Host support (i.e. controls the
/// bus and allows the system to talk to either a single connected USB device,
/// or a tree of USB hubs and USB devices).
///
/// The Launchpad has a micro-AB connector, so you will need a micro-AB to
/// full-size USB A adaptor to connect standard USB devices with a USB A to USB B
/// cable.
pub struct UsbHost {
	usb: tm4c123x::USB0,
	dm: crate::gpio::gpiod::PD4,
	dp: crate::gpio::gpiod::PD5,
}

// FIXME these should be "closed" traits
/// SCK pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DataPlusPin<USB> {}

/// MISO pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DataMinusPin<USB> {}

unsafe impl<T> DataPlusPin<tm4c123x::USB0> for PD5<Input<Analog>> where T: OutputMode {}
unsafe impl<T> DataMinusPin<tm4c123x::USB0> for PD4<Input<Analog>> where T: OutputMode {}

impl UsbHost {
	pub fn new(usb: tm4c123x::USB0, dm: crate::gpio::gpiod::PD4<_>, dp: crate::gpio::gpiod::PD5) -> UsbHost {
		// Power up USB controller here,
		// plus the pins we need.
		UsbHost {
			usb,
			dm,
			dp
		}
	}
}

// End of file
