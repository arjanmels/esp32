#[doc = "Reader of register HINF_CFG_DATA0_REG"]
pub type R = crate::R<u32, super::HINF_CFG_DATA0_REG>;
#[doc = "Writer for register HINF_CFG_DATA0_REG"]
pub type W = crate::W<u32, super::HINF_CFG_DATA0_REG>;
#[doc = "Register HINF_CFG_DATA0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HINF_CFG_DATA0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HINF_DEVICE_ID_FN1`"]
pub type HINF_DEVICE_ID_FN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HINF_DEVICE_ID_FN1`"]
pub struct HINF_DEVICE_ID_FN1_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_DEVICE_ID_FN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HINF_USER_ID_FN1`"]
pub type HINF_USER_ID_FN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HINF_USER_ID_FN1`"]
pub struct HINF_USER_ID_FN1_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_USER_ID_FN1_W<'a> {
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
    pub fn hinf_device_id_fn1(&self) -> HINF_DEVICE_ID_FN1_R {
        HINF_DEVICE_ID_FN1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn hinf_user_id_fn1(&self) -> HINF_USER_ID_FN1_R {
        HINF_USER_ID_FN1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn hinf_device_id_fn1(&mut self) -> HINF_DEVICE_ID_FN1_W {
        HINF_DEVICE_ID_FN1_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn hinf_user_id_fn1(&mut self) -> HINF_USER_ID_FN1_W {
        HINF_USER_ID_FN1_W { w: self }
    }
}