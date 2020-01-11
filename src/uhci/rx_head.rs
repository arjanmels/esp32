#[doc = "Reader of register RX_HEAD"]
pub type R = crate::R<u32, super::RX_HEAD>;
#[doc = "Writer for register RX_HEAD"]
pub type W = crate::W<u32, super::RX_HEAD>;
#[doc = "Register RX_HEAD `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_HEAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_RX_HEAD`"]
pub type UHCI_RX_HEAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_RX_HEAD`"]
pub struct UHCI_RX_HEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_HEAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the packet header received by DMA"]
    #[inline(always)]
    pub fn uhci_rx_head(&self) -> UHCI_RX_HEAD_R {
        UHCI_RX_HEAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the packet header received by DMA"]
    #[inline(always)]
    pub fn uhci_rx_head(&mut self) -> UHCI_RX_HEAD_W {
        UHCI_RX_HEAD_W { w: self }
    }
}