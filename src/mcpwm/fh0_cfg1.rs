#[doc = "Reader of register FH0_CFG1"]
pub type R = crate::R<u32, super::FH0_CFG1>;
#[doc = "Writer for register FH0_CFG1"]
pub type W = crate::W<u32, super::FH0_CFG1>;
#[doc = "Register FH0_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FH0_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_FH0_FORCE_OST`"]
pub type MCPWM_FH0_FORCE_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH0_FORCE_OST`"]
pub struct MCPWM_FH0_FORCE_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_FORCE_OST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH0_FORCE_CBC`"]
pub type MCPWM_FH0_FORCE_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH0_FORCE_CBC`"]
pub struct MCPWM_FH0_FORCE_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_FORCE_CBC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH0_CBCPULSE`"]
pub type MCPWM_FH0_CBCPULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH0_CBCPULSE`"]
pub struct MCPWM_FH0_CBCPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_CBCPULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH0_CLR_OST`"]
pub type MCPWM_FH0_CLR_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH0_CLR_OST`"]
pub struct MCPWM_FH0_CLR_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_CLR_OST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - A toggle (software negation of value of this bit) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_force_ost(&self) -> MCPWM_FH0_FORCE_OST_R {
        MCPWM_FH0_FORCE_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A toggle triggers a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_force_cbc(&self) -> MCPWM_FH0_FORCE_CBC_R {
        MCPWM_FH0_FORCE_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - The cycle-by-cycle mode action refresh moment selection. Bit0: TEZ bit1:TEP"]
    #[inline(always)]
    pub fn mcpwm_fh0_cbcpulse(&self) -> MCPWM_FH0_CBCPULSE_R {
        MCPWM_FH0_CBCPULSE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - A toggle will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_clr_ost(&self) -> MCPWM_FH0_CLR_OST_R {
        MCPWM_FH0_CLR_OST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - A toggle (software negation of value of this bit) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_force_ost(&mut self) -> MCPWM_FH0_FORCE_OST_W {
        MCPWM_FH0_FORCE_OST_W { w: self }
    }
    #[doc = "Bit 3 - A toggle triggers a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_force_cbc(&mut self) -> MCPWM_FH0_FORCE_CBC_W {
        MCPWM_FH0_FORCE_CBC_W { w: self }
    }
    #[doc = "Bits 1:2 - The cycle-by-cycle mode action refresh moment selection. Bit0: TEZ bit1:TEP"]
    #[inline(always)]
    pub fn mcpwm_fh0_cbcpulse(&mut self) -> MCPWM_FH0_CBCPULSE_W {
        MCPWM_FH0_CBCPULSE_W { w: self }
    }
    #[doc = "Bit 0 - A toggle will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn mcpwm_fh0_clr_ost(&mut self) -> MCPWM_FH0_CLR_OST_W {
        MCPWM_FH0_CLR_OST_W { w: self }
    }
}