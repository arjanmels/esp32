#[doc = "Reader of register HOST_SLC1HOST_RX_INFOR_REG"]
pub type R = crate::R<u32, super::HOST_SLC1HOST_RX_INFOR_REG>;
#[doc = "Writer for register HOST_SLC1HOST_RX_INFOR_REG"]
pub type W = crate::W<u32, super::HOST_SLC1HOST_RX_INFOR_REG>;
#[doc = "Register HOST_SLC1HOST_RX_INFOR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC1HOST_RX_INFOR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC1HOST_RX_INFOR`"]
pub type HOST_SLC1HOST_RX_INFOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_SLC1HOST_RX_INFOR`"]
pub struct HOST_SLC1HOST_RX_INFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_RX_INFOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_slc1host_rx_infor(&self) -> HOST_SLC1HOST_RX_INFOR_R {
        HOST_SLC1HOST_RX_INFOR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_slc1host_rx_infor(&mut self) -> HOST_SLC1HOST_RX_INFOR_W {
        HOST_SLC1HOST_RX_INFOR_W { w: self }
    }
}