//! USB Host, Device and OTG support
//!
//! The API here is based on LUFA (https://github.com/abcminiuser/lufa)
//! Where documentation or code has been taken from LUFA, that material is
//! Copyright 2018  Dean Camera (dean [at] fourwalledcubicle [dot] com).
//! Those portions are available under this licence:
//!
//!    Permission to use, copy, modify, distribute, and sell this
//!    software and its documentation for any purpose is hereby granted
//!    without fee, provided that the above copyright notice appear in
//!    all copies and that both that the copyright notice and this
//!    permission notice and warranty disclaimer appear in supporting
//!    documentation, and that the name of the author not be used in
//!    advertising or publicity pertaining to distribution of the
//!    software without specific, written prior permission.
//!
//!    The author disclaims all warranties with regard to this
//!    software, including all implied warranties of merchantability
//!    and fitness.  In no event shall the author be liable for any
//!    special, indirect or consequential damages or any damages
//!    whatsoever resulting from loss of use, data or profits, whether
//!    in an action of contract, negligence or other tortious action,
//!    arising out of or in connection with the use or performance of
//!    this software.

use crate::gpio::gpiod::{PD4, PD5};
use crate::gpio::{Analog, Input};
use crate::sysctl;

/// Puts the on-board USB controller into USB Host support (i.e. controls the
/// bus and allows the system to talk to either a single connected USB device,
/// or a tree of USB hubs and USB devices).
///
/// The TM4C123G Launchpad has a micro-AB connector, so you will need a
/// micro-AB to full-size USB A adaptor to connect standard USB devices with a
/// USB A to USB B cable. You will also need to set the USB power switch to
/// "Device", and provide 5V on the VBUS pin of your board.
///
/// The LM4F120 does not support USB Host.
pub struct UsbHost<DM, DP> {
    usb: tm4c123x::USB0,
    dm: DM,
    dp: DP,
}

// FIXME these should be "closed" traits
/// DP pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DataPlusPin<USB> {}

/// DM pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DataMinusPin<USB> {}

// Analog mode on these pins means USB mode (see Table 23-5 in the TRM)
unsafe impl DataPlusPin<tm4c123x::USB0> for PD5<Input<Analog>> {}
unsafe impl DataMinusPin<tm4c123x::USB0> for PD4<Input<Analog>> {}

/// The different speeds of USB device
pub enum UsbSpeed {
    /// A 1.5 Mbit/sec device
    LowSpeed,
    /// A 11 Mbit/sec device
    FullSpeed,
    /// A 480 Mbit/sec device
    HighSpeed,
    /// A 5.0 Gbit/sec device
    SuperSpeed,
}

/// The hardware supports a special bi-directional Endpoint 0, plus 7
/// additional endpoints.
#[derive(Debug, Copy, Clone)]
pub enum Endpoint {
    /// Endpoint number 0
    EP0,
    /// Endpoint number 1
    EP1,
    /// Endpoint number 2
    EP2,
    /// Endpoint number 3
    EP3,
    /// Endpoint number 4
    EP4,
    /// Endpoint number 5
    EP5,
    /// Endpoint number 6
    EP6,
    /// Endpoint number 7
    EP7,
}

/// Represents a 7-bit USB device address.
#[derive(Debug, Copy, Clone)]
pub struct DeviceAddress(u8);

/// Represents a hub device ID and port (or None, meaning it is the root device)
pub type DeviceConnection = Option<(DeviceAddress, u8)>;

/// Represents a device directly connected to our one and only root port (i.e.
/// not connected to a hub).
pub const DEVICE_CONNECTION_ROOT: DeviceConnection = None;

impl<DM, DP> UsbHost<DM, DP>
where
    DM: DataMinusPin<tm4c123x::USB0>,
    DP: DataPlusPin<tm4c123x::USB0>,
{
    /// Create a new `UsbHost` controller, using the on-board USB Device/Host/OTG peripheral.
    pub fn new(
        usb: tm4c123x::USB0,
        dm: DM,
        dp: DP,
        pc: &mut sysctl::PowerControl,
    ) -> UsbHost<DM, DP> {
        // Power up USB controller here,
        // plus the pins we need.
        sysctl::control_power(
            pc,
            sysctl::Domain::Usb,
            sysctl::RunMode::Run,
            sysctl::PowerState::On,
        );
        sysctl::reset(pc, sysctl::Domain::Usb);
        // Set DEVMODOTG so system reads DEVMOD instead of reading pin PB1 and
        // then clear DEVMOD (which means 'Host Mode')
        usb.gpcs
            .modify(|_r, w| w.devmodotg().set_bit().devmod().clear_bit());

        // Return object
        UsbHost {
            usb,
            dm: dm,
            dp: dp,
        }
    }

    /// Return the original peripheral and pins.
    pub fn destroy(self, pc: &mut sysctl::PowerControl) -> (tm4c123x::USB0, DM, DP) {
        sysctl::reset(pc, sysctl::Domain::Usb);
        sysctl::control_power(
            pc,
            sysctl::Domain::Usb,
            sysctl::RunMode::Run,
            sysctl::PowerState::Off,
        );
        (self.usb, self.dm, self.dp)
    }

    /// Returns the current USB Frame Number. When the bus isn't suspended,
    /// this values goes up by one every millisecond.
    pub fn get_frame_number(&self) -> u16 {
        unimplemented!();
    }

    /// Enable/disable the Start Of Frame interrupt.
    pub fn sof_interrupts_enabled(&mut self, _enabled: bool) {
        unimplemented!();
    }

    /// Resets the USB bus, including the endpoints in any attached device and
    /// pipes on the AVR host. USB bus resets leave the default control pipe
    /// configured (if already configured).
    ///
    /// If the USB bus has been suspended prior to issuing a bus reset, the
    /// attached device will be woken up automatically and the bus resumed
    /// after the reset has been correctly issued.
    pub fn bus_reset(&mut self) {
        unimplemented!();
    }

    /// Return false if a previous bus reset is still in progress, else return
    /// true.
    pub fn bus_is_reset_complete(&mut self) -> bool {
        unimplemented!()
    }

    /// Resumes USB communications with an attached and enumerated device, by
    /// resuming the transmission of the 1MS Start Of Frame messages to the
    /// device. When resumed, USB communications between the host and attached
    /// device may occur.
    pub fn bus_resume(&mut self) {
        unimplemented!();
    }

    /// Suspends the USB bus, preventing any communications from occurring
    /// between the host and attached device until the bus has been resumed.
    /// This stops the transmission of the 1MS Start Of Frame messages to the
    /// device.
    ///
    /// Note: While the USB bus is suspended, all USB interrupt sources are also
    ///       disabled; this means that some events (such as device
    ///       disconnections) will not fire until the bus is resumed.
    pub fn bus_suspend(&mut self) {
        unimplemented!();
    }

    /// Return true if the bus is currently suspended.
    pub fn bus_is_suspended(&self) -> bool {
        unimplemented!();
    }

    /// Returns the speed of the attached device.
    pub fn bus_speed(&self) -> UsbSpeed {
        unimplemented!();
    }

    /// Determines if the attached device is currently issuing a Remote Wakeup
    /// event.
    pub fn remote_wakeup_is_sent(&self) -> bool {
        unimplemented!();
    }

    /// Clears the flag indicating that a Remote Wakeup request has been
    /// issued by an attached device.
    pub fn remote_wakeup_clear_sent(&mut self) {
        unimplemented!();
    }

    /// Accepts a Remote Wakeup request from an attached device. This must be
    /// issued in response to a device's Remote Wakeup request within 2ms for
    /// the request to be accepted and the bus to be resumed.
    pub fn resume_from_wakeup_request(&mut self) {
        unimplemented!();
    }

    /// Determines if a resume from Remote Wakeup request is currently being
    /// sent to an attached device.
    pub fn is_resume_from_wakeup_request_sent(&self) -> bool {
        unimplemented!();
    }

    /// Specify which USB device a particular endpoint is connected to. If
    /// this device is not directly connected, you can also specify the device
    /// address and port of the hub it is connected to.
    pub fn set_device_address_for_endpoint(
        &mut self,
        _endpoint: Endpoint,
        _address: DeviceAddress,
        _connection: DeviceConnection,
    ) {
        unimplemented!();
    }

    /// Get which USB device a particular endpoint is connected to. If this
    /// device is not directly connected, you also get the device address and
    /// port of the hub it is connected to.
    pub fn get_device_address_for_endpoint(
        &mut self,
        _endpoint: Endpoint,
    ) -> (DeviceAddress, DeviceConnection) {
        unimplemented!();
    }
}

impl Into<u8> for DeviceAddress {
    fn into(self) -> u8 {
        self.0
    }
}

// End of file
