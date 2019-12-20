#[doc = "Reader of register DPORT_AHB_MPU_TABLE_0_REG"]
pub type R = crate::R<u32, super::DPORT_AHB_MPU_TABLE_0_REG>;
#[doc = "Writer for register DPORT_AHB_MPU_TABLE_0_REG"]
pub type W = crate::W<u32, super::DPORT_AHB_MPU_TABLE_0_REG>;
#[doc = "Register DPORT_AHB_MPU_TABLE_0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_AHB_MPU_TABLE_0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_AHB_ACCESS_GRANT_0`"]
pub type DPORT_AHB_ACCESS_GRANT_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_AHB_ACCESS_GRANT_0`"]
pub struct DPORT_AHB_ACCESS_GRANT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_ACCESS_GRANT_0_W<'a> {
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
    pub fn dport_ahb_access_grant_0(&self) -> DPORT_AHB_ACCESS_GRANT_0_R {
        DPORT_AHB_ACCESS_GRANT_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_ahb_access_grant_0(&mut self) -> DPORT_AHB_ACCESS_GRANT_0_W {
        DPORT_AHB_ACCESS_GRANT_0_W { w: self }
    }
}