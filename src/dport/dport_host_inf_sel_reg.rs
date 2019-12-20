#[doc = "Reader of register DPORT_HOST_INF_SEL_REG"]
pub type R = crate::R<u32, super::DPORT_HOST_INF_SEL_REG>;
#[doc = "Writer for register DPORT_HOST_INF_SEL_REG"]
pub type W = crate::W<u32, super::DPORT_HOST_INF_SEL_REG>;
#[doc = "Register DPORT_HOST_INF_SEL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_HOST_INF_SEL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_LINK_DEVICE_SEL`"]
pub type DPORT_LINK_DEVICE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_LINK_DEVICE_SEL`"]
pub struct DPORT_LINK_DEVICE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_LINK_DEVICE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PERI_IO_SWAP`"]
pub type DPORT_PERI_IO_SWAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PERI_IO_SWAP`"]
pub struct DPORT_PERI_IO_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PERI_IO_SWAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dport_link_device_sel(&self) -> DPORT_LINK_DEVICE_SEL_R {
        DPORT_LINK_DEVICE_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dport_peri_io_swap(&self) -> DPORT_PERI_IO_SWAP_R {
        DPORT_PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dport_link_device_sel(&mut self) -> DPORT_LINK_DEVICE_SEL_W {
        DPORT_LINK_DEVICE_SEL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dport_peri_io_swap(&mut self) -> DPORT_PERI_IO_SWAP_W {
        DPORT_PERI_IO_SWAP_W { w: self }
    }
}