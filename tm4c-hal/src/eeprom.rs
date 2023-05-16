//! Code for the EEProm module.

/// Possible errors for the Flash memory module
#[derive(Debug, PartialEq)]
pub enum EepromError{
    /// Eeprom is not finished
    Busy,
    /// Address is out of bounds
    AddressOutOfBounds,
    /// Block is out of bounds
    BlockOutOfBounds,
    /// Indicates that writing data would exceed the EEPROM memory space
    WriteWouldOverflow,
    /// Indicates that reading data would exceed the EEPROM memory space
    ReadWouldOverflow,
    /// Requesting to read more data than the provided buffer can hold
    ReadBufferTooSmall,
    /// Access to EEPROM needs to be word aligned
    OffsetShouldBeWordAligned
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
}

/// Series of traits to make access blocks easier
pub trait Blocks {
    /// Returns the blocksize for read / write to the flash
    fn block_size(&self) -> Result<usize, EepromError> ;

    /// Returns the block for a given address. This should always be
    /// a power of 2 and rounded down to the nearest block.
    fn word_offset_to_address(&self, address: usize) -> Result<EepromAddress, EepromError>;

    /// Gives the starting address of a block
    fn address_to_word_offset(&self, block: &EepromAddress) -> Result<usize, EepromError>;
}

/// Erase functions of the EEPROM
pub trait Erase {
    /// Erase a block
    fn erase(&self, block: EepromAddress) -> Result<(), EepromError>;

    /// Mass erase the EEPROM
    fn mass_erase(&self) -> nb::Result<(), EepromError>;
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
    fn read(&mut self, address: &EepromAddress, bytes_to_read: usize, buffer: &mut [u8]) -> Result<(), EepromError>;
}





