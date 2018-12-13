#![doc = "Peripheral access API for EDES microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
extern crate bare_metal;
extern crate vcell;
#[doc = "Ethernet Descriptor"]
pub mod tdes {
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct TDES {
        #[doc = "0x00 - Desc"]
        pub tdes0: TDES0,
        #[doc = "0x04 - Desc"]
        pub tdes1: TDES1,
        #[doc = "0x08 - Desc"]
        pub tdes2: TDES2,
        #[doc = "0x0c - Desc"]
        pub tdes3: TDES3,
        #[doc = "0x10 - Desc"]
        pub tdes4: TDES4,
        #[doc = "0x14 - Desc"]
        pub tdes5: TDES5,
        #[doc = "0x18 - Desc"]
        pub tdes6: TDES6,
        #[doc = "0x1c - Desc"]
        pub tdes7: TDES7,
    }
    unsafe impl Sync for TDES {}
    impl TDES {
        pub const fn new() -> TDES {
            TDES {
                tdes0: TDES0 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes1: TDES1 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes2: TDES2 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes3: TDES3 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes4: TDES4 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes5: TDES5 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes6: TDES6 {
                    register: vcell::VolatileCell::new(0),
                },
                tdes7: TDES7 {
                    register: vcell::VolatileCell::new(0),
                },
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES0 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes0 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES0 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct OWNR {
            bits: bool,
        }
        impl OWNR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct ICR {
            bits: bool,
        }
        impl ICR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct LSR {
            bits: bool,
        }
        impl LSR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct FSR {
            bits: bool,
        }
        impl FSR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct DCR {
            bits: bool,
        }
        impl DCR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct DPR {
            bits: bool,
        }
        impl DPR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSER {
            bits: bool,
        }
        impl TTSER {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct CRCRR {
            bits: bool,
        }
        impl CRCRR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct CICR {
            bits: u8,
        }
        impl CICR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct TERR {
            bits: bool,
        }
        impl TERR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct TCHR {
            bits: bool,
        }
        impl TCHR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct VLICR {
            bits: u8,
        }
        impl VLICR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSSR {
            bits: bool,
        }
        impl TTSSR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct IHER {
            bits: bool,
        }
        impl IHER {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct ESR {
            bits: bool,
        }
        impl ESR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct JTR {
            bits: bool,
        }
        impl JTR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct FFR {
            bits: bool,
        }
        impl FFR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct IPER {
            bits: bool,
        }
        impl IPER {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct LCARR {
            bits: bool,
        }
        impl LCARR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct NCR {
            bits: bool,
        }
        impl NCR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct LCIONR {
            bits: bool,
        }
        impl LCIONR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct XCR {
            bits: bool,
        }
        impl XCR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct VFR {
            bits: bool,
        }
        impl VFR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct CCR {
            bits: u8,
        }
        impl CCR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct EDR {
            bits: bool,
        }
        impl EDR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct UFR {
            bits: bool,
        }
        impl UFR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct DBR {
            bits: bool,
        }
        impl DBR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Proxy"]
        pub struct _OWNW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OWNW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _ICW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ICW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _LSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LSW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _FSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _FSW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 28;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _DCW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DCW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _DPW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DPW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSEW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _CRCRW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CRCRW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _CICW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CICW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 0x03;
                const OFFSET: u8 = 22;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TERW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TERW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TCHW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TCHW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _VLICW<'a> {
            w: &'a mut W,
        }
        impl<'a> _VLICW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 0x03;
                const OFFSET: u8 = 18;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSSW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _IHEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _IHEW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _ESW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ESW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _JTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _JTW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _FFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _FFW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _IPEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _IPEW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _LCARW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LCARW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _NCW<'a> {
            w: &'a mut W,
        }
        impl<'a> _NCW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _LCIONW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LCIONW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _XCW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XCW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _VFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _VFW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 0x0f;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _EDW<'a> {
            w: &'a mut W,
        }
        impl<'a> _EDW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _UFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _UFW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _DBW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DBW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn own(&self) -> OWNR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 31;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                OWNR { bits }
            }
            #[doc = "Bit 30"]
            #[inline]
            pub fn ic(&self) -> ICR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 30;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                ICR { bits }
            }
            #[doc = "Bit 29"]
            #[inline]
            pub fn ls(&self) -> LSR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 29;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                LSR { bits }
            }
            #[doc = "Bit 28"]
            #[inline]
            pub fn fs(&self) -> FSR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 28;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                FSR { bits }
            }
            #[doc = "Bit 27"]
            #[inline]
            pub fn dc(&self) -> DCR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 27;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                DCR { bits }
            }
            #[doc = "Bit 26"]
            #[inline]
            pub fn dp(&self) -> DPR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 26;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                DPR { bits }
            }
            #[doc = "Bit 25"]
            #[inline]
            pub fn ttse(&self) -> TTSER {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 25;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                TTSER { bits }
            }
            #[doc = "Bit 24"]
            #[inline]
            pub fn crcr(&self) -> CRCRR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 24;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                CRCRR { bits }
            }
            #[doc = "Bits 22:23"]
            #[inline]
            pub fn cic(&self) -> CICR {
                let bits = {
                    const MASK: u8 = 0x03;
                    const OFFSET: u8 = 22;
                    ((self.bits >> OFFSET) & MASK as u32) as u8
                };
                CICR { bits }
            }
            #[doc = "Bit 21"]
            #[inline]
            pub fn ter(&self) -> TERR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 21;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                TERR { bits }
            }
            #[doc = "Bit 20"]
            #[inline]
            pub fn tch(&self) -> TCHR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 20;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                TCHR { bits }
            }
            #[doc = "Bits 18:19"]
            #[inline]
            pub fn vlic(&self) -> VLICR {
                let bits = {
                    const MASK: u8 = 0x03;
                    const OFFSET: u8 = 18;
                    ((self.bits >> OFFSET) & MASK as u32) as u8
                };
                VLICR { bits }
            }
            #[doc = "Bit 17"]
            #[inline]
            pub fn ttss(&self) -> TTSSR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 17;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                TTSSR { bits }
            }
            #[doc = "Bit 16"]
            #[inline]
            pub fn ihe(&self) -> IHER {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 16;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                IHER { bits }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn es(&self) -> ESR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 15;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                ESR { bits }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn jt(&self) -> JTR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                JTR { bits }
            }
            #[doc = "Bit 13"]
            #[inline]
            pub fn ff(&self) -> FFR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 13;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                FFR { bits }
            }
            #[doc = "Bit 12"]
            #[inline]
            pub fn ipe(&self) -> IPER {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 12;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                IPER { bits }
            }
            #[doc = "Bit 11"]
            #[inline]
            pub fn lcar(&self) -> LCARR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 11;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                LCARR { bits }
            }
            #[doc = "Bit 10"]
            #[inline]
            pub fn nc(&self) -> NCR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 10;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                NCR { bits }
            }
            #[doc = "Bit 9"]
            #[inline]
            pub fn lcion(&self) -> LCIONR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 9;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                LCIONR { bits }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn xc(&self) -> XCR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                XCR { bits }
            }
            #[doc = "Bit 7"]
            #[inline]
            pub fn vf(&self) -> VFR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                VFR { bits }
            }
            #[doc = "Bits 3:6"]
            #[inline]
            pub fn cc(&self) -> CCR {
                let bits = {
                    const MASK: u8 = 0x0f;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u32) as u8
                };
                CCR { bits }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn ed(&self) -> EDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                EDR { bits }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn uf(&self) -> UFR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                UFR { bits }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn db(&self) -> DBR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                DBR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn own(&mut self) -> _OWNW {
                _OWNW { w: self }
            }
            #[doc = "Bit 30"]
            #[inline]
            pub fn ic(&mut self) -> _ICW {
                _ICW { w: self }
            }
            #[doc = "Bit 29"]
            #[inline]
            pub fn ls(&mut self) -> _LSW {
                _LSW { w: self }
            }
            #[doc = "Bit 28"]
            #[inline]
            pub fn fs(&mut self) -> _FSW {
                _FSW { w: self }
            }
            #[doc = "Bit 27"]
            #[inline]
            pub fn dc(&mut self) -> _DCW {
                _DCW { w: self }
            }
            #[doc = "Bit 26"]
            #[inline]
            pub fn dp(&mut self) -> _DPW {
                _DPW { w: self }
            }
            #[doc = "Bit 25"]
            #[inline]
            pub fn ttse(&mut self) -> _TTSEW {
                _TTSEW { w: self }
            }
            #[doc = "Bit 24"]
            #[inline]
            pub fn crcr(&mut self) -> _CRCRW {
                _CRCRW { w: self }
            }
            #[doc = "Bits 22:23"]
            #[inline]
            pub fn cic(&mut self) -> _CICW {
                _CICW { w: self }
            }
            #[doc = "Bit 21"]
            #[inline]
            pub fn ter(&mut self) -> _TERW {
                _TERW { w: self }
            }
            #[doc = "Bit 20"]
            #[inline]
            pub fn tch(&mut self) -> _TCHW {
                _TCHW { w: self }
            }
            #[doc = "Bits 18:19"]
            #[inline]
            pub fn vlic(&mut self) -> _VLICW {
                _VLICW { w: self }
            }
            #[doc = "Bit 17"]
            #[inline]
            pub fn ttss(&mut self) -> _TTSSW {
                _TTSSW { w: self }
            }
            #[doc = "Bit 16"]
            #[inline]
            pub fn ihe(&mut self) -> _IHEW {
                _IHEW { w: self }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn es(&mut self) -> _ESW {
                _ESW { w: self }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn jt(&mut self) -> _JTW {
                _JTW { w: self }
            }
            #[doc = "Bit 13"]
            #[inline]
            pub fn ff(&mut self) -> _FFW {
                _FFW { w: self }
            }
            #[doc = "Bit 12"]
            #[inline]
            pub fn ipe(&mut self) -> _IPEW {
                _IPEW { w: self }
            }
            #[doc = "Bit 11"]
            #[inline]
            pub fn lcar(&mut self) -> _LCARW {
                _LCARW { w: self }
            }
            #[doc = "Bit 10"]
            #[inline]
            pub fn nc(&mut self) -> _NCW {
                _NCW { w: self }
            }
            #[doc = "Bit 9"]
            #[inline]
            pub fn lcion(&mut self) -> _LCIONW {
                _LCIONW { w: self }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn xc(&mut self) -> _XCW {
                _XCW { w: self }
            }
            #[doc = "Bit 7"]
            #[inline]
            pub fn vf(&mut self) -> _VFW {
                _VFW { w: self }
            }
            #[doc = "Bits 3:6"]
            #[inline]
            pub fn cc(&mut self) -> _CCW {
                _CCW { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn ed(&mut self) -> _EDW {
                _EDW { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn uf(&mut self) -> _UFW {
                _UFW { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn db(&mut self) -> _DBW {
                _DBW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES1 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct SAIC_HIR {
            bits: bool,
        }
        impl SAIC_HIR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct SAIC_LOR {
            bits: u8,
        }
        impl SAIC_LOR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct TBS2R {
            bits: u16,
        }
        impl TBS2R {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct RERR {
            bits: bool,
        }
        impl RERR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct RCHR {
            bits: bool,
        }
        impl RCHR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct TBS1R {
            bits: u16,
        }
        impl TBS1R {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _SAIC_HIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SAIC_HIW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _SAIC_LOW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SAIC_LOW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 0x03;
                const OFFSET: u8 = 29;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TBS2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _TBS2W<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u16) -> &'a mut W {
                const MASK: u16 = 0x1fff;
                const OFFSET: u8 = 16;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _RERW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RERW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _RCHW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RCHW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TBS1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _TBS1W<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u16) -> &'a mut W {
                const MASK: u16 = 0x1fff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn saic_hi(&self) -> SAIC_HIR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 31;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                SAIC_HIR { bits }
            }
            #[doc = "Bits 29:30"]
            #[inline]
            pub fn saic_lo(&self) -> SAIC_LOR {
                let bits = {
                    const MASK: u8 = 0x03;
                    const OFFSET: u8 = 29;
                    ((self.bits >> OFFSET) & MASK as u32) as u8
                };
                SAIC_LOR { bits }
            }
            #[doc = "Bits 16:28"]
            #[inline]
            pub fn tbs2(&self) -> TBS2R {
                let bits = {
                    const MASK: u16 = 0x1fff;
                    const OFFSET: u8 = 16;
                    ((self.bits >> OFFSET) & MASK as u32) as u16
                };
                TBS2R { bits }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn rer(&self) -> RERR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 15;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                RERR { bits }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn rch(&self) -> RCHR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                RCHR { bits }
            }
            #[doc = "Bits 0:12"]
            #[inline]
            pub fn tbs1(&self) -> TBS1R {
                let bits = {
                    const MASK: u16 = 0x1fff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u16
                };
                TBS1R { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn saic_hi(&mut self) -> _SAIC_HIW {
                _SAIC_HIW { w: self }
            }
            #[doc = "Bits 29:30"]
            #[inline]
            pub fn saic_lo(&mut self) -> _SAIC_LOW {
                _SAIC_LOW { w: self }
            }
            #[doc = "Bits 16:28"]
            #[inline]
            pub fn tbs2(&mut self) -> _TBS2W {
                _TBS2W { w: self }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn rer(&mut self) -> _RERW {
                _RERW { w: self }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn rch(&mut self) -> _RCHW {
                _RCHW { w: self }
            }
            #[doc = "Bits 0:12"]
            #[inline]
            pub fn tbs1(&mut self) -> _TBS1W {
                _TBS1W { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES2 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes2 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES2 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct BUF1PTRR {
            bits: u32,
        }
        impl BUF1PTRR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _BUF1PTRW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BUF1PTRW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf1ptr(&self) -> BUF1PTRR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                BUF1PTRR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf1ptr(&mut self) -> _BUF1PTRW {
                _BUF1PTRW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES3 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes3 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES3 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct BUF2PTR_NDAR {
            bits: u32,
        }
        impl BUF2PTR_NDAR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _BUF2PTR_NDAW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BUF2PTR_NDAW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf2ptr_nda(&self) -> BUF2PTR_NDAR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                BUF2PTR_NDAR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf2ptr_nda(&mut self) -> _BUF2PTR_NDAW {
                _BUF2PTR_NDAW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES4 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes4 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES4 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES5 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes5 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES5 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES6 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes6 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES6 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSLR {
            bits: u32,
        }
        impl TTSLR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSLW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSLW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsl(&self) -> TTSLR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                TTSLR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsl(&mut self) -> _TTSLW {
                _TTSLW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct TDES7 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod tdes7 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::TDES7 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSHR {
            bits: u32,
        }
        impl TTSHR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSHW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSHW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsh(&self) -> TTSHR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                TTSHR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsh(&mut self) -> _TTSHW {
                _TTSHW { w: self }
            }
        }
    }
}
#[doc = "Ethernet Descriptor"]
pub mod rdes {
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RDES {
        #[doc = "0x00 - Desc"]
        pub rdes0: RDES0,
        #[doc = "0x04 - Desc"]
        pub rdes1: RDES1,
        #[doc = "0x08 - Desc"]
        pub rdes2: RDES2,
        #[doc = "0x0c - Desc"]
        pub rdes3: RDES3,
        #[doc = "0x10 - Desc"]
        pub rdes4: RDES4,
        #[doc = "0x14 - Desc"]
        pub rdes5: RDES5,
        #[doc = "0x18 - Desc"]
        pub rdes6: RDES6,
        #[doc = "0x1c - Desc"]
        pub rdes7: RDES7,
    }
    unsafe impl Sync for RDES {}
    impl RDES {
        pub const fn new() -> RDES {
            RDES {
                rdes0: RDES0 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes1: RDES1 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes2: RDES2 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes3: RDES3 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes4: RDES4 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes5: RDES5 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes6: RDES6 {
                    register: vcell::VolatileCell::new(0),
                },
                rdes7: RDES7 {
                    register: vcell::VolatileCell::new(0),
                },
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES0 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes0 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES0 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct OWNR {
            bits: bool,
        }
        impl OWNR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct AFMR {
            bits: bool,
        }
        impl AFMR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct FLR {
            bits: u16,
        }
        impl FLR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct ESR {
            bits: bool,
        }
        impl ESR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct LSR {
            bits: bool,
        }
        impl LSR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Proxy"]
        pub struct _OWNW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OWNW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _AFMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _AFMW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _FLW<'a> {
            w: &'a mut W,
        }
        impl<'a> _FLW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u16) -> &'a mut W {
                const MASK: u16 = 0x3fff;
                const OFFSET: u8 = 16;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _ESW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ESW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _LSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LSW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn own(&self) -> OWNR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 31;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                OWNR { bits }
            }
            #[doc = "Bit 30"]
            #[inline]
            pub fn afm(&self) -> AFMR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 30;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                AFMR { bits }
            }
            #[doc = "Bits 16:29"]
            #[inline]
            pub fn fl(&self) -> FLR {
                let bits = {
                    const MASK: u16 = 0x3fff;
                    const OFFSET: u8 = 16;
                    ((self.bits >> OFFSET) & MASK as u32) as u16
                };
                FLR { bits }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn es(&self) -> ESR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 15;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                ESR { bits }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn ls(&self) -> LSR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                LSR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn own(&mut self) -> _OWNW {
                _OWNW { w: self }
            }
            #[doc = "Bit 30"]
            #[inline]
            pub fn afm(&mut self) -> _AFMW {
                _AFMW { w: self }
            }
            #[doc = "Bits 16:29"]
            #[inline]
            pub fn fl(&mut self) -> _FLW {
                _FLW { w: self }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn es(&mut self) -> _ESW {
                _ESW { w: self }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn ls(&mut self) -> _LSW {
                _LSW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES1 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct SAIC_HIR {
            bits: bool,
        }
        impl SAIC_HIR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct SAIC_LOR {
            bits: u8,
        }
        impl SAIC_LOR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct TBS2R {
            bits: u16,
        }
        impl TBS2R {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        #[doc = r" Value of the field"]
        pub struct RERR {
            bits: bool,
        }
        impl RERR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct RCHR {
            bits: bool,
        }
        impl RCHR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits
            }
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = r" Value of the field"]
        pub struct TBS1R {
            bits: u16,
        }
        impl TBS1R {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _SAIC_HIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SAIC_HIW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _SAIC_LOW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SAIC_LOW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 0x03;
                const OFFSET: u8 = 29;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TBS2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _TBS2W<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u16) -> &'a mut W {
                const MASK: u16 = 0x1fff;
                const OFFSET: u8 = 16;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _RERW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RERW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _RCHW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RCHW<'a> {
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _TBS1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _TBS1W<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u16) -> &'a mut W {
                const MASK: u16 = 0x1fff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn saic_hi(&self) -> SAIC_HIR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 31;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                SAIC_HIR { bits }
            }
            #[doc = "Bits 29:30"]
            #[inline]
            pub fn saic_lo(&self) -> SAIC_LOR {
                let bits = {
                    const MASK: u8 = 0x03;
                    const OFFSET: u8 = 29;
                    ((self.bits >> OFFSET) & MASK as u32) as u8
                };
                SAIC_LOR { bits }
            }
            #[doc = "Bits 16:28"]
            #[inline]
            pub fn tbs2(&self) -> TBS2R {
                let bits = {
                    const MASK: u16 = 0x1fff;
                    const OFFSET: u8 = 16;
                    ((self.bits >> OFFSET) & MASK as u32) as u16
                };
                TBS2R { bits }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn rer(&self) -> RERR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 15;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                RERR { bits }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn rch(&self) -> RCHR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                };
                RCHR { bits }
            }
            #[doc = "Bits 0:12"]
            #[inline]
            pub fn tbs1(&self) -> TBS1R {
                let bits = {
                    const MASK: u16 = 0x1fff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u16
                };
                TBS1R { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 31"]
            #[inline]
            pub fn saic_hi(&mut self) -> _SAIC_HIW {
                _SAIC_HIW { w: self }
            }
            #[doc = "Bits 29:30"]
            #[inline]
            pub fn saic_lo(&mut self) -> _SAIC_LOW {
                _SAIC_LOW { w: self }
            }
            #[doc = "Bits 16:28"]
            #[inline]
            pub fn tbs2(&mut self) -> _TBS2W {
                _TBS2W { w: self }
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn rer(&mut self) -> _RERW {
                _RERW { w: self }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn rch(&mut self) -> _RCHW {
                _RCHW { w: self }
            }
            #[doc = "Bits 0:12"]
            #[inline]
            pub fn tbs1(&mut self) -> _TBS1W {
                _TBS1W { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES2 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes2 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES2 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct BUF1PTRR {
            bits: u32,
        }
        impl BUF1PTRR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _BUF1PTRW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BUF1PTRW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf1ptr(&self) -> BUF1PTRR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                BUF1PTRR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf1ptr(&mut self) -> _BUF1PTRW {
                _BUF1PTRW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES3 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes3 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES3 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct BUF2PTR_NDAR {
            bits: u32,
        }
        impl BUF2PTR_NDAR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _BUF2PTR_NDAW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BUF2PTR_NDAW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf2ptr_nda(&self) -> BUF2PTR_NDAR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                BUF2PTR_NDAR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn buf2ptr_nda(&mut self) -> _BUF2PTR_NDAW {
                _BUF2PTR_NDAW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES4 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes4 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES4 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES5 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes5 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES5 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES6 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes6 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES6 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSLR {
            bits: u32,
        }
        impl TTSLR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSLW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSLW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsl(&self) -> TTSLR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                TTSLR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsl(&mut self) -> _TTSLW {
                _TTSLW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct RDES7 {
        pub register: vcell::VolatileCell<u32>,
    }
    #[doc = "Desc"]
    pub mod rdes7 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::RDES7 {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct TTSHR {
            bits: u32,
        }
        impl TTSHR {
            #[doc = r" Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
        #[doc = r" Proxy"]
        pub struct _TTSHW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TTSHW<'a> {
            #[doc = r" Writes raw bits to the field"]
            #[inline]
            pub unsafe fn bits(self, value: u32) -> &'a mut W {
                const MASK: u32 = 0xffff_ffff;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsh(&self) -> TTSHR {
                let bits = {
                    const MASK: u32 = 0xffff_ffff;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) as u32
                };
                TTSHR { bits }
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 0:31"]
            #[inline]
            pub fn ttsh(&mut self) -> _TTSHW {
                _TTSHW { w: self }
            }
        }
    }
}
