#[doc = "Reader of register HOST_SLCHOSTID"]
pub type R = crate::R<u32, super::HOST_SLCHOSTID>;
#[doc = "Writer for register HOST_SLCHOSTID"]
pub type W = crate::W<u32, super::HOST_SLCHOSTID>;
#[doc = "Register HOST_SLCHOSTID `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOSTID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_ID`"]
pub type HOST_SLCHOST_ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_SLCHOST_ID`"]
pub struct HOST_SLCHOST_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_id(&self) -> HOST_SLCHOST_ID_R {
        HOST_SLCHOST_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_id(&mut self) -> HOST_SLCHOST_ID_W {
        HOST_SLCHOST_ID_W { w: self }
    }
}