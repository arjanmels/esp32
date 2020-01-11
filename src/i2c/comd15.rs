#[doc = "Reader of register COMD15"]
pub type R = crate::R<u32, super::COMD15>;
#[doc = "Writer for register COMD15"]
pub type W = crate::W<u32, super::COMD15>;
#[doc = "Register COMD15 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMD15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_COMMAND15_DONE`"]
pub type I2C_COMMAND15_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_COMMAND15_DONE`"]
pub struct I2C_COMMAND15_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_COMMAND15_DONE_W<'a> {
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
#[doc = "Reader of field `I2C_COMMAND15`"]
pub type I2C_COMMAND15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_COMMAND15`"]
pub struct I2C_COMMAND15_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_COMMAND15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When command15 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command15_done(&self) -> I2C_COMMAND15_DONE_R {
        I2C_COMMAND15_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - This is the content of command15. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn i2c_command15(&self) -> I2C_COMMAND15_R {
        I2C_COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - When command15 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command15_done(&mut self) -> I2C_COMMAND15_DONE_W {
        I2C_COMMAND15_DONE_W { w: self }
    }
    #[doc = "Bits 0:13 - This is the content of command15. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn i2c_command15(&mut self) -> I2C_COMMAND15_W {
        I2C_COMMAND15_W { w: self }
    }
}