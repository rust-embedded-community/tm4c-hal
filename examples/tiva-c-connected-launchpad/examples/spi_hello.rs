#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use core::fmt::Write;
use cortex_m_rt::entry;
use tm4c129x_hal::{self as hal, prelude::*};

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_120mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let cp = hal::CorePeripherals::take().unwrap();
    let syst = cp.SYST;
    let mut delay = hal::delay::Delay::new(syst, &clocks);

    let mut porta = p.GPIO_PORTA_AHB.split(&sc.power_control);

    // Initialise SPI
    let mut spi = hal::spi::Spi::spi0(
        p.SSI0,
        (
            // CLK
            porta
                .pa2
                .into_af_push_pull::<hal::gpio::AF15>(&mut porta.control),
            // MISO
            porta
                .pa5
                .into_af_push_pull::<hal::gpio::AF15>(&mut porta.control),
            // MOSI
            porta
                .pa4
                .into_af_push_pull::<hal::gpio::AF15>(&mut porta.control),
        ),
        // Mode
        hal::spi::MODE_0,
        // Frequency
        1_u32.mhz(),
        // Clock
        &clocks,
        // Power Control
        &sc.power_control,
    );

    // Initialise CS pin
    let mut cs = porta
        .pa3
        .into_push_pull_output();
    cs.set_high();

    loop {
        let message = "Hello, World!";
        cs.set_low();
        for c in message.chars() {
            spi.send(c as u8);
        }
        cs.set_high();
    }
}
