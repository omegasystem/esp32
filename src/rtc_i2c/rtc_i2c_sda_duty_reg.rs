# [ doc = "Reader of register RTC_I2C_SDA_DUTY_REG" ] pub type R = crate :: R < u32 , super :: RTC_I2C_SDA_DUTY_REG > ; # [ doc = "Writer for register RTC_I2C_SDA_DUTY_REG" ] pub type W = crate :: W < u32 , super :: RTC_I2C_SDA_DUTY_REG > ; # [ doc = "Register RTC_I2C_SDA_DUTY_REG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: RTC_I2C_SDA_DUTY_REG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `RTC_I2C_SDA_DUTY`" ] pub type RTC_I2C_SDA_DUTY_R = crate :: R < u32 , u32 > ; # [ doc = "Write proxy for field `RTC_I2C_SDA_DUTY`" ] pub struct RTC_I2C_SDA_DUTY_W < 'a > { w : & 'a mut W , } impl < 'a > RTC_I2C_SDA_DUTY_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u32 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x000f_ffff ) | ( ( value as u32 ) & 0x000f_ffff ) ; self . w } } impl R { # [ doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL" ] # [ inline ( always ) ] pub fn rtc_i2c_sda_duty ( & self ) -> RTC_I2C_SDA_DUTY_R { RTC_I2C_SDA_DUTY_R :: new ( ( self . bits & 0x000f_ffff ) as u32 ) } } impl W { # [ doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL" ] # [ inline ( always ) ] pub fn rtc_i2c_sda_duty ( & mut self ) -> RTC_I2C_SDA_DUTY_W { RTC_I2C_SDA_DUTY_W { w : self } } }