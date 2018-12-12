#![no_std]

/// Example of a function that both `tm4c123x-hal` and `tm4c129x-hal` would
/// need.
pub fn some_common_function(x: u32) -> u32 {
	x.wrapping_add(1)
}
