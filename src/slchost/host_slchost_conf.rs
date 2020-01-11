#[doc = "Reader of register HOST_SLCHOST_CONF"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF>;
#[doc = "Writer for register HOST_SLCHOST_CONF"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF>;
#[doc = "Register HOST_SLCHOST_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_HSPEED_CON_EN`"]
pub type HOST_HSPEED_CON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_HSPEED_CON_EN`"]
pub struct HOST_HSPEED_CON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HSPEED_CON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `HOST_SDIO_PAD_PULLUP`"]
pub type HOST_SDIO_PAD_PULLUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SDIO_PAD_PULLUP`"]
pub struct HOST_SDIO_PAD_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO_PAD_PULLUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `HOST_SDIO20_INT_DELAY`"]
pub type HOST_SDIO20_INT_DELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SDIO20_INT_DELAY`"]
pub struct HOST_SDIO20_INT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO20_INT_DELAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HOST_FRC_QUICK_IN`"]
pub type HOST_FRC_QUICK_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_FRC_QUICK_IN`"]
pub struct HOST_FRC_QUICK_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_QUICK_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `HOST_FRC_POS_SAMP`"]
pub type HOST_FRC_POS_SAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_FRC_POS_SAMP`"]
pub struct HOST_FRC_POS_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_POS_SAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `HOST_FRC_NEG_SAMP`"]
pub type HOST_FRC_NEG_SAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_FRC_NEG_SAMP`"]
pub struct HOST_FRC_NEG_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_NEG_SAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `HOST_FRC_SDIO20`"]
pub type HOST_FRC_SDIO20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_FRC_SDIO20`"]
pub struct HOST_FRC_SDIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_SDIO20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `HOST_FRC_SDIO11`"]
pub type HOST_FRC_SDIO11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_FRC_SDIO11`"]
pub struct HOST_FRC_SDIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_SDIO11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&self) -> HOST_HSPEED_CON_EN_R {
        HOST_HSPEED_CON_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&self) -> HOST_SDIO_PAD_PULLUP_R {
        HOST_SDIO_PAD_PULLUP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&self) -> HOST_SDIO20_INT_DELAY_R {
        HOST_SDIO20_INT_DELAY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&self) -> HOST_FRC_QUICK_IN_R {
        HOST_FRC_QUICK_IN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&self) -> HOST_FRC_POS_SAMP_R {
        HOST_FRC_POS_SAMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&self) -> HOST_FRC_NEG_SAMP_R {
        HOST_FRC_NEG_SAMP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&self) -> HOST_FRC_SDIO20_R {
        HOST_FRC_SDIO20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&self) -> HOST_FRC_SDIO11_R {
        HOST_FRC_SDIO11_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&mut self) -> HOST_HSPEED_CON_EN_W {
        HOST_HSPEED_CON_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&mut self) -> HOST_SDIO_PAD_PULLUP_W {
        HOST_SDIO_PAD_PULLUP_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&mut self) -> HOST_SDIO20_INT_DELAY_W {
        HOST_SDIO20_INT_DELAY_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&mut self) -> HOST_FRC_QUICK_IN_W {
        HOST_FRC_QUICK_IN_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&mut self) -> HOST_FRC_POS_SAMP_W {
        HOST_FRC_POS_SAMP_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&mut self) -> HOST_FRC_NEG_SAMP_W {
        HOST_FRC_NEG_SAMP_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&mut self) -> HOST_FRC_SDIO20_W {
        HOST_FRC_SDIO20_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&mut self) -> HOST_FRC_SDIO11_W {
        HOST_FRC_SDIO11_W { w: self }
    }
}