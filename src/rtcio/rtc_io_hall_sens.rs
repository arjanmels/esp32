#[doc = "Reader of register RTC_IO_HALL_SENS"]
pub type R = crate::R<u32, super::RTC_IO_HALL_SENS>;
#[doc = "Writer for register RTC_IO_HALL_SENS"]
pub type W = crate::W<u32, super::RTC_IO_HALL_SENS>;
#[doc = "Register RTC_IO_HALL_SENS `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_HALL_SENS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_XPD_HALL`"]
pub type RTC_IO_XPD_HALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_XPD_HALL`"]
pub struct RTC_IO_XPD_HALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_XPD_HALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_HALL_PHASE`"]
pub type RTC_IO_HALL_PHASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_HALL_PHASE`"]
pub struct RTC_IO_HALL_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_HALL_PHASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn rtc_io_xpd_hall(&self) -> RTC_IO_XPD_HALL_R {
        RTC_IO_XPD_HALL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn rtc_io_hall_phase(&self) -> RTC_IO_HALL_PHASE_R {
        RTC_IO_HALL_PHASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn rtc_io_xpd_hall(&mut self) -> RTC_IO_XPD_HALL_W {
        RTC_IO_XPD_HALL_W { w: self }
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn rtc_io_hall_phase(&mut self) -> RTC_IO_HALL_PHASE_W {
        RTC_IO_HALL_PHASE_W { w: self }
    }
}