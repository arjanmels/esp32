#[doc = "Reader of register CVSD_CONF0"]
pub type R = crate::R<u32, super::CVSD_CONF0>;
#[doc = "Writer for register CVSD_CONF0"]
pub type W = crate::W<u32, super::CVSD_CONF0>;
#[doc = "Register CVSD_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CVSD_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_CVSD_Y_MIN`"]
pub type I2S_CVSD_Y_MIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_CVSD_Y_MIN`"]
pub struct I2S_CVSD_Y_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_Y_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_CVSD_Y_MAX`"]
pub type I2S_CVSD_Y_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_CVSD_Y_MAX`"]
pub struct I2S_CVSD_Y_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CVSD_Y_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn i2s_cvsd_y_min(&self) -> I2S_CVSD_Y_MIN_R {
        I2S_CVSD_Y_MIN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s_cvsd_y_max(&self) -> I2S_CVSD_Y_MAX_R {
        I2S_CVSD_Y_MAX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn i2s_cvsd_y_min(&mut self) -> I2S_CVSD_Y_MIN_W {
        I2S_CVSD_Y_MIN_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s_cvsd_y_max(&mut self) -> I2S_CVSD_Y_MAX_W {
        I2S_CVSD_Y_MAX_W { w: self }
    }
}