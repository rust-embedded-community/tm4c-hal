//! Code for the EEProm module.

use core::convert::TryInto;

use crate::sysctl::{self};
use cortex_m::asm::delay;
use tm4c123x::EEPROM;
pub use tm4c_hal::eeprom::{Blocks, Busy, EepromAddress, EepromError, Erase, Read, Write};

// Number of EEPROM block on the TM4C123
const EEPROM_BLOCK_SIZE: usize = 16;

// Number of EEPROM blocks on the TM4C123
const EEPROM_NUM_BLOCKS: usize = 32;

// Total number of bytes in the EEPROM on the TM4C123
const EEPROM_END_ADDRESS_BYTES: usize = 2048;

// Total number of words in the EEPROM on the TM4C123
const EEPROM_END_ADDRESS_WORDS: usize = 512;

// Size of the EEPROM word in bytes
const BYTES_PER_WORD: usize = 4;

/// Eeprom struct
pub struct Eeprom {
    /// Eeprom registers
    eeprom: EEPROM,
}

impl Eeprom {
    /// Configures a new EEPROM struct using the datasheet section 8.2.4.2
    pub fn new(eeprom: EEPROM, pc: &sysctl::PowerControl) -> Self {
        let final_eeprom = Eeprom { eeprom };

        // See Section 8.2.4.2 EEPROM Initialization and Configuration
        // in the datasheet:

        // 0. Power on the EEPROM peripheral
        sysctl::control_power(
            pc,
            sysctl::Domain::Eeprom,
            tm4c_hal::sysctl::RunMode::Run,
            tm4c_hal::sysctl::PowerState::On,
        );

        // 1. The datasheet calls for at least a 6 cycle delay before polling
        // the working register. Need to make sure the loop isn't optimized
        // out.
        delay(20);

        // 2. Poll busy
        final_eeprom.wait();

        // 3. Read PRETRY and ERETRY
        // Note: If either bit is set, the data sheet indicates this is a pretty severe
        // error with the EEPROM and the EEPROM shouldn't be used, which is why
        // a panic!() is chosen over an error. There could be issues with the
        // chip, core voltage, or EEPROM; regardless, it probably should be
        // investigated further.
        // See section 8.2.4.2
        if final_eeprom.eeprom.eesupp.read().eretry().bit_is_set() {
            panic!("Eeprom ERETRY bit set, please investigate or stop using the EEPROM peripheral");
        }

        if final_eeprom.eeprom.eesupp.read().pretry().bit_is_set() {
            panic!("Eeprom PRETRY bit set, please investigate or stop using the EEPROM peripheral");
        }

        // 4. Software reset
        sysctl::reset(pc, sysctl::Domain::Eeprom);

        // 5. Another delay
        delay(20);

        // 6. Poll busy
        final_eeprom.wait();

        // 7. Recheck PRETRY and ERETRY
        // Note: If either bit is set, the data sheet indicates this is a pretty severe
        // error with the EEPROM and the EEPROM shouldn't be used, which is why
        // a panic!() is chosen over an error. There could be issues with the
        // chip, core voltage, or EEPROM; regardless, it probably should be
        // investigated further.
        // See section 8.2.4.2
        if final_eeprom.eeprom.eesupp.read().eretry().bit_is_set() {
            panic!("Eeprom ERETRY bit set, please investigate or stop using the EEPROM peripheral");
        }

        if final_eeprom.eeprom.eesupp.read().pretry().bit_is_set() {
            panic!("Eeprom PRETRY bit set, please investigate or stop using the EEPROM peripheral");
        }

        // 8. All done
        final_eeprom
    }

    /// Set the block register
    fn set_block(&self, block: usize) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if block < EEPROM_NUM_BLOCKS {
            unsafe {
                self.eeprom.eeblock.write(|w| w.bits(block as u32));
            }

            // Changing blocks requires a small delay, see Section 8.2.4.1 Timing Considerations
            delay(4);

            self.wait();

            Ok(())
        } else {
            Err(EepromError::BlockOutOfBounds)
        }
    }

    /// Set the offset register
    fn set_offset(&self, offset: usize) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if offset < EEPROM_BLOCK_SIZE {
            unsafe {
                self.eeprom.eeoffset.write(|w| w.bits(offset as u32));
            }

            self.wait();

            Ok(())
        } else {
            Err(EepromError::OffsetOutOfBounds)
        }
    }

    /// Set the block and offset registers
    fn set_block_and_offset(&self, address: &EepromAddress) -> Result<(), EepromError> {
        self.wait();
        self.set_block(address.block())?;
        self.set_offset(address.offset())?;
        Ok(())
    }

    /// Checks if read / writing a certain number of bytes from an address is
    /// valid. Returns true if EEPROM access is valid, false if there
    /// are any issues (overflow or invalid address).
    fn is_access_valid(&self, address: &EepromAddress, length_bytes: usize) -> bool {
        // Check if the initial address is valid, then check byte length
        match self.address_to_word_index(&address) {
            Ok(start_word_address) => {
                return start_word_address * BYTES_PER_WORD + length_bytes
                    < EEPROM_END_ADDRESS_BYTES;
            }
            Err(_) => {
                return false;
            }
        }
    }

    /// Increments the block and offset by 1 word. Will wrap both the offset and
    /// block to 0 if an increment would cause either to exceed their bounds.
    ///
    /// For example:
    /// * Block 0, Offset, 1 -> Block 0, Offset 2
    /// * Block 0, Offset, 15 -> Block 1, Offset 0
    /// * Block 31, Offset, 15 -> Block 0, Offset 0
    fn increment_offset(
        &mut self,
        starting_address: &mut EepromAddress,
    ) -> Result<(), EepromError> {
        starting_address.increment(EEPROM_BLOCK_SIZE, EEPROM_NUM_BLOCKS);
        self.set_block_and_offset(&starting_address)?;
        Ok(())
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
    fn block_size(&self) -> Result<usize, EepromError> {
        Ok(EEPROM_BLOCK_SIZE)
    }

    fn word_index_to_address(&self, word_address: usize) -> Result<EepromAddress, EepromError> {
        if word_address > EEPROM_END_ADDRESS_WORDS {
            return Err(EepromError::AddressOutOfBounds);
        } else {
            let block = word_address / EEPROM_BLOCK_SIZE;
            let offset = word_address - (block * EEPROM_BLOCK_SIZE);
            Ok(EepromAddress::new(block, offset))
        }
    }

    fn address_to_word_index(&self, block: &EepromAddress) -> Result<usize, EepromError> {
        if block.block() > EEPROM_NUM_BLOCKS || block.offset() > EEPROM_BLOCK_SIZE {
            return Err(EepromError::BlockOutOfBounds);
        } else {
            return Ok(block.block() * EEPROM_BLOCK_SIZE + block.offset());
        }
    }
}

impl Write for Eeprom {
    fn write(&mut self, address: &EepromAddress, data: &[u8]) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        // Check if the address is valid and if the data will fit
        if !self.is_access_valid(address, data.len()) {
            return Err(EepromError::WriteWouldOverflow);
        }

        self.set_block_and_offset(address)?;

        let chunk_iter = data.chunks_exact(4);
        let leftover_bytes = chunk_iter.remainder();
        let mut address_copy = *address;

        for chunk in chunk_iter {
            let tmp = u32::from_le_bytes(chunk.try_into().unwrap());

            self.wait();

            unsafe {
                self.eeprom.eerdwr.write(|w| w.bits(tmp));
            }

            self.increment_offset(&mut address_copy)?;
        }

        // Buffer the leftover bytes, if any, and write
        if leftover_bytes.len() != 0 {
            let mut buffer = [0 as u8; 4];
            for (i, byte) in leftover_bytes.iter().enumerate() {
                buffer[i] = *byte;
            }

            self.wait();

            unsafe {
                self.eeprom
                    .eerdwr
                    .write(|w| w.bits(u32::from_le_bytes(buffer)));
            }
        }

        self.wait();

        Ok(())
    }
}

impl Read for Eeprom {
    fn read(
        &mut self,
        address: &EepromAddress,
        bytes_to_read: usize,
        buffer: &mut [u8],
    ) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if bytes_to_read > buffer.len() {
            return Err(EepromError::ReadBufferTooSmall);
        }

        if !self.is_access_valid(&address, bytes_to_read) {
            return Err(EepromError::ReadWouldOverflow);
        }

        let num_words = bytes_to_read / BYTES_PER_WORD;
        let leftover_bytes = bytes_to_read % BYTES_PER_WORD;
        let mut address_copy = *address;

        self.set_block_and_offset(&address)?;

        let mut byte_offset = 0;

        for _i in 0..num_words {
            self.wait();

            let word_as_bytes = self.eeprom.eerdwr.read().bits().to_le_bytes();

            self.increment_offset(&mut address_copy)?;

            for byte in word_as_bytes {
                buffer[byte_offset] = byte;
                byte_offset += 1;
            }
        }

        if leftover_bytes != 0 {
            self.wait();

            let word_as_bytes = self.eeprom.eerdwr.read().bits().to_le_bytes();

            self.increment_offset(&mut address_copy)?;

            for index in 0..leftover_bytes {
                buffer[byte_offset] = word_as_bytes[index];
                byte_offset += 1;
            }
        }

        self.wait();

        Ok(())
    }
}

impl Erase for Eeprom {
    fn erase(&mut self, address: &EepromAddress, length_bytes: usize) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        if !self.is_access_valid(address, length_bytes) {
            return Err(EepromError::WriteWouldOverflow);
        }

        let num_words = length_bytes / BYTES_PER_WORD;
        let leftover_bytes = length_bytes % BYTES_PER_WORD;
        let mut address_copy = *address;

        self.set_block_and_offset(&address)?;

        let zero = 0 as u32;
        for _i in 0..num_words {
            self.wait();

            unsafe {
                self.eeprom.eerdwr.write(|w| w.bits(zero));
            }

            self.increment_offset(&mut address_copy)?;
        }

        // Special case here, need to read-modify-write
        if leftover_bytes != 0 {
            self.wait();

            let mut word = self.eeprom.eerdwr.read().bits().to_le_bytes();

            for i in 0..leftover_bytes {
                word[i] = 0;
            }

            unsafe {
                self.eeprom
                    .eerdwr
                    .write(|w| w.bits(u32::from_le_bytes(word)));
            }
        }

        self.wait();

        Ok(())
    }

    fn erase_block(&mut self, block: usize) -> Result<(), EepromError> {
        if self.is_busy() {
            return Err(EepromError::Busy);
        }

        self.set_block(block)?;

        let mut address = EepromAddress::new(block, 0);

        let zeros = [0 as u8; EEPROM_BLOCK_SIZE * BYTES_PER_WORD];

        self.write(&mut address, &zeros)?;

        self.wait();

        Ok(())
    }
}
