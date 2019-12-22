#[doc = "Reader of register MCPWM_CAP_TIMER_PHASE_REG"]
pub type R = crate::R<u32, super::MCPWM_CAP_TIMER_PHASE_REG>;
#[doc = "Writer for register MCPWM_CAP_TIMER_PHASE_REG"]
pub type W = crate::W<u32, super::MCPWM_CAP_TIMER_PHASE_REG>;
#[doc = "Register MCPWM_CAP_TIMER_PHASE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_CAP_TIMER_PHASE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP_PHASE`"]
pub type MCPWM_CAP_PHASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCPWM_CAP_PHASE`"]
pub struct MCPWM_CAP_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn mcpwm_cap_phase(&self) -> MCPWM_CAP_PHASE_R {
        MCPWM_CAP_PHASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn mcpwm_cap_phase(&mut self) -> MCPWM_CAP_PHASE_W {
        MCPWM_CAP_PHASE_W { w: self }
    }
}