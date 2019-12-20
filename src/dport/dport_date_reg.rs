#[doc = "Reader of register DPORT_DATE_REG"]
pub type R = crate::R<u32, super::DPORT_DATE_REG>;
#[doc = "Writer for register DPORT_DATE_REG"]
pub type W = crate::W<u32, super::DPORT_DATE_REG>;
#[doc = "Register DPORT_DATE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_DATE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_DATE`"]
pub type DPORT_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_DATE`"]
pub struct DPORT_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn dport_date(&self) -> DPORT_DATE_R {
        DPORT_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn dport_date(&mut self) -> DPORT_DATE_W {
        DPORT_DATE_W { w: self }
    }
}