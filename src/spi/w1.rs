#[doc = "Reader of register W1"]
pub type R = crate::R<u32, super::W1>;
#[doc = "Writer for register W1"]
pub type W = crate::W<u32, super::W1>;
#[doc = "Register W1 `reset()`'s with value 0"]
impl crate::ResetValue for super::W1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF1`"]
pub type SPI_BUF1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF1`"]
pub struct SPI_BUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf1(&self) -> SPI_BUF1_R {
        SPI_BUF1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf1(&mut self) -> SPI_BUF1_W {
        SPI_BUF1_W { w: self }
    }
}