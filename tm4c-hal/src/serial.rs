//! Serial code that is generic to both the TM4C123 and TM4C129, such as the pin traits.

/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

/// CTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait CtsPin<UART> {
    /// Enables the CTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DCD pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DcdPin<UART> {
    /// Enables the DCD functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DSR pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DsrPin<UART> {
    /// Enables the DSR functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// DTR pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait DtrPin<UART> {
    /// Enables the DTR functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// RI pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RiPin<UART> {
    /// Enables the RI functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

/// RTS pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RtsPin<UART> {
    /// Enables the RTS functionality if a valid pin is given (not `()`).
    fn enable(&mut self, _uart: &mut UART);
}

unsafe impl<U> TxPin<U> for () {}

unsafe impl<U> RxPin<U> for () {}

unsafe impl<U> CtsPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}
unsafe impl<U> DcdPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}
unsafe impl<U> DsrPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}
unsafe impl<U> DtrPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}
unsafe impl<U> RiPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}
unsafe impl<U> RtsPin<U> for () {
    fn enable(&mut self, _uart: &mut U) {
        // Do nothing
    }
}

/// writeln!() emits LF chars, so this is useful
/// if you're writing text with your UART
#[derive(PartialEq, Clone, Copy)]
pub enum NewlineMode {
    /// Emit octets as received
    Binary,
    /// Emit an extra CR before every LF
    SwapLFtoCRLF,
}
