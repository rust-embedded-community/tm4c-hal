//! Serial code that is generic to both the TM4C123 and TM4C129, such as the pin traits.

// The macro is required for the "sealed trait" pattern to work:
// the traits and the gpios have to be defined in the same crate

///! An internal macro to generate the UART traits
#[macro_export]
macro_rules! uart_traits_macro {
    () => {
        /// TX pin
        pub trait TxPin<UART>: crate::Sealed {}

        /// RX pin
        pub trait RxPin<UART>: crate::Sealed {}

        /// CTS pin
        pub trait CtsPin<UART>: crate::Sealed {
            /// Enables the CTS functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        /// DCD pin
        pub trait DcdPin<UART>: crate::Sealed {
            /// Enables the DCD functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        /// DSR pin
        pub trait DsrPin<UART>: crate::Sealed {
            /// Enables the DSR functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        /// DTR pin
        pub trait DtrPin<UART>: crate::Sealed {
            /// Enables the DTR functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        /// RI pin
        pub trait RiPin<UART>: crate::Sealed {
            /// Enables the RI functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        /// RTS pin
        pub trait RtsPin<UART>: crate::Sealed {
            /// Enables the RTS functionality if a valid pin is given (not `()`).
            fn enable(&mut self, _uart: &mut UART);
        }

        impl<U> TxPin<U> for () {}

        impl<U> RxPin<U> for () {}

        impl<U> CtsPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
        impl<U> DcdPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
        impl<U> DsrPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
        impl<U> DtrPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
        impl<U> RiPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
        impl<U> RtsPin<U> for () {
            fn enable(&mut self, _uart: &mut U) {
                // Do nothing
            }
        }
    };
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
