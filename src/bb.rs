//! Code to handle bit-banding.
//!
//! Bit-banding is where the SoC maps each 8-bit byte to 8 consecutive 32-bit
//! words. Writing a 1 to that word sets the matching bit. Writing a 0 clears
//! the matching bit. It means you can perform atomic bit set/clear; i.e.
//! without a read-modify-write.

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;

/// Sets/Clears a bit at the given address atomically, using the bit-banding
/// feature. We take a const pointer and mutate it, but that's because the
/// svd2rust crate will only give us const pointers.
pub unsafe fn change_bit<T>(address: *const T, bit: u8, value: bool) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    write_volatile(bit_word, if value { 0x01 } else { 0x00 });
}

/// Sets and then Clears a bit at the given address atomically, using the bit-
/// banding feature. We take a const pointer and mutate it, but that's because
/// the svd2rust crate will only give us const pointers.
pub unsafe fn toggle_bit<T>(address: *const T, bit: u8) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    write_volatile(bit_word, 0x01);
    write_volatile(bit_word, 0x00);
}

/// Spins while reading a bit at the given address atomically, using the bit-
/// banding feature. We take a const pointer and mutate it, but that's because
/// the svd2rust crate will only give us const pointers.
pub fn spin_bit<T>(address: *const T, bit: u8) {
    while !read_bit(address, bit) {
        nop();
    }
}

/// Reads a bit at the given address atomically, using the bit-banding
/// feature.
pub fn read_bit<T>(address: *const T, bit: u8) -> bool {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    unsafe { read_volatile(bit_word) != 0 }
}

/// Address must be >= 0x2000_0000 and <= 0x2007_FFFC. Bit must be < 32.
fn ref_to_bitband(address: u32, bit: u8) -> *mut u32 {
    let prefix = address & 0xF000_0000;
    let byte_offset = address & 0x0FFF_FFFF;
    let bit_word_offset = (byte_offset * 32) + (bit as u32 * 4);
    let bit_word_addr = bit_word_offset + prefix + 0x0200_0000;
    bit_word_addr as *mut u32
}
