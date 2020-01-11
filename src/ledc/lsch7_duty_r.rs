#[doc = "Reader of register LSCH7_DUTY_R"]
pub type R = crate::R<u32, super::LSCH7_DUTY_R>;
#[doc = "Writer for register LSCH7_DUTY_R"]
pub type W = crate::W<u32, super::LSCH7_DUTY_R>;
#[doc = "Register LSCH7_DUTY_R `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH7_DUTY_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_LSCH7`"]
pub type LEDC_DUTY_LSCH7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DUTY_LSCH7`"]
pub struct LEDC_DUTY_LSCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_LSCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel7."]
    #[inline(always)]
    pub fn ledc_duty_lsch7(&self) -> LEDC_DUTY_LSCH7_R {
        LEDC_DUTY_LSCH7_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel7."]
    #[inline(always)]
    pub fn ledc_duty_lsch7(&mut self) -> LEDC_DUTY_LSCH7_W {
        LEDC_DUTY_LSCH7_W { w: self }
    }
}