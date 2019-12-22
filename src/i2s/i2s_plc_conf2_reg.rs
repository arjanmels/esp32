#[doc = "Reader of register I2S_PLC_CONF2_REG"]
pub type R = crate::R<u32, super::I2S_PLC_CONF2_REG>;
#[doc = "Writer for register I2S_PLC_CONF2_REG"]
pub type W = crate::W<u32, super::I2S_PLC_CONF2_REG>;
#[doc = "Register I2S_PLC_CONF2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_PLC_CONF2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_MIN_PERIOD`"]
pub type I2S_MIN_PERIOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_MIN_PERIOD`"]
pub struct I2S_MIN_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MIN_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_SEG_MOD`"]
pub type I2S_CVSD_SEG_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_CVSD_SEG_MOD`"]
pub struct I2S_CVSD_SEG_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_SEG_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn i2s_min_period(&self) -> I2S_MIN_PERIOD_R {
        I2S_MIN_PERIOD_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn i2s_cvsd_seg_mod(&self) -> I2S_CVSD_SEG_MOD_R {
        I2S_CVSD_SEG_MOD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn i2s_min_period(&mut self) -> I2S_MIN_PERIOD_W {
        I2S_MIN_PERIOD_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn i2s_cvsd_seg_mod(&mut self) -> I2S_CVSD_SEG_MOD_W {
        I2S_CVSD_SEG_MOD_W { w: self }
    }
}