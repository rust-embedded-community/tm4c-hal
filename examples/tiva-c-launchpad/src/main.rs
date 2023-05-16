#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use core::fmt::Write;
use cortex_m_rt::entry;
use tm4c123x_hal::{self as hal, prelude::*};
use tm4c123x_hal::eeprom::Eeprom;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    let eeprom = Eeprom::new(p.EEPROM, &sc.power_control);

    let mut eeprom_buffer = [0 as u8; 64]; // 64 byte read buffer

    let address = eeprom.word_offset_to_address(52).unwrap();
    assert_eq!(address.block(), 3, "Word 50 should be in block 3, offset 4");
    assert_eq!(address.offset(), 4, "Word 50 should be in block 3, offset 4");

    let word_index = eeprom.address_to_word_offset(&address).unwrap();
    assert_eq!(word_index, 52, "Word index for block 3, offset 4 should be 52");

    // Simplest case, middle of a block, no straddle
    let test_array_1: [u8; 4] = [1, 2, 3, 4];
    test_write_read(eeprom, &address, &test_array_1, &mut buffer);

    // Test boundry conditions for access that straddles a block
    let test_array_2: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let address_straddle_block = EepromAddress::new(0, 12);
    test_write_read(eeprom, &address_straddle_block, &test_array_2, &mut buffer);

    // Test case for unaligned EEPROM access.
    let unaligned_read_write_address = EepromAddress::new(0, 13);
    match eeprom.write(&unaligned_read_write_address, &test_array_2) {
        Ok(_) => {
            assert!(true, "Unaligned word read / write should NOT be ok");
        }
        Err(code) => {
            assert_eq!(code, EepromError::OffsetShouldBeWordAligned, "This write test should fail due to alignment issues");
        }
    }

    match eeprom.read(&unaligned_read_write_address, 4, &mut buffer) {
        Ok(_) => {
            assert!(true, "Unaligned word read / write should NOT be ok");
        }
        Err(code) => {
            assert_eq!(code, EepromError::OffsetShouldBeWordAligned, "This read test should fail due to alignment issues");
        }
    }

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

    let mut counter = 0u32;
    loop {
        writeln!(uart, "Hello, world! counter={}", counter).unwrap();
        counter = counter.wrapping_add(1);
    }
}
