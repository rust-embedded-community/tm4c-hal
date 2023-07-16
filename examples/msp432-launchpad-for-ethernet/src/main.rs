#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use core::fmt::Write;
use cortex_m_rt::entry;
use tm4c129x_hal::{self as hal, prelude::*};

#[entry]
fn main() -> ! {
    let cp = hal::CorePeripherals::take().unwrap();
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_25mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_120mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA_AHB.split(&sc.power_control);
    let portn = p.GPIO_PORTN.split(&sc.power_control);
    let portf = p.GPIO_PORTF_AHB.split(&sc.power_control);

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
    let mut led1 = portn.pn1.into_push_pull_output();
    let mut led2 = portn.pn0.into_push_pull_output();
    let mut led3 = portf.pf4.into_push_pull_output();
    let mut led4 = portf.pf0.into_push_pull_output();

    let mut counter = 0u32;
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 120_000_000u32);
    loop {
        writeln!(uart, "Hello, world! counter={}", counter).unwrap();
        let led_state = counter % 4;
        if led_state == 0 {
            led1.set_high();
            led2.set_low();
            led3.set_low();
            led4.set_low();
        }
        if led_state == 1 {
            led1.set_low();
            led2.set_high();
            led3.set_low();
            led4.set_low();
        }
        if led_state == 2 {
            led1.set_low();
            led2.set_low();
            led3.set_high();
            led4.set_low();
        }
        if led_state == 3 {
            led1.set_low();
            led2.set_low();
            led3.set_low();
            led4.set_high();
        }

        counter = counter.wrapping_add(1);
        delay.delay_ms(200);
    }
}
