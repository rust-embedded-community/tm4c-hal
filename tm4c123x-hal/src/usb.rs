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
}

/// The hardware supports a special bi-directional Endpoint 0, plus 7
/// additional endpoints.
#[derive(Debug, Copy, Clone)]
pub enum Endpoint {
    /// Endpoint number 0 (TX & RX)
    ControlEp0,
    /// Endpoint number 1 TX
    TxEp1,
    /// Endpoint number 2 TX
    TxEp2,
    /// Endpoint number 3 TX
    TxEp3,
    /// Endpoint number 4 Tx
    TxEp4,
    /// Endpoint number 5 Tx
    TxEp5,
    /// Endpoint number 6 Tx
    TxEp6,
    /// Endpoint number 7 Tx
    TxEp7,
    /// Endpoint number 1 RX
    RxEp1,
    /// Endpoint number 2 RX
    RxEp2,
    /// Endpoint number 3 RX
    RxEp3,
    /// Endpoint number 4 Rx
    RxEp4,
    /// Endpoint number 5 Rx
    RxEp5,
    /// Endpoint number 6 Rx
    RxEp6,
    /// Endpoint number 7 Rx
    RxEp7,
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
        self.usb.frame.read().frame().bits()
    }

    /// Enable/disable the Start Of Frame interrupt.
    pub fn sof_interrupts_enabled(&mut self, enabled: bool) {
        self.usb.ie.modify(|_r, w| w.sof().bit(enabled))
    }

    /// Resets the USB bus, including the endpoints in any attached device and
    /// pipes on the host. USB bus resets leave the default control pipe
    /// configured (if already configured).
    ///
    /// If the USB bus has been suspended prior to issuing a bus reset, the
    /// attached device will be woken up automatically and the bus resumed
    /// after the reset has been correctly issued.
    pub fn bus_reset(&mut self) {
        self.usb.power.modify(|_r, w| w.reset().set_bit());
    }

    /// Return false if a previous bus reset is still in progress, else return
    /// true.
    pub fn bus_is_reset_complete(&mut self) -> bool {
        // @TODO if we have delayed 20ms here
        if false {
            self.usb.power.modify(|_r, w| w.reset().clear_bit());
            true
        } else {
            false
        }
    }

    /// Resumes USB communications with an attached and enumerated device, by
    /// resuming the transmission of the 1MS Start Of Frame messages to the
    /// device. When resumed, USB communications between the host and attached
    /// device may occur. This takes the host out of suspend mode.
    pub fn bus_resume(&mut self) {
        self.usb.power.modify(|_r, w| w.resume().set_bit());
        // @TODO Delay at least 10ms but not more than 15ms
        self.usb.power.modify(|_r, w| w.resume().clear_bit());
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
        self.usb.power.modify(|_r, w| w.suspend().set_bit());
    }

    /// Return true if the bus is currently suspended.
    pub fn bus_is_suspended(&self) -> bool {
        self.usb.power.read().suspend().bit_is_set()
    }

    /// Returns the speed of the attached device.
    pub fn bus_speed(&self) -> UsbSpeed {
        if self.usb.devctl.read().fsdev().bit_is_set() {
            UsbSpeed::FullSpeed
        } else {
            UsbSpeed::LowSpeed
        }
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
    /// address and port of the hub it is connected to. It will optionally set
    /// the speed for the endpoint too.
    pub fn set_device_address_for_endpoint(
        &mut self,
        endpoint: Endpoint,
        address: DeviceAddress,
        connection: DeviceConnection,
        speed: Option<UsbSpeed>,
    ) {
        let (hub_address, port) = match connection {
            None => (0, 0),
            Some((hub_address, port)) => (hub_address.0, port),
        };
        match endpoint {
            Endpoint::ControlEp0 => {
                // Also controls the RX addresses as they are the same
                self.usb
                    .txfuncaddr0
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr0
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport0
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.type0.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.type0.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp1 => {
                self.usb
                    .txfuncaddr1
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr1
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport1
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype1.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype1.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp2 => {
                self.usb
                    .txfuncaddr2
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr2
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport2
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype2.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype2.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp3 => {
                self.usb
                    .txfuncaddr3
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr3
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport3
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype3.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype3.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp4 => {
                self.usb
                    .txfuncaddr4
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr4
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport4
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype4.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype4.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp5 => {
                self.usb
                    .txfuncaddr5
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr5
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport5
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype5.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype5.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp6 => {
                self.usb
                    .txfuncaddr6
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr6
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport6
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype6.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype6.write(|w| w.speed().full()),
                }
            }
            Endpoint::TxEp7 => {
                self.usb
                    .txfuncaddr7
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .txhubaddr7
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .txhubport7
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.txtype7.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.txtype7.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp1 => {
                self.usb
                    .rxfuncaddr1
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr1
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport1
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype1.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype1.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp2 => {
                self.usb
                    .rxfuncaddr2
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr2
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport2
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype2.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype2.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp3 => {
                self.usb
                    .rxfuncaddr3
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr3
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport3
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype3.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype3.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp4 => {
                self.usb
                    .rxfuncaddr4
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr4
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport4
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype4.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype4.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp5 => {
                self.usb
                    .rxfuncaddr5
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr5
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport5
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype5.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype5.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp6 => {
                self.usb
                    .rxfuncaddr6
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr6
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport6
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype6.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype6.write(|w| w.speed().full()),
                }
            }
            Endpoint::RxEp7 => {
                self.usb
                    .rxfuncaddr7
                    .write(|w| unsafe { w.addr().bits(address.0) });
                self.usb
                    .rxhubaddr7
                    .write(|w| unsafe { w.addr().bits(hub_address) });
                self.usb
                    .rxhubport7
                    .write(|w| unsafe { w.port().bits(port) });
                match speed {
                    None => {}
                    Some(UsbSpeed::LowSpeed) => self.usb.rxtype7.write(|w| w.speed().low()),
                    Some(UsbSpeed::FullSpeed) => self.usb.rxtype7.write(|w| w.speed().full()),
                }
            }
        }
    }

    /// Get which USB device a particular endpoint is connected to. If this
    /// device is not directly connected, you also get the device address and
    /// port of the hub it is connected to.
    pub fn get_device_address_for_endpoint(
        &mut self,
        endpoint: Endpoint,
    ) -> (DeviceAddress, DeviceConnection) {
        let (address, hub_address, hub_port) = match endpoint {
            Endpoint::ControlEp0 => {
                let address = self.usb.txfuncaddr0.read().bits();
                let hub_address = self.usb.txhubaddr0.read().addr().bits();
                let hub_port = self.usb.txhubport0.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp1 => {
                let address = self.usb.txfuncaddr1.read().bits();
                let hub_address = self.usb.txhubaddr1.read().addr().bits();
                let hub_port = self.usb.txhubport1.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp2 => {
                let address = self.usb.txfuncaddr2.read().bits();
                let hub_address = self.usb.txhubaddr2.read().addr().bits();
                let hub_port = self.usb.txhubport2.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp3 => {
                let address = self.usb.txfuncaddr3.read().bits();
                let hub_address = self.usb.txhubaddr3.read().addr().bits();
                let hub_port = self.usb.txhubport3.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp4 => {
                let address = self.usb.txfuncaddr4.read().bits();
                let hub_address = self.usb.txhubaddr4.read().addr().bits();
                let hub_port = self.usb.txhubport4.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp5 => {
                let address = self.usb.txfuncaddr5.read().bits();
                let hub_address = self.usb.txhubaddr5.read().addr().bits();
                let hub_port = self.usb.txhubport5.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp6 => {
                let address = self.usb.txfuncaddr6.read().bits();
                let hub_address = self.usb.txhubaddr6.read().addr().bits();
                let hub_port = self.usb.txhubport6.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::TxEp7 => {
                let address = self.usb.txfuncaddr7.read().bits();
                let hub_address = self.usb.txhubaddr7.read().addr().bits();
                let hub_port = self.usb.txhubport7.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp1 => {
                let address = self.usb.rxfuncaddr1.read().bits();
                let hub_address = self.usb.rxhubaddr1.read().addr().bits();
                let hub_port = self.usb.rxhubport1.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp2 => {
                let address = self.usb.rxfuncaddr2.read().bits();
                let hub_address = self.usb.rxhubaddr2.read().addr().bits();
                let hub_port = self.usb.rxhubport2.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp3 => {
                let address = self.usb.rxfuncaddr3.read().bits();
                let hub_address = self.usb.rxhubaddr3.read().addr().bits();
                let hub_port = self.usb.rxhubport3.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp4 => {
                let address = self.usb.rxfuncaddr4.read().bits();
                let hub_address = self.usb.rxhubaddr4.read().addr().bits();
                let hub_port = self.usb.rxhubport4.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp5 => {
                let address = self.usb.rxfuncaddr5.read().bits();
                let hub_address = self.usb.rxhubaddr5.read().addr().bits();
                let hub_port = self.usb.rxhubport5.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp6 => {
                let address = self.usb.rxfuncaddr6.read().bits();
                let hub_address = self.usb.rxhubaddr6.read().addr().bits();
                let hub_port = self.usb.rxhubport6.read().port().bits();
                (address, hub_address, hub_port)
            }
            Endpoint::RxEp7 => {
                let address = self.usb.rxfuncaddr7.read().bits();
                let hub_address = self.usb.rxhubaddr7.read().addr().bits();
                let hub_port = self.usb.rxhubport7.read().port().bits();
                (address, hub_address, hub_port)
            }
        };
        if hub_address != 0 {
            (
                DeviceAddress(address),
                Some((DeviceAddress(hub_address), hub_port)),
            )
        } else {
            (DeviceAddress(address), None)
        }
    }
}

impl Into<u8> for DeviceAddress {
    fn into(self) -> u8 {
        self.0
    }
}

// End of file
