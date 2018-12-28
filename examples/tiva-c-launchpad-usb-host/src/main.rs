//! This is a USB Host example
//!
//! The Tm4C12x family have an On-The-Go port, but we're forcing it into USB
//! Host mode. The user will need to supply a micro-AB to USB-A adaptor, in
//! order to plug in a standard USB device to the port on the side of the
//! Launchpad.

#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate tm4c123x_hal as hal;

use core::fmt::Write;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::{entry, exception};
use hal::prelude::*;
use hal::tm4c123x::interrupt;
use hal::usb;

const TICKS_PER_SECOND: u32 = 100;
static TICK_COUNT: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let mut cp = hal::CorePeripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    configure_systick(&mut cp, &clocks);

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let portd = p.GPIO_PORTD.split(&sc.power_control);

    // Activate UART
    let mut uart = hal::serial::Serial::uart0(
        p.UART0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );
    writeln!(uart, "UART activated!").unwrap();

    // Configure the USB pins
    // USB0DM PD4 - USB Analog Mode (Input, Analog, 2mA drive strength)
    let dm = portd.pd4.into_analog_input();
    // USB0DP PD5
    let dp = portd.pd5.into_analog_input();
    // USB0EPEN PF4/PC6/PD2
    // Not required (No control over host power)
    // USB0ID PB0
    // Not required (OTG only)
    // USB0PFLT PC7/PD3
    // ?
    // USB0VBUS PB1
    // ?

    // Create a USB Host object
    let usb_host = usb::UsbHost::new(p.USB0, dm, dp, &mut sc.power_control);

    // Create the USB Host stack using the USB host peripheral
    // TODO

    // Register any class drivers (e.g. HID)
    // TODO

    // Register any device driver (e.g. Keyboard)
    // TODO

    // Start the stack
    //

    loop {
        let time = get_ticks();
        writeln!(uart, "Time is {}", time).unwrap();
        // Poke the stack (telling it the current time).
        // A C stack might fire callbacks from this poke routine - how
        // would we do this in Rust?
        // Maybe each class driver implements a HostClassDriver trait,
        // which allows the stack to pass in events. HostClassDrivers would
        // need to be passed in by reference with dynamic dispatch, as they
        // could be of different sizes, and typing a Host object on six different
        // HostClassDrivers would be impractical.

        // On a host, our general events might be:
        // * Supported Device Added
        // * Unsupported Device Added
        // * Device Removed
        // * Bus power overloaded

        // Applications will want to control the device driver, which will
        // want to send messages to the device via the bus. So the bus itself
        // will need interior mutability, so we can have lots of bus
        // references kicking around.
    }
}

/// Set the Cortex-M SysTick up for TICKS_PER_SECOND ticks.
fn configure_systick(p: &mut hal::CorePeripherals, clocks: &hal::sysctl::Clocks) {
    p.SYST
        .set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    p.SYST.set_reload((clocks.sysclk.0 / TICKS_PER_SECOND) - 1);
    p.SYST.clear_current();
    p.SYST.enable_counter();
    p.SYST.enable_interrupt();
}

/// Called when SysTick underflows (e.g. with 1/TICKS_PER_SECOND seconds has
/// elapsed).
#[exception]
fn SysTick() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// USB interrupt.
interrupt!(USB0, usb_interrupt);
fn usb_interrupt() {}

/// Get the current time, in units of 1/TICKS_PER_SECOND seconds.
fn get_ticks() -> usize {
    TICK_COUNT.load(Ordering::Relaxed)
}
