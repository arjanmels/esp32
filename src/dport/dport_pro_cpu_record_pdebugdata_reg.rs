#[doc = "Reader of register DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
pub type R = crate::R<u32, super::DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG>;
#[doc = "Writer for register DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG"]
pub type W = crate::W<u32, super::DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG>;
#[doc = "Register DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PRO_CPU_RECORD_PDEBUGDATA_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_RECORD_PRO_PDEBUGDATA`"]
pub type DPORT_RECORD_PRO_PDEBUGDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_RECORD_PRO_PDEBUGDATA`"]
pub struct DPORT_RECORD_PRO_PDEBUGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_RECORD_PRO_PDEBUGDATA_W<'a> {
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
    pub fn dport_record_pro_pdebugdata(&self) -> DPORT_RECORD_PRO_PDEBUGDATA_R {
        DPORT_RECORD_PRO_PDEBUGDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_record_pro_pdebugdata(&mut self) -> DPORT_RECORD_PRO_PDEBUGDATA_W {
        DPORT_RECORD_PRO_PDEBUGDATA_W { w: self }
    }
}