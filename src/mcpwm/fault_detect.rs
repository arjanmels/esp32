#[doc = "Reader of register FAULT_DETECT"]
pub type R = crate::R<u32, super::FAULT_DETECT>;
#[doc = "Writer for register FAULT_DETECT"]
pub type W = crate::W<u32, super::FAULT_DETECT>;
#[doc = "Register FAULT_DETECT `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULT_DETECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_EVENT_F2`"]
pub type MCPWM_EVENT_F2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EVENT_F2`"]
pub struct MCPWM_EVENT_F2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EVENT_F2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_EVENT_F1`"]
pub type MCPWM_EVENT_F1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EVENT_F1`"]
pub struct MCPWM_EVENT_F1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EVENT_F1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_EVENT_F0`"]
pub type MCPWM_EVENT_F0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EVENT_F0`"]
pub struct MCPWM_EVENT_F0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EVENT_F0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_F2_POLE`"]
pub type MCPWM_F2_POLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F2_POLE`"]
pub struct MCPWM_F2_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F2_POLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_F1_POLE`"]
pub type MCPWM_F1_POLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F1_POLE`"]
pub struct MCPWM_F1_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F1_POLE_W<'a> {
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
#[doc = "Reader of field `MCPWM_F0_POLE`"]
pub type MCPWM_F0_POLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F0_POLE`"]
pub struct MCPWM_F0_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F0_POLE_W<'a> {
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
#[doc = "Reader of field `MCPWM_F2_EN`"]
pub type MCPWM_F2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F2_EN`"]
pub struct MCPWM_F2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_F1_EN`"]
pub type MCPWM_F1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F1_EN`"]
pub struct MCPWM_F1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_F0_EN`"]
pub type MCPWM_F0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_F0_EN`"]
pub struct MCPWM_F0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_F0_EN_W<'a> {
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
    #[doc = "Bit 8 - Set and reset by hardware. If set event_f2 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f2(&self) -> MCPWM_EVENT_F2_R {
        MCPWM_EVENT_F2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set and reset by hardware. If set event_f1 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f1(&self) -> MCPWM_EVENT_F1_R {
        MCPWM_EVENT_F1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set and reset by hardware. If set event_f0 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f0(&self) -> MCPWM_EVENT_F0_R {
        MCPWM_EVENT_F0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set event_f2 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f2_pole(&self) -> MCPWM_F2_POLE_R {
        MCPWM_F2_POLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set event_f1 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f1_pole(&self) -> MCPWM_F1_POLE_R {
        MCPWM_F1_POLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set event_f0 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f0_pole(&self) -> MCPWM_F0_POLE_R {
        MCPWM_F0_POLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to enable generation of event_f2"]
    #[inline(always)]
    pub fn mcpwm_f2_en(&self) -> MCPWM_F2_EN_R {
        MCPWM_F2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to enable generation of event_f1"]
    #[inline(always)]
    pub fn mcpwm_f1_en(&self) -> MCPWM_F1_EN_R {
        MCPWM_F1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set to enable generation of event_f0"]
    #[inline(always)]
    pub fn mcpwm_f0_en(&self) -> MCPWM_F0_EN_R {
        MCPWM_F0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Set and reset by hardware. If set event_f2 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f2(&mut self) -> MCPWM_EVENT_F2_W {
        MCPWM_EVENT_F2_W { w: self }
    }
    #[doc = "Bit 7 - Set and reset by hardware. If set event_f1 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f1(&mut self) -> MCPWM_EVENT_F1_W {
        MCPWM_EVENT_F1_W { w: self }
    }
    #[doc = "Bit 6 - Set and reset by hardware. If set event_f0 is on going"]
    #[inline(always)]
    pub fn mcpwm_event_f0(&mut self) -> MCPWM_EVENT_F0_W {
        MCPWM_EVENT_F0_W { w: self }
    }
    #[doc = "Bit 5 - Set event_f2 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f2_pole(&mut self) -> MCPWM_F2_POLE_W {
        MCPWM_F2_POLE_W { w: self }
    }
    #[doc = "Bit 4 - Set event_f1 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f1_pole(&mut self) -> MCPWM_F1_POLE_W {
        MCPWM_F1_POLE_W { w: self }
    }
    #[doc = "Bit 3 - Set event_f0 trigger polarity on FAULT2 source from GPIO matrix. 0: level low 1: level high"]
    #[inline(always)]
    pub fn mcpwm_f0_pole(&mut self) -> MCPWM_F0_POLE_W {
        MCPWM_F0_POLE_W { w: self }
    }
    #[doc = "Bit 2 - Set to enable generation of event_f2"]
    #[inline(always)]
    pub fn mcpwm_f2_en(&mut self) -> MCPWM_F2_EN_W {
        MCPWM_F2_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set to enable generation of event_f1"]
    #[inline(always)]
    pub fn mcpwm_f1_en(&mut self) -> MCPWM_F1_EN_W {
        MCPWM_F1_EN_W { w: self }
    }
    #[doc = "Bit 0 - Set to enable generation of event_f0"]
    #[inline(always)]
    pub fn mcpwm_f0_en(&mut self) -> MCPWM_F0_EN_W {
        MCPWM_F0_EN_W { w: self }
    }
}