//! Code for the EEProm module.
//!
//! Tested on a TM4C123 Tiva C Series Launchpad
//!
//! Note: This code manually increments the EEBLOCK and EEOFFSET registers
//! after each read and write instead of using the EERDWRINC register. The
//! debugger was giving inconsistent register results for the EEOFFSET register
//! after using EERDWRINC. Also, the EERDWRINC does not increment the block in
//! the case of a wrap of the offset, so it seems less useful for data that
//! spans blocks.
//!
//! This flexibility comes at the cost of efficiency, as the
//! datasheet calls for at least 4 cycles of delay after setting the EEBLOCK
//! register.

/// Possible errors for the Flash memory module
#[derive(Debug, PartialEq)]
pub enum EepromError {
    /// Eeprom is not finished
    Busy,
    /// Address is out of bounds
    AddressOutOfBounds,
    /// Block is out of bounds
    BlockOutOfBounds,
    /// Offset is out of bounds
    OffsetOutOfBounds,
    /// Indicates that writing data would exceed the EEPROM memory space
    WriteWouldOverflow,
    /// Indicates that reading data would exceed the EEPROM memory space
    ReadWouldOverflow,
    /// Requesting to read more data than the provided buffer can hold
    ReadBufferTooSmall,
}

impl core::fmt::Display for EepromError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EepromError::Busy => write!(f, "Eeprom is busy"),
            EepromError::AddressOutOfBounds => write!(f, "Address is out of bounds"),
            EepromError::BlockOutOfBounds => write!(f, "Block is out of bounds"),
            EepromError::OffsetOutOfBounds => write!(f, "Offset is out of bounds"),
            EepromError::WriteWouldOverflow => {
                write!(f, "Writing this data would overflow the EEPROM")
            }
            EepromError::ReadWouldOverflow => {
                write!(f, "Reading this data would overflow the EEPROM")
            }
            EepromError::ReadBufferTooSmall => write!(f, "Allocated buffer too small for reading"),
        }
    }
}

/// Struct used to pack the block and offset
#[derive(Clone, Copy)]
pub struct EepromAddress {
    /// Eeprom block
    block: usize,
    /// Eeprom offset in a block
    offset: usize,
}

impl EepromAddress {
    /// Creates a new EepromAddres with configured block and offset
    pub fn new(block: usize, offset: usize) -> Self {
        EepromAddress { block, offset }
    }

    /// Returns the block
    pub fn block(&self) -> usize {
        self.block
    }

    /// Returns the offset
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Increments the offset by one, if that would cause an overflow, increment the block. If
    /// both the block and offset wrap, the output for the new block and offset
    /// will both be 0.
    pub fn increment(&mut self, offset_size: usize, block_size: usize) -> &mut Self {
        self.offset += 1;
        if self.offset >= offset_size {
            self.offset = 0;
            self.block += 1;
            if self.block >= block_size {
                self.block = 0;
            }
        }

        self
    }
}

/// Series of traits to make access blocks easier
pub trait Blocks {
    /// Returns the blocksize for read / write to the flash
    fn block_size(&self) -> Result<usize, EepromError>;

    /// Returns the EepromAddress for a given index. Valid indexes are 0 to
    /// EEPROM_END_ADDRESS_WORDS.
    fn word_index_to_address(&self, index: usize) -> Result<EepromAddress, EepromError>;

    /// Gives the the word index (0 to EEPROM_END_ADDRESS_WORDS) for a
    /// given EepromAddress
    fn address_to_word_index(&self, block: &EepromAddress) -> Result<usize, EepromError>;
}

/// Erase functions of the EEPROM
pub trait Erase {
    /// Erase (zero out) data starting at an address spanning a length of bytes (not words!)
    fn erase(&mut self, address: &EepromAddress, length_bytes: usize) -> Result<(), EepromError>;

    /// Erase (zero out) a block
    fn erase_block(&mut self, block: usize) -> Result<(), EepromError>;
}

/// Check if the Eeprom is busy
pub trait Busy {
    /// Check the EEDONE register, true if busy
    fn is_busy(&self) -> bool;

    /// Blocks until the EEPROM is not busy
    fn wait(&self);
}

/// Write data to the EEPROM
pub trait Write {
    /// Write data to a flash address
    fn write(&mut self, address: &EepromAddress, data: &[u8]) -> Result<(), EepromError>;
}

/// Read data from the EEPROM
pub trait Read {
    /// Eeprom Address to start reading data from
    fn read(
        &mut self,
        address: &EepromAddress,
        bytes_to_read: usize,
        buffer: &mut [u8],
    ) -> Result<(), EepromError>;
}
