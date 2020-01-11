#[doc = "Reader of register BLK2_RDATA3"]
pub type R = crate::R<u32, super::BLK2_RDATA3>;
#[doc = "Writer for register BLK2_RDATA3"]
pub type W = crate::W<u32, super::BLK2_RDATA3>;
#[doc = "Register BLK2_RDATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK2_RDATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK2_DOUT3`"]
pub type EFUSE_BLK2_DOUT3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_BLK2_DOUT3`"]
pub struct EFUSE_BLK2_DOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK2_DOUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn efuse_blk2_dout3(&self) -> EFUSE_BLK2_DOUT3_R {
        EFUSE_BLK2_DOUT3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn efuse_blk2_dout3(&mut self) -> EFUSE_BLK2_DOUT3_W {
        EFUSE_BLK2_DOUT3_W { w: self }
    }
}