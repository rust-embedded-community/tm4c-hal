#![doc = "Peripheral access API for EPHY microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
extern crate bare_metal;
extern crate vcell;
pub type EPHY = ephy::RegisterBlock;
#[doc = "Ethernet PHY"]
pub mod ephy {
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Desc"]
        pub bmcr: BMCR,
        #[doc = "0x02 - Desc"]
        pub bmsr: BMSR,
    }
    impl RegisterBlock {
        pub fn new(index: u8) -> RegisterBlock {
            RegisterBlock {
                bmcr: BMCR {
                    index,
                    register: crate::ethernet::EphyReg(0u8),
                },
                bmsr: BMSR {
                    index,
                    register: crate::ethernet::EphyReg(1u8),
                },
            }
        }
    }
    #[doc = "Desc"]
    pub struct BMCR {
        pub register: crate::ethernet::EphyReg,
        pub index: u8,
    }
    #[doc = "Desc"]
    pub mod bmcr {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::BMCR {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, emac0: &mut tm4c129x::EMAC0, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get(emac0, self.index);
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(emac0, self.index, w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self, emac0: &mut tm4c129x::EMAC0) -> R {
                R {
                    bits: self.register.get(emac0, self.index),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, emac0: &mut tm4c129x::EMAC0, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(emac0, self.index, w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self, emac0: &mut tm4c129x::EMAC0) {
                self.write(emac0, |w| w)
            }
        }
        #[doc = r" Value of the field"]
        pub struct MIIRESETR {
            bits: bool,
        }
        impl MIIRESETR {
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
        pub struct MIILOOPBKR {
            bits: bool,
        }
        impl MIILOOPBKR {
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
        pub struct SPEEDR {
            bits: bool,
        }
        impl SPEEDR {
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
        pub struct ANENR {
            bits: bool,
        }
        impl ANENR {
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
        pub struct PWRDWNR {
            bits: bool,
        }
        impl PWRDWNR {
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
        pub struct ISOLATER {
            bits: bool,
        }
        impl ISOLATER {
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
        pub struct RESTARTANR {
            bits: bool,
        }
        impl RESTARTANR {
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
        pub struct DUPLEXMR {
            bits: bool,
        }
        impl DUPLEXMR {
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
        pub struct COLLTSTR {
            bits: bool,
        }
        impl COLLTSTR {
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
        pub struct _MIIRESETW<'a> {
            w: &'a mut W,
        }
        impl<'a> _MIIRESETW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _MIILOOPBKW<'a> {
            w: &'a mut W,
        }
        impl<'a> _MIILOOPBKW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _SPEEDW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SPEEDW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _ANENW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ANENW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _PWRDWNW<'a> {
            w: &'a mut W,
        }
        impl<'a> _PWRDWNW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _ISOLATEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ISOLATEW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _RESTARTANW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RESTARTANW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _DUPLEXMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DUPLEXMW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = r" Proxy"]
        pub struct _COLLTSTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _COLLTSTW<'a> {
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
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn miireset(&self) -> MIIRESETR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 15;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                MIIRESETR { bits }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn miiloopbk(&self) -> MIILOOPBKR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                MIILOOPBKR { bits }
            }
            #[doc = "Bit 13"]
            #[inline]
            pub fn speed(&self) -> SPEEDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 13;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                SPEEDR { bits }
            }
            #[doc = "Bit 12"]
            #[inline]
            pub fn anen(&self) -> ANENR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 12;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                ANENR { bits }
            }
            #[doc = "Bit 11"]
            #[inline]
            pub fn pwrdwn(&self) -> PWRDWNR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 11;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                PWRDWNR { bits }
            }
            #[doc = "Bit 10"]
            #[inline]
            pub fn isolate(&self) -> ISOLATER {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 10;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                ISOLATER { bits }
            }
            #[doc = "Bit 9"]
            #[inline]
            pub fn restartan(&self) -> RESTARTANR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 9;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                RESTARTANR { bits }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn duplexm(&self) -> DUPLEXMR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                DUPLEXMR { bits }
            }
            #[doc = "Bit 7"]
            #[inline]
            pub fn colltst(&self) -> COLLTSTR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                COLLTSTR { bits }
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
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 15"]
            #[inline]
            pub fn miireset(&mut self) -> _MIIRESETW {
                _MIIRESETW { w: self }
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn miiloopbk(&mut self) -> _MIILOOPBKW {
                _MIILOOPBKW { w: self }
            }
            #[doc = "Bit 13"]
            #[inline]
            pub fn speed(&mut self) -> _SPEEDW {
                _SPEEDW { w: self }
            }
            #[doc = "Bit 12"]
            #[inline]
            pub fn anen(&mut self) -> _ANENW {
                _ANENW { w: self }
            }
            #[doc = "Bit 11"]
            #[inline]
            pub fn pwrdwn(&mut self) -> _PWRDWNW {
                _PWRDWNW { w: self }
            }
            #[doc = "Bit 10"]
            #[inline]
            pub fn isolate(&mut self) -> _ISOLATEW {
                _ISOLATEW { w: self }
            }
            #[doc = "Bit 9"]
            #[inline]
            pub fn restartan(&mut self) -> _RESTARTANW {
                _RESTARTANW { w: self }
            }
            #[doc = "Bit 8"]
            #[inline]
            pub fn duplexm(&mut self) -> _DUPLEXMW {
                _DUPLEXMW { w: self }
            }
            #[doc = "Bit 7"]
            #[inline]
            pub fn colltst(&mut self) -> _COLLTSTW {
                _COLLTSTW { w: self }
            }
        }
    }
    #[doc = "Desc"]
    pub struct BMSR {
        pub register: crate::ethernet::EphyReg,
        pub index: u8,
    }
    #[doc = "Desc"]
    pub mod bmsr {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        impl super::BMSR {
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self, emac0: &mut tm4c129x::EMAC0) -> R {
                R {
                    bits: self.register.get(emac0, self.index),
                }
            }
        }
        #[doc = r" Value of the field"]
        pub struct _100BTXFDR {
            bits: bool,
        }
        impl _100BTXFDR {
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
        pub struct _100BTXHDR {
            bits: bool,
        }
        impl _100BTXHDR {
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
        pub struct _10BTFDR {
            bits: bool,
        }
        impl _10BTFDR {
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
        pub struct _10BTHDR {
            bits: bool,
        }
        impl _10BTHDR {
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
        pub struct MFPRESUPR {
            bits: bool,
        }
        impl MFPRESUPR {
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
        pub struct ANCR {
            bits: bool,
        }
        impl ANCR {
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
        pub struct RFAULTR {
            bits: bool,
        }
        impl RFAULTR {
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
        pub struct ANENR {
            bits: bool,
        }
        impl ANENR {
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
        pub struct LINKSTATR {
            bits: bool,
        }
        impl LINKSTATR {
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
        pub struct JABBERR {
            bits: bool,
        }
        impl JABBERR {
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
        pub struct EXTENR {
            bits: bool,
        }
        impl EXTENR {
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
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 14"]
            #[inline]
            pub fn _100btxfd(&self) -> _100BTXFDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                _100BTXFDR { bits }
            }
            #[doc = "Bit 13"]
            #[inline]
            pub fn _100btxhd(&self) -> _100BTXHDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 13;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                _100BTXHDR { bits }
            }
            #[doc = "Bit 12"]
            #[inline]
            pub fn _10btfd(&self) -> _10BTFDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 12;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                _10BTFDR { bits }
            }
            #[doc = "Bit 11"]
            #[inline]
            pub fn _10bthd(&self) -> _10BTHDR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 11;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                _10BTHDR { bits }
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn mfpresup(&self) -> MFPRESUPR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                MFPRESUPR { bits }
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn anc(&self) -> ANCR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                ANCR { bits }
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn rfault(&self) -> RFAULTR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                RFAULTR { bits }
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn anen(&self) -> ANENR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                ANENR { bits }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn linkstat(&self) -> LINKSTATR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                LINKSTATR { bits }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn jabber(&self) -> JABBERR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                JABBERR { bits }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn exten(&self) -> EXTENR {
                let bits = {
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                };
                EXTENR { bits }
            }
        }
    }
}
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS_EPHY: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "EPHY"]
    pub EPHY: (
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
        EPHY,
    ),
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS_EPHY } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS_EPHY);
        DEVICE_PERIPHERALS_EPHY = true;
        Peripherals {
            EPHY: (
                EPHY::new(0),
                EPHY::new(1),
                EPHY::new(2),
                EPHY::new(3),
                EPHY::new(4),
                EPHY::new(5),
                EPHY::new(6),
                EPHY::new(7),
                EPHY::new(8),
                EPHY::new(9),
                EPHY::new(10),
                EPHY::new(11),
                EPHY::new(12),
                EPHY::new(13),
                EPHY::new(14),
                EPHY::new(15),
            ),
        }
    }
}
