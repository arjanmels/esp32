#[doc = "Reader of register DPORT_ROM_PD_CTRL_REG"]
pub type R = crate::R<u32, super::DPORT_ROM_PD_CTRL_REG>;
#[doc = "Writer for register DPORT_ROM_PD_CTRL_REG"]
pub type W = crate::W<u32, super::DPORT_ROM_PD_CTRL_REG>;
#[doc = "Register DPORT_ROM_PD_CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_ROM_PD_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SHARE_ROM_PD`"]
pub type DPORT_SHARE_ROM_PD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SHARE_ROM_PD`"]
pub struct DPORT_SHARE_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SHARE_ROM_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_ROM_PD`"]
pub type DPORT_APP_ROM_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_ROM_PD`"]
pub struct DPORT_APP_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_ROM_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_ROM_PD`"]
pub type DPORT_PRO_ROM_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_ROM_PD`"]
pub struct DPORT_PRO_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_ROM_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dport_share_rom_pd(&self) -> DPORT_SHARE_ROM_PD_R {
        DPORT_SHARE_ROM_PD_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_app_rom_pd(&self) -> DPORT_APP_ROM_PD_R {
        DPORT_APP_ROM_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_pd(&self) -> DPORT_PRO_ROM_PD_R {
        DPORT_PRO_ROM_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dport_share_rom_pd(&mut self) -> DPORT_SHARE_ROM_PD_W {
        DPORT_SHARE_ROM_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_app_rom_pd(&mut self) -> DPORT_APP_ROM_PD_W {
        DPORT_APP_ROM_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_pd(&mut self) -> DPORT_PRO_ROM_PD_W {
        DPORT_PRO_ROM_PD_W { w: self }
    }
}