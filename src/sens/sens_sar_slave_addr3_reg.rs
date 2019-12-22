# [ doc = "Reader of register SENS_SAR_SLAVE_ADDR3_REG" ] pub type R = crate :: R < u32 , super :: SENS_SAR_SLAVE_ADDR3_REG > ; # [ doc = "Writer for register SENS_SAR_SLAVE_ADDR3_REG" ] pub type W = crate :: W < u32 , super :: SENS_SAR_SLAVE_ADDR3_REG > ; # [ doc = "Register SENS_SAR_SLAVE_ADDR3_REG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SENS_SAR_SLAVE_ADDR3_REG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SENS_TSENS_RDY_OUT`" ] pub type SENS_TSENS_RDY_OUT_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SENS_TSENS_RDY_OUT`" ] pub struct SENS_TSENS_RDY_OUT_W < 'a > { w : & 'a mut W , } impl < 'a > SENS_TSENS_RDY_OUT_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 30 ) ) | ( ( ( value as u32 ) & 0x01 ) << 30 ) ; self . w } } # [ doc = "Reader of field `SENS_TSENS_OUT`" ] pub type SENS_TSENS_OUT_R = crate :: R < u8 , u8 > ; # [ doc = "Write proxy for field `SENS_TSENS_OUT`" ] pub struct SENS_TSENS_OUT_W < 'a > { w : & 'a mut W , } impl < 'a > SENS_TSENS_OUT_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0xff << 22 ) ) | ( ( ( value as u32 ) & 0xff ) << 22 ) ; self . w } } # [ doc = "Reader of field `SENS_I2C_SLAVE_ADDR4`" ] pub type SENS_I2C_SLAVE_ADDR4_R = crate :: R < u16 , u16 > ; # [ doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR4`" ] pub struct SENS_I2C_SLAVE_ADDR4_W < 'a > { w : & 'a mut W , } impl < 'a > SENS_I2C_SLAVE_ADDR4_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u16 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x07ff << 11 ) ) | ( ( ( value as u32 ) & 0x07ff ) << 11 ) ; self . w } } # [ doc = "Reader of field `SENS_I2C_SLAVE_ADDR5`" ] pub type SENS_I2C_SLAVE_ADDR5_R = crate :: R < u16 , u16 > ; # [ doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR5`" ] pub struct SENS_I2C_SLAVE_ADDR5_W < 'a > { w : & 'a mut W , } impl < 'a > SENS_I2C_SLAVE_ADDR5_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u16 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x07ff ) | ( ( value as u32 ) & 0x07ff ) ; self . w } } impl R { # [ doc = "Bit 30 - indicate temperature sensor out ready" ] # [ inline ( always ) ] pub fn sens_tsens_rdy_out ( & self ) -> SENS_TSENS_RDY_OUT_R { SENS_TSENS_RDY_OUT_R :: new ( ( ( self . bits >> 30 ) & 0x01 ) != 0 ) } # [ doc = "Bits 22:29 - temperature sensor data out" ] # [ inline ( always ) ] pub fn sens_tsens_out ( & self ) -> SENS_TSENS_OUT_R { SENS_TSENS_OUT_R :: new ( ( ( self . bits >> 22 ) & 0xff ) as u8 ) } # [ doc = "Bits 11:21" ] # [ inline ( always ) ] pub fn sens_i2c_slave_addr4 ( & self ) -> SENS_I2C_SLAVE_ADDR4_R { SENS_I2C_SLAVE_ADDR4_R :: new ( ( ( self . bits >> 11 ) & 0x07ff ) as u16 ) } # [ doc = "Bits 0:10" ] # [ inline ( always ) ] pub fn sens_i2c_slave_addr5 ( & self ) -> SENS_I2C_SLAVE_ADDR5_R { SENS_I2C_SLAVE_ADDR5_R :: new ( ( self . bits & 0x07ff ) as u16 ) } } impl W { # [ doc = "Bit 30 - indicate temperature sensor out ready" ] # [ inline ( always ) ] pub fn sens_tsens_rdy_out ( & mut self ) -> SENS_TSENS_RDY_OUT_W { SENS_TSENS_RDY_OUT_W { w : self } } # [ doc = "Bits 22:29 - temperature sensor data out" ] # [ inline ( always ) ] pub fn sens_tsens_out ( & mut self ) -> SENS_TSENS_OUT_W { SENS_TSENS_OUT_W { w : self } } # [ doc = "Bits 11:21" ] # [ inline ( always ) ] pub fn sens_i2c_slave_addr4 ( & mut self ) -> SENS_I2C_SLAVE_ADDR4_W { SENS_I2C_SLAVE_ADDR4_W { w : self } } # [ doc = "Bits 0:10" ] # [ inline ( always ) ] pub fn sens_i2c_slave_addr5 ( & mut self ) -> SENS_I2C_SLAVE_ADDR5_W { SENS_I2C_SLAVE_ADDR5_W { w : self } } }