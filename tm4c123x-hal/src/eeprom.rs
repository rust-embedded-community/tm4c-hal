//! Code for the EEProm module.

use core::{convert::TryInto};

use tm4c123x::{EEPROM};
use crate::sysctl::{self};
pub use tm4c_hal::eeprom::{Blocks, Busy, Write, Read, EepromError, EepromAddress};

// Number of EEPROM block on the TM4C123
const EEPROM_BLOCK_SIZE : usize = 16;

// Number of EEPROM blocks on the TM4C123
const EEPROM_NUM_BLOCKS : usize = 32;

// Total number of bytes in the EEPROM on the TM4C123
const EEPROM_END_ADDRESS_BYTES : usize = 2048;

// Total number of words in the EEPROM on the TM4C123
const EEPROM_END_ADDRESS_WORDS : usize = 512;

// Size of the EEPROM word in bytes
const BYTES_PER_WORD : usize = 4;

/// Eeprom struct
pub struct Eeprom {
    /// Eeprom registers
    eeprom: EEPROM,
}

impl Eeprom {
    /// Configures a new EEPROM with a start / end address defined by the 
    /// user.
    pub fn new(eeprom: EEPROM, _pc: &sysctl::PowerControl) -> Self {
        
        let final_eeprom = Eeprom { eeprom };
        
        sysctl::control_power(
            _pc, 
            sysctl::Domain::Eeprom, 
            tm4c_hal::sysctl::RunMode::Run,
            tm4c_hal::sysctl::PowerState::On);
        sysctl::reset(_pc, sysctl::Domain::Eeprom);

        final_eeprom.wait();

        final_eeprom
    }

    /// Set the block register
    fn set_block(&self, block: usize) {
        unsafe {
            self.eeprom.eeblock.write(|w| {
                w.bits(block as u32)
            });

            self.wait();
        }
    }

    /// Set the offset register
    fn set_offset(&self, offset: usize) {
        unsafe {
            self.eeprom.eeoffset.write(|w| {
                w.bits(offset as u32)
            });

            self.wait();
        }
    }

    /// Set the block and offset registers
    fn set_block_and_offset(&self, address: &EepromAddress) {
        self.set_block(address.block());
        self.set_offset(address.offset());
    }

    /// Checks if read / writing a certain number of bytes from an address is
    /// valid.
    fn validate_byte_array_bounds(&self, address: &EepromAddress, length_bytes: usize) -> bool {
        // Check if the initial address is valid, then check byte length
        match self.address_to_word_offset(&address) {
            Ok(start_word_address) => {
                return start_word_address*BYTES_PER_WORD + length_bytes < EEPROM_END_ADDRESS_BYTES;
            }
            Err(_) => {
                return false;
            }
        }
    }
}

impl Busy for Eeprom {
    fn is_busy(&self) -> bool {
        self.eeprom.eedone.read().working().bit_is_set()
    }

    fn wait(&self) {
        while self.is_busy() == true {}
    }
}

impl Blocks for Eeprom {
    fn block_size(&self) -> Result<usize, EepromError>  {
        Ok(EEPROM_BLOCK_SIZE)
    }

    fn word_offset_to_address(&self, word_address: usize) -> Result<EepromAddress, EepromError> {
        if word_address > EEPROM_END_ADDRESS_WORDS {
            return Err(EepromError::AddressOutOfBounds);
        }else{
            let block = word_address / EEPROM_BLOCK_SIZE;
            let offset = word_address - (block * EEPROM_BLOCK_SIZE);
            Ok(EepromAddress::new(block, offset))
        }
    }

    fn address_to_word_offset(&self, block: &EepromAddress) -> Result<usize, EepromError> {
        if block.block() > EEPROM_NUM_BLOCKS || block.offset() > EEPROM_BLOCK_SIZE {
            return Err(EepromError::BlockOutOfBounds);
        }else{
            return Ok(block.block() * EEPROM_BLOCK_SIZE + block.offset());
        }
    }
}

impl Write for Eeprom {
    fn write(&mut self, address: &EepromAddress, data: &[u8]) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if address.offset() % BYTES_PER_WORD != 0 {
            return Err(EepromError::OffsetShouldBeWordAligned);
        }

        // Check if the address is valid and if the data will fit
        if self.validate_byte_array_bounds(address, data.len()) {
            self.set_block_and_offset(address);
            
            let chunk_iter = data.chunks_exact(4);
            let leftover_bytes = chunk_iter.remainder();

            // Write the easy part using the auto increment register
            for chunk in chunk_iter {
                let tmp = u32::from_le_bytes(chunk.try_into().unwrap());
                
                self.wait();
                
                unsafe {
                    self.eeprom.eerdwrinc.write(|w| {
                        w.bits(tmp)
                    });
                }
            }

            // Buffer the leftover bytes, if any, and write
            if leftover_bytes.len() != 0 {
                let mut buffer = [0 as u8; 4];
                for (i, byte) in leftover_bytes.iter().enumerate() {
                    buffer[i] = *byte;
                }

                self.wait();

                unsafe {
                    self.eeprom.eerdwrinc.write(|w| {
                        w.bits(u32::from_le_bytes(buffer))
                    });
                }
            }

            self.wait();

            Ok(())
        }else{
            Err(EepromError::WriteWouldOverflow)
        }
    }
}

impl Read for Eeprom {
    fn read(&mut self, address: &EepromAddress, bytes_to_read: usize, buffer: &mut [u8]) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if bytes_to_read > buffer.len() {
            return Err(EepromError::ReadBufferTooSmall);
        }

        if address.offset() % BYTES_PER_WORD != 0 {
            return Err(EepromError::OffsetShouldBeWordAligned);
        }
        
        if self.validate_byte_array_bounds(&address, bytes_to_read) {
            let num_words = bytes_to_read / BYTES_PER_WORD;
            let leftover_bytes = bytes_to_read % BYTES_PER_WORD;

            self.set_block_and_offset(&address);

            let mut byte_offset = 0;

            for _i in 0..num_words {
                self.wait();

                let word_as_bytes = self.eeprom.eerdwrinc.read().bits().to_le_bytes();

                for byte in  word_as_bytes {
                    buffer[byte_offset ] = byte;
                    byte_offset += 1;
                }
            }

            if leftover_bytes != 0 {
                self.wait();

                let word_as_bytes = self.eeprom.eerdwrinc.read().bits().to_le_bytes();

                for index in  0..leftover_bytes {
                    buffer[byte_offset] = word_as_bytes[index];
                    byte_offset += 1;
                }
            }

            Ok(())
        }else{
            Err(EepromError::ReadWouldOverflow)
        }

    }
}
